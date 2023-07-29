
use std::collections::HashMap;
use std::sync::{RwLock, Arc, Mutex, TryLockResult, TryLockError};
use std::thread;

use crate::types::{Type, Value,TypeUtils, ValRef};

#[derive(Debug, Clone)]
pub struct Variable {
    the_type: Option<Type>,
    value: Option<Value>,
}

impl Variable {
    pub fn new(value: Value) -> Variable {
        Variable {
            the_type: Some(value.get_type()),
            value: Some(value),
        }
    }
    
    pub fn is_mutable(&self) -> bool {
        match self.value {
            Some(ref value) => value.is_mutable(),
            None => false,
        }
    }
}

impl Variable {
    pub fn assign_value(&mut self, r_value: Value) {
        if self.the_type.get_type() != r_value.get_type() {
            panic!("Tried to assign a value of the wrong type to a variable");
        }
        match self.value {
            Some(ref mut value) => {
                if value.is_mutable() {
                    value.set_value(r_value);
                } else {
                    panic!("Tried to assign a value to an immutable variable");
                }
            }
            None => {
                self.value = Some(r_value);
            }
        }
        
    }
    pub fn set_value(&mut self, r_value: Value) {
        if self.the_type.get_type() != r_value.get_type() {
            panic!("Tried to set a value of the wrong type to a variable");
        }
        match self.value {
            Some(ref mut value) => {
                value.set_value(r_value);
            }
            None => {
                self.value = Some(Value::new_ref(r_value));
            }
        }
    }


    pub fn get_immutable(&self) -> Value {
        if let Some(ref value) = self.value {
            if value.is_mutable() {
                value.get_immutable().clone()
            } else {
                value.clone()
            }
        } else {
            panic!("Tried to get the value of a variable that doesn't have a value");
        }
    }

    pub fn get_mutable(&self) -> Value {
        if let Some(ref value) = self.value {
            if value.is_mutable() {
                value.clone()
            } else {
                panic!("Tried to get the mutable value of an immutable variable");
            }
        } else {
            panic!("Tried to get the value of a variable that doesn't have a value");
        }
    }
    
    //TODO: Add function that allows us to bind generics to a variable
}

/// This represents a typeclass implementation.
/// It contains a Type to allow us to type check and a hashmap of functions.
pub struct TypeClass {
    types: Type,
    functions: HashMap<Type, Value>,//change i32 to a function ast
}

impl TypeClass {
    pub fn new(types: Type, functions: HashMap<Type, Value>) -> TypeClass {
        TypeClass {
            types,
            functions,
        }
    }
}

impl TypeClass {
    pub fn get_function(&mut self, name: &Type) -> Option<&Value> {
        self.functions.get(name)
    }
    

}

/// This represents the interpreter's data structure.
/// There are two symbol tables, one for named functions and one for typeclasses. The typeclass one has double indirection because we don't know the type of a function since there will be multiple implementations,
/// There is also a hashmap that allows us to lookup the typeclass for a type.
/// We then we have a hashmap that allows us to lookup the valid typeclasses for a type so we can't implement typeclasses that don't exist.
/// We then have a hashmap that allows us to lookup global variables. These are either immutable or mutable, But they are all local to the thread. Immutable Variables can't be reassigned.
/// We then have a hashmap that allows us to lookup shared global variables. These are all immutable and cannot be reassigned by any thread.
/// We then have a hashmap that allows us to lookup mutable global variables. These are all mutable and can be reassigned by any thread. They are however protected by a mutex.
#[derive(Debug, Clone)]
pub struct Interpreter {
    function_symbol_table: Arc<RwLock<HashMap<String, Value>>>,
    type_class_symbol_table: Arc<RwLock<HashMap<String, HashMap<Type, Value>>>>,
    default_symbol_table: Arc<RwLock<HashMap<String, Value>>>,
    valid_typeclasses: Arc<RwLock<HashMap<Type, Vec<Type>>>>,
    local_global_variables: HashMap<String, Variable>,
    shared_global_variables: Arc<RwLock<HashMap<String, Variable>>>,
    mutable_global_variables: Arc<RwLock<HashMap<String, Arc<Mutex<Variable>>>>>,
}


impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            function_symbol_table: Arc::new(RwLock::new(HashMap::new())),
            type_class_symbol_table: Arc::new(RwLock::new(HashMap::new())),
            default_symbol_table: Arc::new(RwLock::new(HashMap::new())),
            valid_typeclasses: Arc::new(RwLock::new(HashMap::new())),
            local_global_variables: HashMap::new(),
            shared_global_variables: Arc::new(RwLock::new(HashMap::new())),
            mutable_global_variables: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Interpreter {

    /// This function is how we add a new type class as well as their default implementation if there is one
    pub fn add_typeclass(&mut self, class: Type, functions: Vec<Result<Type,(String, Value)>>) {
        let mut table = self.default_symbol_table.write().expect("Interpretrer was not able to be written to");

        let mut func_table = Vec::new();
        for func in functions {
            match func {
                Ok(the_type) => func_table.push(the_type),
                Err((name, fun)) => {
                    func_table.push(fun.get_type());
                    table.insert(name, fun);
                }
            }
        }

        self.valid_typeclasses.write().expect("Interpreter was not able to be written to").insert(class, func_table);
    }

    pub fn add_typeclass_instance(&mut self, class: Type, functions: Vec<(String, Value)>) {
        if !self.valid_typeclasses.read().unwrap().contains_key(&class) {
            panic!("Tried to add a typeclass instance for a typeclass that doesn't exist");
        }

        let mut table = self.type_class_symbol_table.write().expect("Interpreter was not able to be written to");
        for (name, func) in functions {
            if let Some(ref mut v_table) = table.get_mut(&name) {
                v_table.insert(class.clone(), func);
            }

        }
    }
    
    pub fn new_for_thread(& self) -> Interpreter {
        Interpreter {
            function_symbol_table: self.function_symbol_table.clone(),
            type_class_symbol_table: self.type_class_symbol_table.clone(),
            default_symbol_table: self.default_symbol_table.clone(),
            valid_typeclasses: self.valid_typeclasses.clone(),
            local_global_variables: HashMap::new(),
            shared_global_variables: self.shared_global_variables.clone(),
            mutable_global_variables: self.mutable_global_variables.clone(),
        }
    }


    pub fn add_function(& mut self, name: &str, value: Value) {
        self.function_symbol_table.write().unwrap().insert(name.to_string(), value);
    }

    pub fn set_value(&mut self, name: &str, function_variables: &mut HashMap<String, Variable>, value: Value) {
        if self.mutable_global_variables.read().unwrap().contains_key(name) {
            unimplemented!("Need to implement mutable global variables");
            /*loop {
                match self.mutable_global_variables.try_write() {
                    TryLockResult::Err(TryLockError::WouldBlock) => {
                        continue;
                    }
                    TryLockResult::Err(TryLockError::Poisoned(_)) => {
                        panic!("Another thread panicked while holding the lock");
                    }
                    TryLockResult::Ok(mut guard) => {
                        guard.insert(name.to_string(), Arc::new(Mutex::new(Variable::new(ValuePtr::new_mut(value)))));
                        break;
                    }
                }
            }*/
        }
        else if self.shared_global_variables.read().unwrap().contains_key(name) {
            panic!("Tried to assign to a shared global variable");
        }
        else if let Some(variable) = self.local_global_variables.get_mut(name) {
            //change it so that we check the value of the variable and see if it is mutable or not
            if variable.is_mutable() {
                variable.set_value(value);
            }
            else {
                panic!("Tried to assign to an immutable variable");
            }
        }
        else if let Some(variable) = function_variables.get_mut(name) {
            variable.set_value(value);
        }
        else {
            panic!("Tried to assign to a variable that doesn't exist");
        }
        
    }

    pub fn get_value(&self, name: &str, function_variables: &HashMap<String, Value>) -> Option<Value> {
        if let Some(variable) = function_variables.get(name) {
            return Some(variable.clone());
        }
        if let Some(variable) = self.local_global_variables.get(name) {
            return Some(variable.get_immutable());
        }
        if let Some(variable) = self.shared_global_variables.read().unwrap().get(name) {
            return Some(variable.get_immutable());
        }
        if let Some(variable) = self.mutable_global_variables.read().unwrap().get(name) {
            let value;
            loop {
                match variable.try_lock() {
                    TryLockResult::Err(TryLockError::WouldBlock) => {
                        continue;
                    }
                    TryLockResult::Err(TryLockError::Poisoned(_)) => {
                        panic!("Another thread panicked while holding the lock");
                    }
                    TryLockResult::Ok(guard) => {
                        value = guard.clone();
                        break;
                    }
                }
            }
            return Some(value.get_immutable());
        }
        None
        
    }

    fn function_caller(& mut self, function_name: &str, function: Value, arguments: Vec<Value>) -> Value {
        match function {
            Value::Function(threaded, args, effects, ret_type, variable_map, body) => {
                let mut variable_map = variable_map;
                let mut pass_by_ref = false;
                for ((name, the_type),mut arg) in args.iter().zip(arguments.into_iter()) {
                    //TODO: add in ablity to bind generics to types
                    if the_type.get_type() != arg.get_type() {
                        panic!("Tried to call function {} with argument of type {} when it expected type {}", function_name, arg.get_type(), the_type.get_type());
                    }
                    if the_type.is_ref() {
                        pass_by_ref = true;
                        variable_map.insert(name.to_string(), arg.clone());
                    }
                    else {
                        variable_map.insert(name.to_string(), arg.get_immutable());
                    }
                }
                if threaded {
                    if pass_by_ref {
                        panic!("Tried to call a threaded function with a reference");
                    }
                    let mut interpreter = self.clone();


                    let handle = Arc::new(RwLock::new(thread::spawn(move || {
                        return interpreter.evaluate_block(&mut variable_map, &body);
                    })));

                    return Value::create_promise(handle, ret_type.clone());

                }

                return self.evaluate_block(&mut variable_map, &body);
                

            },
            _ => {
                panic!("Tried to call something that isn't a function");
            }
        }
    }

    pub fn call_function(&mut self, name: &str, arguments: Vec<Value>, local_variables: HashMap<String, Value>) -> Value {

        let function = if let Some(function) = self.function_symbol_table.read().expect("Unable to read interpreter").get(name) {
            function.clone()
        }
        else if let Some(function) = self.type_class_symbol_table.read().expect("Unable to read interpreter").get(name).unwrap_or(&HashMap::new()).get(&arguments[0].get_type()) {
            function.clone()

        }
        else if let Some(function) = self.default_symbol_table.read().expect("Unable to read interpreter").get(name) {
            function.clone()
        }
        else if let Some(function) = self.check_if_function(name, &local_variables) {
            function
        }
        else {
            panic!("Either tried to call a function that doesn't exist or tried to call something that isn't a function: {}", name);
        };
        return self.function_caller(name, function, arguments);

    }

    fn check_if_function(&self, name: &str, local_variables: &HashMap<String, Value>) -> Option<Value> {
        if let Some(function) = local_variables.get(name) {
            return Some(function.get_immutable().clone());
        }
        else {
            return None;
        }
    }

    fn evaluate_block(&mut self, function_variables: &mut HashMap<String, Value>, block: &String) -> Value {
        unimplemented!("Interpretation of functions is not yet implemented");
    }

    
}

