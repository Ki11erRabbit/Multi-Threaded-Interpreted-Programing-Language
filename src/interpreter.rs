
use std::collections::HashMap;
use std::sync::{RwLock, Arc, Mutex, TryLockResult, TryLockError};

use crate::types::{ValuePtr, Type, Value};


pub struct Variable<'a> {
    the_type: Option<Type>,
    value: ValuePtr<'a>,
}

impl Variable<'_> {
    pub fn is_mutable(&self) -> bool {
        match self.value {
            ValuePtr::Mut(_) => true,
            ValuePtr::Immu() => false,
        }
    }
}

impl <'a>Variable<'a> {
    pub fn set_value(&mut self, value: Value<'a>) {
        if self.the_type.get_type() != value.get_type() {
            panic!("Tried to assign a value of the wrong type to a variable");
        }

        match self.value {
            ValuePtr::Mut(ref mut value_ptr) => {
                *value_ptr = value;
            }
            ValuePtr::Immu() => {
                self.value = ValuePtr::new_immu(value);
            }
        }
    }
    //TODO: Add function that allows us to bind generics to a variable
}

/// This represents a typeclass implementation.
/// It contains a Type to allow us to type check and a hashmap of functions.
pub struct TypeClass<'a> {
    types: Type,
    functions: HashMap<Type, Value<'a>>,//change i32 to a function ast
}

impl TypeClass<'_> {
    pub fn new(types: Type, functions: HashMap<Type, Value<'static>>) -> TypeClass<'static> {
        TypeClass {
            types,
            functions,
        }
    }
}

impl <'a>TypeClass<'a> {
    pub fn get_function(&mut self, name: &Type) -> Option<&Value<'a>> {
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
pub struct Interpreter<'a> {
    function_symbol_table: Arc<RwLock<HashMap<String, Value<'a>>>>,
    type_class_symbol_table: Arc<RwLock<HashMap<String, HashMap<Type, Value<'a>>>>>,
    default_symbol_table: Arc<RwLock<HashMap<String, Value<'a>>>>,
    valid_typeclasses: Arc<RwLock<HashMap<Type, Vec<Type>>>>,
    local_global_variables: HashMap<String, Variable<'a>>,
    shared_global_variables: Arc<RwLock<HashMap<String, Variable<'a>>>>,
    mutable_global_variables: Arc<RwLock<HashMap<String, Arc<Mutex<Variable<'a>>>>>>,
}


impl Interpreter<'_> {
    pub fn new() -> Interpreter<'static> {
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

impl<'a> Interpreter<'a> {

    /// This function is how we add a new type class as well as their default implementation if there is one
    pub fn add_typeclass(&mut self, class: Type, functions: Vec<Result<Type,(String, Value<'a>)>>) {
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

    pub fn add_typeclass_instance(&mut self, class: Type, functions: Vec<(String, Value<'a>)>) {
        if !self.valid_typeclasses.read().unwrap().contains_key(&class) {
            panic!("Tried to add a typeclass instance for a typeclass that doesn't exist");
        }

        let mut table = self.type_class_symbol_table.write().expect("Interpreter was not able to be written to");
        for (name, func) in functions {
            if let Some(ref mut v_table) = table.get_mut(&name) {
                v_table.insert(class, func);
            }

        }
    }
    
    pub fn new_for_thread(& self) -> Interpreter<'a> {
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


    pub fn add_function(&'a mut self, name: &str, value: Value<'a>) {
        self.function_symbol_table.write().unwrap().insert(name.to_string(), value);
    }

    pub fn set_value(&mut self, name: &str, function_variables: &mut HashMap<String, Variable<'a>>, value: Value<'a>) {
        if self.mutable_global_variables.read().unwrap().contains_key(name) {
            loop {
                match self.mutable_global_variables.try_write() {
                    TryLockResult::Err(TryLockError::WouldBlock) => {
                        continue;
                    }
                    TryLockResult::Err(TryLockError::Poisoned(_)) => {
                        panic!("Another thread panicked while holding the lock");
                    }
                    TryLockResult::Ok(mut guard) => {
                        guard.insert(name.to_string(), Arc::new(Mutex::new(ValuePtr::new_mut(value))));
                        break;
                    }
                }
            }
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

    pub fn get_value(&self, name: &str, function_variables: &HashMap<String, ValuePtr<'a>>) -> Option<ValuePtr<'a>> {
        if let Some(variable) = function_variables.get(name) {
            return Some(variable.clone());
        }
        if let Some(variable) = self.local_global_variables.get(name) {
            return Some(variable.clone());
        }
        if let Some(variable) = self.shared_global_variables.read().unwrap().get(name) {
            return Some(variable.clone());
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
            return Some(value);
        }
        None
        
    }

    
}

