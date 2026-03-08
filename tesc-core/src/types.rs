use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum Value {
    String(String),

    Process(std::process::Child),

    Reference(Rc<RefCell<Value>>),
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,

    Process,

    Reference(Box<Type>),
    Void,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::String => write!(f, "String"),
            Type::Process => write!(f, "Process"),
            Type::Reference(t) => write!(f, "&{t}"),
            Type::Void => write!(f, "()"),
        }
    }
}
