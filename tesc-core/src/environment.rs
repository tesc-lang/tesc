use std::{cell::RefCell, collections::HashMap, process::Stdio, rc::Rc};

use crate::statement::Value;

pub type Frame = HashMap<String, Rc<RefCell<Value>>>;

pub struct Environment {
    // TODO: Spaghetti stack
    stack: Vec<Frame>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push_frame(&mut self) {
        self.stack.push(Frame::new());
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
