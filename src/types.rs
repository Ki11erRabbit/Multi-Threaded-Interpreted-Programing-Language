


use std::collections::HashMap;
use std::sync::Mutex;
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    TypeList {
        name: Box<Type>,
        parameters: Vec<Type>,
    },
    Function {
        parameters: Vec<Type>,
        effects: Vec<Type>,
        return_type: Box<Type>,
    },
    Single(String),
    Tuple(Vec<Type>),
    Unit,
}


impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Single(name) => write!(f, "{}", name),
            Type::Tuple(types) => {
                let output = String::new();
                let mut output = output + "(";
                for t in types {
                    output += &format!("{}, ", t);
                }
                output.pop();
                output.pop();
                output += ")";
                write!(f, "{}", output)
            },
            Type::Function{parameters, effects, return_type} => {
                let mut output = "fn(".to_string();

                for p in parameters {
                    output += &format!("{}, ", p);
                }
                output.pop();
                output.pop();
                output += ")";
                for e in effects {
                    output += &format!("{} ", e);
                }
                output += "-> ";
                output += &format!("{}", return_type);
                write!(f, "{}", output)
            },
            Type::TypeList{name, parameters} => {
                let mut output = format!("({}", name);
                for p in parameters {
                    output += &format!(" {}", p);
                }
                output += ")";
                write!(f, "{}", output)
            },
            Type::Unit => write!(f, "()"),
            
        }
    }

}



pub trait TypeUtils {
    fn get_type(&self) -> Type;
}

impl TypeUtils for Type {
    fn get_type(&self) -> Type {
        self.clone()
    }
}

impl TypeUtils for &Type {
    fn get_type(&self) -> Type {
        (*self).clone()
    }
}

impl TypeUtils for Option<Type> {
    fn get_type(&self) -> Type {
        match self {
            Some(v) => v.get_type(),
            None => Type::Single("Any".to_string()),
        }
    }
}

impl TypeUtils for &Option<Type> {
    fn get_type(&self) -> Type {
        match self {
            Some(v) => v.get_type(),
            None => Type::Single("Any".to_string()),
        }
    }
}

impl TypeUtils for Option<Value<'_>> {
    fn get_type(&self) -> Type {
        match self {
            Some(v) => v.get_type(),
            None => Type::Single("Any".to_string()),
        }
    }
}

impl TypeUtils for &Option<Value<'_>> {
    fn get_type(&self) -> Type {
        match self {
            Some(v) => v.get_type(),
            None => Type::Single("Any".to_string()),
        }
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlgebraicType {
    Sum,
    Product,
}

#[derive(Debug,Clone)]
pub enum Value<'a> {
    Int(i64),
    UInt(u64),
    Float(f64),
    Char(char),
    Byte(u8),
    List(Vec<Value<'a>>, Type),
    Vector(&'a [Value<'a>], Type),
    Tuple(Vec<Value<'a>>),
    Function(Vec<(String, Option<Type>)>,//Mapping of variable to type
             Vec<Type>,//TODO: add in effects
             Type,//Return type
             HashMap<String, Value<'a>>,//Mapping of variable to value. This allows us to have higher order functions
             String,//Function body
    ),// TODO: add in function body
    Promise(&'a Mutex<Box<Value<'a>>>, Type),//Return Value from a multi-threaded function
    Algebraic {
        agb_type: AlgebraicType,
        types: Vec<Type>,
        name: String,
        values: HashMap<Type, Value<'a>>,
    },
    Alias {
        parent: Type,
        name: Type,
        value: Box<Value<'a>>,
    },
}


impl TypeUtils for Value<'_> {
    fn get_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Single("Int".to_string()),
            Value::UInt(_) => Type::Single("UInt".to_string()),
            Value::Float(_) => Type::Single("Float".to_string()),
            Value::Char(_) => Type::Single("Char".to_string()),
            Value::Byte(_) => Type::Single("Byte".to_string()),
            Value::List(_, t) => Type::TypeList{name: Box::new(Type::Single("List".to_string())), parameters: vec![t.get_type()]},
            Value::Vector(_, t) => Type::TypeList{name: Box::new(Type::Single("Vector".to_string())), parameters: vec![t.get_type()]},
            Value::Tuple(values) => Type::Tuple(values.iter().map(|v| v.get_type()).collect()),
            Value::Function(parameters, effects, return_type, _, _) => Type::Function{parameters: parameters.iter().map(|(_, t)| t.get_type()).collect(), effects: effects.clone(), return_type: Box::new(return_type.get_type())},
            Value::Promise(_, t) => Type::TypeList{name: Box::new(Type::Single("Promise".to_string())), parameters: vec![t.get_type()]},
            Value::Algebraic{agb_type, types, name, values} => Type::TypeList{ name: Box::new(Type::Single(name.clone())), parameters: types.iter().map(|t| t.get_type()).collect()},
            Value::Alias{parent, name, value} => name.get_type(),
        }

    }

}

impl TypeUtils for &Value<'_> {
    fn get_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Single("Int".to_string()),
            Value::UInt(_) => Type::Single("UInt".to_string()),
            Value::Float(_) => Type::Single("Float".to_string()),
            Value::Char(_) => Type::Single("Char".to_string()),
            Value::Byte(_) => Type::Single("Byte".to_string()),
            Value::List(_, t) => Type::TypeList{name: Box::new(Type::Single("List".to_string())), parameters: vec![t.get_type()]},
            Value::Vector(_, t) => Type::TypeList{name: Box::new(Type::Single("Vector".to_string())), parameters: vec![t.get_type()]},
            Value::Tuple(values) => Type::Tuple(values.iter().map(|v| v.get_type()).collect()),
            Value::Function(parameters, effects, return_type, _, _) => Type::Function{parameters: parameters.iter().map(|(_, t)| t.get_type()).collect(), effects: effects.clone(), return_type: Box::new(return_type.get_type())},
            Value::Promise(_, t) => Type::TypeList{name: Box::new(Type::Single("Promise".to_string())), parameters: vec![t.get_type()]},
            Value::Algebraic{agb_type, types, name, values} => Type::TypeList{ name: Box::new(Type::Single(name.clone())), parameters: types.iter().map(|t| t.get_type()).collect()},
            Value::Alias{parent, name, value} => name.get_type(),
        }

    }

}


pub type ValueImmu<'a> = Rc<Value<'a>>;
pub type ValueMut<'a> = Rc<RefCell<Value<'a>>>;

pub trait ValueRef {
    fn get_type(&self) -> Type;

    fn get_value(&self) -> &Value;

    fn get_value_mut(&self) -> &mut Value;

}


