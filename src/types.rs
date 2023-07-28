


use std::collections::HashMap;
use std::sync::{Arc,Mutex};
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::PartialEq;
use std::thread::JoinHandle;

#[derive(Debug, Clone,Eq, Hash)]
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
    Ref(Box<Type>),
    Unit,
}

impl Type {
    pub fn is_reference(&self) -> bool {
        match self {
            Type::Ref(_) => true,
            _ => false,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Single(a), Type::Single(b)) => {
                if a == "Any" || b == "Any" {
                    true
                } else {
                    if a.len() == 1 || b.len() == 1 {
                        true
                    } else {
                        a == b
                    }
                }
            },
            (Type::Tuple(a), Type::Tuple(b)) => a == b,
            (Type::Function{parameters: a, effects: b, return_type: c}, Type::Function{parameters: d, effects: e, return_type: f}) => a == d && b == e && c == f,
            (Type::TypeList{name: a, parameters: b}, Type::TypeList{name: c, parameters: d}) => a == c && b == d,
            (Type::Unit, Type::Unit) => true,
            (Type::Ref(a), Type::Ref(b)) => *a == *b,
            (Type::Ref(a), b) => **a == *b,
            (a, Type::Ref(b)) => *a == **b,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
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
            Type::Ref(t) => write!(f, "&{}", t),
            
        }
    }

}



pub trait TypeUtils {
    fn get_type(&self) -> Type;

    fn is_ref(&self) -> bool;
}

impl TypeUtils for Type {
    fn get_type(&self) -> Type {
        self.clone()
    }

    fn is_ref(&self) -> bool {
        self.is_reference()
    }
}

impl TypeUtils for &Type {
    fn get_type(&self) -> Type {
        (*self).clone()
    }

    fn is_ref(&self) -> bool {
        self.is_reference()
    }
}

impl TypeUtils for Option<Type> {
    fn get_type(&self) -> Type {
        match self {
            Some(v) => v.get_type(),
            None => Type::Single("Any".to_string()),
        }
    }

    fn is_ref(&self) -> bool {
        match self {
            Some(v) => v.is_reference(),
            None => false,
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

    fn is_ref(&self) -> bool {
        match self {
            Some(v) => v.is_reference(),
            None => false,
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

    fn is_ref(&self) -> bool {
        match self {
            Some(v) => v.get_type().is_reference(),
            None => false,
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

    fn is_ref(&self) -> bool {
        match self {
            Some(v) => v.get_type().is_reference(),
            None => false,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlgebraicType {
    Sum,
    Product,
}

#[derive(Debug)]
pub enum Value<'a> {
    Int(i64),
    UInt(u64),
    Float(f64),
    Char(char),
    Byte(u8),
    List(Vec<Value<'a>>, Type),
    Vector(&'a [Value<'a>], Type),
    Tuple(Vec<Value<'a>>),
    Function(bool,//Will We Spawn a new thread
        Vec<(String, Option<Type>)>,//Mapping of variable to type
             Vec<Type>,//TODO: add in effects
             Type,//Return type
             HashMap<String, Value<'a>>,//Mapping of variable to value. This allows us to have higher order functions
             String,//Function body
    ),// TODO: add in function body
    Promise(JoinHandle<Value<'a>>, Type,),//Return Value from a multi-threaded function
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
    Ref(&'a mut Value<'a>),
}

impl Clone for Value<'_> {
    fn clone(&self) -> Self {
        match self {
            Value::Int(i) => Value::Int(*i),
            Value::UInt(i) => Value::UInt(*i),
            Value::Float(i) => Value::Float(*i),
            Value::Char(i) => Value::Char(*i),
            Value::Byte(i) => Value::Byte(*i),
            Value::List(i, t) => Value::List(i.clone(), t.clone()),
            Value::Vector(i, t) => Value::Vector(i.clone(), t.clone()),
            Value::Tuple(i) => Value::Tuple(i.clone()),
            Value::Function(a, b, c, d, e, f) => Value::Function(*a, b.clone(), c.clone(), d.clone(), e.clone(), f.clone()),
            Value::Promise(_, _) => panic!("Cannot clone a promise"),
            Value::Algebraic{agb_type, types, name, values} => Value::Algebraic{agb_type: agb_type.clone(), types: types.clone(), name: name.clone(), values: values.clone()},
            Value::Alias{parent, name, value} => Value::Alias{parent: parent.clone(), name: name.clone(), value: value.clone()},
            Value::Ref(_) => panic!("Cannot clone a reference"),
        }
    }
}

impl <'a>Value<'a> {
    pub fn create_promise(handle: JoinHandle<Value<'a>>, the_type: Type) -> Value<'a> {
        Value::Promise(handle, the_type)
    }

    pub fn create_reference(&'a mut self) -> Value<'a> {
        Value::Ref(self)
    }
}


impl <'a>TypeUtils for Value<'a> {
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
            Value::Function(_,parameters, effects, return_type, _, _) => Type::Function{parameters: parameters.iter().map(|(_, t)| t.get_type()).collect(), effects: effects.clone(), return_type: Box::new(return_type.get_type())},
            Value::Promise(_, t) => Type::TypeList{name: Box::new(Type::Single("Promise".to_string())), parameters: vec![t.get_type()]},
            Value::Algebraic{agb_type, types, name, values} => Type::TypeList{ name: Box::new(Type::Single(name.clone())), parameters: types.iter().map(|t| t.get_type()).collect()},
            Value::Alias{parent, name, value} => name.get_type(),
            Value::Ref(i) => Type::Ref(Box::new(i.get_type()))
        }

    }

    fn is_ref(&self) -> bool {
        match self {
            Value::Ref(_) => true,
            _ => false,
        }
    }

}

impl <'a>TypeUtils for &Value<'a> {
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
            Value::Function(_,parameters, effects, return_type, _, _) => Type::Function{parameters: parameters.iter().map(|(_, t)| t.get_type()).collect(), effects: effects.clone(), return_type: Box::new(return_type.get_type())},
            Value::Promise(_, t) => Type::TypeList{name: Box::new(Type::Single("Promise".to_string())), parameters: vec![t.get_type()]},
            Value::Algebraic{agb_type, types, name, values} => Type::TypeList{ name: Box::new(Type::Single(name.clone())), parameters: types.iter().map(|t| t.get_type()).collect()},
            Value::Alias{parent, name, value} => name.get_type(),
            Value::Ref(i) => Type::Ref(Box::new(i.get_type()))
        }

    }

    fn is_ref(&self) -> bool {
        match self {
            Value::Ref(_) => true,
            _ => false,
        }
    }

}


pub type ValueImmu<'a> = Rc<Value<'a>>;
pub type ValueMut<'a> = Rc<RefCell<Value<'a>>>;

#[derive(Debug, Clone)]
pub enum ValuePtr<'a> {
    Immu(Value<'a>),
    Mut(Value<'a>),
}

impl TypeUtils for ValuePtr<'_> {
    fn get_type(&self) -> Type {
        match self {
            ValuePtr::Immu(v) => v.get_type(),
            ValuePtr::Mut(v) => v.get_type(),
        }
    }

    fn is_ref(&self) -> bool {
        match self {
            ValuePtr::Immu(v) => panic!("Immutable value cannot be a reference"),
            ValuePtr::Mut(v) => v.is_ref(),
        }
    }
}
impl TypeUtils for &ValuePtr<'_> {
    fn get_type(&self) -> Type {
        match self {
            ValuePtr::Immu(v) => v.get_type(),
            ValuePtr::Mut(v) => v.get_type(),
        }
    }

    fn is_ref(&self) -> bool {
        match self {
            ValuePtr::Immu(v) => panic!("Immutable value cannot be a reference"),
            ValuePtr::Mut(v) => v.is_ref(),
        }
    }
}

impl <'a>ValuePtr<'a> {
    pub fn new_immu(value: Value<'a>) -> Self {
        ValuePtr::Immu(value)
    }
    pub fn new_mut(value: Value<'a>) -> Self {
        ValuePtr::Mut(value)
    }
}

impl<'a> ValueRef for ValuePtr<'a> {
    fn get_value(&self) -> Value {
        match self {
            ValuePtr::Immu(v) => v.clone(),
            ValuePtr::Mut(v) => v.clone(),
        }
    }

    fn get_value_mut(&'a mut self) -> Value<'a> {
        match self {
            ValuePtr::Immu(v) => panic!("Cannot get mutable reference to immutable value"),
            ValuePtr::Mut(v) => v.create_reference(),
        }
    }
}


pub trait ValueRef {
    fn get_value(&self) -> Value;

    fn get_value_mut(&self) -> Value;
}

