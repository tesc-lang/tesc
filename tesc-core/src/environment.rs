use std::{cell::RefCell, collections::HashMap, process::Stdio, rc::Rc};

use crate::types::{Type, Value};

pub type TypeCheckFrame = HashMap<String, Type>;

pub struct TypeCheckEnv {
    // TODO: Spaghetti stack
    stack: Vec<TypeCheckFrame>,
}

impl Default for TypeCheckEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeCheckEnv {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push_frame(&mut self) {
        self.stack.push(TypeCheckFrame::new());
    }

    pub fn pop_frame(&mut self) {
        self.stack.pop().unwrap();
    }

    pub fn insert(&mut self, ident: String, r#type: Type) {
        self.stack.last_mut().unwrap().insert(ident, r#type);
    }

    pub fn get(&mut self, ident: &String) -> Option<Type> {
        for frame in self.stack.iter().rev() {
            if let Some(value) = frame.get(ident) {
                return Some(value.clone());
            }
        }
        None
    }
}

pub type RunTimeFrame = HashMap<String, Rc<RefCell<Value>>>;
pub struct RunTimeEnv {
    // TODO: Spaghetti stack
    stack: Vec<RunTimeFrame>,
}

impl Default for RunTimeEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl RunTimeEnv {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push_frame(&mut self) {
        self.stack.push(RunTimeFrame::new());
    }

    pub fn pop_frame(&mut self) {
        self.stack.pop().unwrap();
    }

    pub fn get(&mut self, ident: &String) -> Option<Rc<RefCell<Value>>> {
        for frame in self.stack.iter().rev() {
            if let Some(value) = frame.get(ident) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn spawn(&mut self, name: String, command: String) {
        let mut split = command.split(' ');
        let command = split.next().unwrap();
        let args = split.collect::<Vec<_>>();
        self.stack.last_mut().unwrap().insert(
            name,
            Rc::new(RefCell::new(Value::Process(
                std::process::Command::new(command)
                    .args(args)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap(),
            ))),
        );
    }
}
