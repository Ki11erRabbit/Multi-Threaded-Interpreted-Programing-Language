use chumsky::prelude::*;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    class: Option<String>,
    value: String,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.class {
            Some(class) => write!(f, "{} {}", class, self.value),
            None => write!(f, "{}", self.value),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlgebraicType {
    Sum,
    Product,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'a> {
    Int(i64),
    UInt(u64),
    Float(f64),
    Char(char),
    Byte(u8),
    List(Box<Value>),
    HList(Vec<Value>),
    Vector(&'a [Value]),
    HVector(Box<Value>),
    Tuple(Vec<Value>),
    Function(Vec<(String, Type)>,//Mapping of variable to type
             Vec<String>,//TODO: add in effects
             Type,//Return type
             HashMap<String, Value>,//Mapping of variable to value. This allows us to have higher order functions
             String,//Function body
    ),// TODO: add in function body
    Promise(Mutex<Value>, String),//Return Value from a multi-threaded function
    Algebraic {
        agb_type: AlgebraicType,
        types: Vec<Type>,
        name: String,
        values: HashMap<String, Value>,
    },
    Alias {
        parent: String,
        name: String,
        value: Box<Value>,
    },
}

impl Value {

    pub fn get_type(&self) -> String {
        match self {
            Int(_) => "Int".to_string(),
            UInt(_) => "UInt".to_string(),
            Float(_) => "Float".to_string(),
            Char(_) => "Char".to_string(),
            Byte(_) => "Byte".to_string(),
            List(_) => "List".to_string(),
            HList(_) => "H-List".to_string(),
            Vector(_) => "Vector".to_string(),
            HVector(_) => "H-Vector".to_string(),
            Tuple(values) => {
                if values.len() == 0 {
                    "()".to_string()
                } else { // exmaple: (Int, Int, Int)
                    let mut result = "(".to_string();
                    for value in values {
                        result.push_str(&value.get_type());
                        result.push_str(", ");
                    }
                    result.pop();
                    result.pop();
                    result.push(')');
                    result
                }
            },
            Promise(_, tag) => {
                format!("Promise {}", tag)
            },
            Algebraic{agb_type, types, name, values} => {
                let mut result = format!("{} ", name);
                if types.len() == 0 {
                } else {
                    for t in types {
                        result.push_str(&t.to_string());
                        result.push_str(" ");
                    }
                    result.pop();
                }
                result

            },
            Alias{parent, name, value} => {
                name
            },
            Function(types, return_type, effects,_ , _) => {
                let mut result = "fn(".to_string();
                for (name, t) in types {
                    result.push_str(&t.to_string());
                    result.push_str(", ");
                }
                result.pop();
                result.pop();
                result.push(')');

                for effect in effects {
                    result.push_str(&effect);
                    result.push_str(" ");
                }
                result.push_str("-> ");
                result.push_str(&return_type.to_string());

                result
            },
        }
    }
            
            

}


