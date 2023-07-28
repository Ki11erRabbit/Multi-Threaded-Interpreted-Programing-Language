
use std::collections::HashMap;
use std::sync::{RwLock, Arc, Mutex, TryLockResult, TryLockError};

use crate::types::{ValuePtr, Type, Value};



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
/// There are two symbol tables, one for named functions and one for typeclasses.
/// There is also a hashmap that allows us to lookup the typeclass for a type.
/// We then we have a hashmap that allows us to lookup the valid typeclasses for a type so we can't implement typeclasses that don't exist.
/// We then have a hashmap that allows us to lookup global variables. These are either immutable or mutable, But they are all local to the thread.
/// We then have a hashmap that allows us to lookup shared global variables. These are all immutable and cannot be reassigned by any thread.
/// We then have a hashmap that allows us to lookup mutable global variables. These are all mutable and can be reassigned by any thread. They are however protected by a mutex.
pub struct Interpreter<'a> {
    function_symbol_table: Arc<RwLock<HashMap<String, Value<'a>>>>,
    typeclass_symbol_table: Arc<RwLock<HashMap<Type, TypeClass<'a>>>>,
    symbol_table_to_typeclass: Arc<RwLock<HashMap<String, Type>>>,
    valid_typeclasses: Arc<RwLock<HashMap<Type, Vec<Type>>>>,
    local_global_variables: HashMap<String, ValuePtr<'a>>,
    shared_global_variables: Arc<RwLock<HashMap<String, ValuePtr<'a>>>>,
    mutable_global_variables: Arc<RwLock<HashMap<String, Arc<Mutex<ValuePtr<'a>>>>>>,
}


impl Interpreter<'_> {
    pub fn new() -> Interpreter<'static> {
        Interpreter {
            function_symbol_table: Arc::new(RwLock::new(HashMap::new())),
            typeclass_symbol_table: Arc::new(RwLock::new(HashMap::new())),
            symbol_table_to_typeclass: Arc::new(RwLock::new(HashMap::new())),
            valid_typeclasses: Arc::new(RwLock::new(HashMap::new())),
            local_global_variables: HashMap::new(),
            shared_global_variables: Arc::new(RwLock::new(HashMap::new())),
            mutable_global_variables: Arc::new(RwLock::new(HashMap::new())),
        }
    }


}

impl<'a> Interpreter<'a> {
    
    pub fn new_for_thread(& self) -> Interpreter<'a> {
        Interpreter {
            function_symbol_table: self.function_symbol_table.clone(),
            typeclass_symbol_table: self.typeclass_symbol_table.clone(),
            symbol_table_to_typeclass: self.symbol_table_to_typeclass.clone(),
            valid_typeclasses: self.valid_typeclasses.clone(),
            local_global_variables: HashMap::new(),
            shared_global_variables: self.shared_global_variables.clone(),
            mutable_global_variables: self.mutable_global_variables.clone(),
        }
    }

    pub fn lookup_variable(&self, name: &str, function_variables: &HashMap<String, ValuePtr<'a>>) -> Option<ValuePtr<'a>> {
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

