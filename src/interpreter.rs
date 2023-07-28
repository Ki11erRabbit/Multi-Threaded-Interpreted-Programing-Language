
use std::collections::HashMap;
use std::sync::{RwLock, Arc, Mutex};

use crate::types::{ValueImmu, ValueMut, Type, Value};


pub enum Variable<'a> {
    Immutable(ValueImmu<'a>),
    Mutable(ValueMut<'a>),
}

pub struct TypeClass<'a> {
    types: Type,
    functions: HashMap<String, Value<'a>>,//change i32 to a function ast
}


pub struct Interpreter<'a> {
    function_symbol_table: Arc<RwLock<HashMap<String, Value<'a>>>>,
    typeclass_symbol_table: Arc<RwLock<HashMap<String, TypeClass<'a>>>>,
    symbol_table_to_typeclass: Arc<RwLock<HashMap<String, String>>>,
    local_global_variables: HashMap<String, Variable<'a>>,
    shared_global_variables: Arc<RwLock<HashMap<String, Variable<'a>>>>,
    mutable_global_variables: Arc<RwLock<HashMap<String, Arc<Mutex<Variable<'a>>>>>>,
}


impl Interpreter<'_> {
    pub fn new() -> Interpreter<'static> {
        Interpreter {
            function_symbol_table: Arc::new(RwLock::new(HashMap::new())),
            typeclass_symbol_table: Arc::new(RwLock::new(HashMap::new())),
            symbol_table_to_typeclass: Arc::new(RwLock::new(HashMap::new())),
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
            local_global_variables: HashMap::new(),
            shared_global_variables: self.shared_global_variables.clone(),
            mutable_global_variables: self.mutable_global_variables.clone(),
        }
    }
}

