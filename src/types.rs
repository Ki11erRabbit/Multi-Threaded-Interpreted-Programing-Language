


use std::collections::HashMap;
use std::sync::{Arc,RwLock};
use std::fmt;
use std::cell::{RefCell, RefMut, Ref};
use std::rc::Rc;
use std::cmp::PartialEq;
use std::thread::JoinHandle;

use core::marker::{Sync,Send};



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
    Alias(Box<Type>, Box<Type>),
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
            (Type::Ref(a), Type::Ref(b)) => a == b,
            (Type::Ref(a), b) => **a == *b,
            (a, Type::Ref(b)) => *a == **b,
            (Type::Alias(a, b), Type::Alias(c, d)) => a == c && b == d,
            (Type::Alias(a, b), c) => **a == *c,
            (a, Type::Alias(b, c)) => *a == **b,
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
            Type::Alias(_, b) => write!(f, "{}", b),
            
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
        match self {
            Type::Ref(_) => true,
            _ => false,
        }
    }
}

impl TypeUtils for &Type {
    fn get_type(&self) -> Type {
        (*self).clone()
    }

    fn is_ref(&self) -> bool {
        match self {
            Type::Ref(_) => true,
            _ => false,
        }
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
            Some(v) => v.is_ref(),
            None => false,
        }
    }
}

impl TypeUtils for &Option<Type> {
    fn get_type(&self) -> Type {
        self.get_type()
    }

    fn is_ref(&self) -> bool {
        self.is_ref()
    }
}

impl TypeUtils for Option<Value> {
    fn get_type(&self) -> Type {
        match self {
            Some(v) => v.get_type(),
            None => Type::Single("Any".to_string()),
        }
    }

    fn is_ref(&self) -> bool {
        match self {
            Some(v) => v.is_ref(),
            None => false,
        }
    }
}

impl TypeUtils for &Option<Value> {
    fn get_type(&self) -> Type {
        self.get_type()
    }

    fn is_ref(&self) -> bool {
        self.is_ref()
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlgebraicType {
    Sum,
    Product,
}

#[derive(Debug)]
pub enum Value {
    Int(i64),
    UInt(u64),
    Float(f64),
    Char(char),
    Byte(u8),
    List(Vec<Value>, Type),
    //Vector(Rc<RefCell<[Value]>>, Type),
    Tuple(Vec<Value>),
    Function(bool,//Will We Spawn a new thread
        Vec<(String, Option<Type>)>,//Mapping of variable to type
             Vec<Type>,//TODO: add in effects
             Type,//Return type
             HashMap<String, Value>,//Mapping of variable to value. This allows us to have higher order functions
             String,//Function body
    ),// TODO: add in function body
    Promise(Arc<RwLock<JoinHandle<Value>>>, Type,),//Return Value from a multi-threaded function
    Algebraic {
        agb_type: AlgebraicType,
        types: Vec<Type>,
        name: String,
        values: HashMap<Type, Value>,
    },
    Alias {
        parent: Type,
        name: Type,
        value: Box<Value>,
    },
    Ref(ValRef),
}

impl Value {
    pub fn is_mutable(&self) -> bool {
        match self {
            Value::Ref(_) => true,
            _ => false,
        }
    }

    pub fn set_value(&mut self, value: Value) {
        match self {
            Value::Ref(r) => {
                *r.value.borrow_mut() = value;
            },
            _ => {},
        }
    }

    pub fn new_ref(self) -> Self {
        Value::Ref(ValRef::new(self))
    }

    pub fn get_immutable(&self) -> Self {
        match self {
            Value::Ref(r) => {
                r.borrow().clone()
            },
            _ => self.clone(),
        }
    }

    pub fn get_mutable(&self) -> Self {
        match self {
            Value::Ref(r) => {
                self.clone()
            },
            _ => panic!("Cannot get mutable reference to immutable value"),
        }
    }
}

unsafe impl Sync for ValRef {}
unsafe impl Send for ValRef {}

#[derive(Debug,Clone)]
pub struct ValRef {
    pub value: Rc<RefCell<Value>>,
}

impl ValRef {
    pub fn new(value: Value) -> Self {
        ValRef {
            value: Rc::new(RefCell::new(value)),
        }
    }

    pub fn borrow(&self) -> Ref<Value> {
        self.value.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<Value> {
        self.value.borrow_mut()
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Int(i) => Value::Int(*i),
            Value::UInt(i) => Value::UInt(*i),
            Value::Float(i) => Value::Float(*i),
            Value::Char(i) => Value::Char(*i),
            Value::Byte(i) => Value::Byte(*i),
            Value::List(i, t) => Value::List(i.clone(), t.clone()),
            //Value::Vector(i, t) => Value::Vector(i.clone(), t.clone()),
            Value::Tuple(i) => Value::Tuple(i.clone()),
            Value::Function(a, b, c, d, e, f) => Value::Function(*a, b.clone(), c.clone(), d.clone(), e.clone(), f.clone()),
            Value::Promise(_, _) => panic!("Cannot clone a promise"),
            Value::Algebraic{agb_type, types, name, values} => Value::Algebraic{agb_type: agb_type.clone(), types: types.clone(), name: name.clone(), values: values.clone()},
            Value::Alias{parent, name, value} => Value::Alias{parent: parent.clone(), name: name.clone(), value: value.clone()},
            Value::Ref(_) => panic!("Cannot clone a reference"),
        }
   }
}

impl Value {
    pub fn create_promise(handle: Arc<RwLock<JoinHandle<Value>>>, the_type: Type) -> Value {
        Value::Promise(handle, the_type)
    }

    pub fn create_reference(&self) -> Value {
        match self {
            Value::Ref(r) => Value::Ref(r.clone()),
            _ => panic!("Cannot create a reference to a non-reference value"),
        }
    }
}


impl TypeUtils for Value {
    fn get_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Single("Int".to_string()),
            Value::UInt(_) => Type::Single("UInt".to_string()),
            Value::Float(_) => Type::Single("Float".to_string()),
            Value::Char(_) => Type::Single("Char".to_string()),
            Value::Byte(_) => Type::Single("Byte".to_string()),
            Value::List(_, t) => Type::TypeList{name: Box::new(Type::Single("List".to_string())), parameters: vec![t.get_type()]},
            //Value::Vector(_, t) => Type::TypeList{name: Box::new(Type::Single("Vector".to_string())), parameters: vec![t.get_type()]},
            Value::Tuple(values) => Type::Tuple(values.iter().map(|v| v.get_type()).collect()),
            Value::Function(_,parameters, effects, return_type, _, _) => Type::Function{parameters: parameters.iter().map(|(_, t)| t.get_type()).collect(), effects: effects.clone(), return_type: Box::new(return_type.get_type())},
            Value::Promise(_, t) => Type::TypeList{name: Box::new(Type::Single("Promise".to_string())), parameters: vec![t.get_type()]},
            Value::Algebraic{agb_type, types, name, values} => Type::TypeList{ name: Box::new(Type::Single(name.clone())), parameters: types.iter().map(|t| t.get_type()).collect()},
            Value::Alias{parent, name, value} => name.get_type(),
            Value::Ref(i) => i.value.borrow().get_type(),
        }
    }

    fn is_ref(&self) -> bool {
        match self {
            Value::Ref(_) => true,
            _ => false,
        }
    }
}

impl TypeUtils for &Value {
    fn get_type(&self) -> Type {
        self.get_type()
    }

    fn is_ref(&self) -> bool {
        self.is_ref()
    }
}


/*pub type ValueImmu = Rc<Value>;
pub type ValueMut = Rc<RefCell<Value>>;

#[derive(Debug, Clone)]
pub enum ValuePtr {
    Immu(Value),
    Mut(Value),
}

impl TypeUtils for ValuePtr {
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
impl TypeUtils for &ValuePtr {
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

impl ValuePtr {
    pub fn new_immu(value: Value) -> Self {
        ValuePtr::Immu(value)
    }
    pub fn new_mut(value: Value) -> Self {
        ValuePtr::Mut(value)
    }
}

impl ValueRef for ValuePtr {
    fn get_value(&self) -> Value {
        match self {
            ValuePtr::Immu(v) => v.clone(),
            ValuePtr::Mut(v) => v.clone(),
        }
    }

    fn get_value_mut(&mut self) -> Value {
        match self {
            ValuePtr::Immu(v) => panic!("Cannot get mutable reference to immutable value"),
            ValuePtr::Mut(v) => v.create_reference(),
        }
    }
}*/


pub trait ValueRef {
    fn get_value(&self) -> Value;

    fn get_value_mut(& mut self) -> Value;
}


