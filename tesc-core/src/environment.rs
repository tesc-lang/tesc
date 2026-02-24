use std::{
    io::{Read, Write},
    process::Stdio,
};

use crate::{r#type::Type, statement::Statement};

pub struct Environment {
    pub test: Option<String>,
    pub child: Option<std::process::Child>,
    stack: Vec<Frame>,

    global_scope: Vec<Item>,
}

pub struct Frame;

#[derive(Clone, Debug)]
pub enum Item {
    Function {
        id: String,
        arguments: Vec<Type>,
        body: Vec<Statement>,
    },
}

impl Environment {
    pub fn new() -> Self {
        Self {
            test: None,
            child: None,
            stack: Vec::new(),
            global_scope: Vec::new(),
        }
    }

    pub fn spawn(&mut self, command: String) {
        let mut split = command.split(' ');
        let command = split.next().unwrap();
        let args = split.collect::<Vec<_>>();
        self.child = Some(
            std::process::Command::new(command)
                .args(args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap(),
        );
    }

    pub fn kill_child(&mut self) {
        self.child.take().unwrap().kill().unwrap();
    }

    pub fn send_to_child(&mut self, bytes: &[u8]) {
        self.child
            .as_ref()
            .unwrap()
            .stdin
            .as_ref()
            .unwrap()
            .write_all(bytes)
            .unwrap();
    }

    pub fn recieve_from_child(&mut self, buf: &mut String) {
        self.child
            .as_mut()
            .unwrap()
            .stdout
            .as_mut()
            .unwrap()
            .read_to_string(buf)
            .unwrap();
    }

    pub fn get_fn(&self, id: String, arguments: &[Type]) -> Result<Item, ()> {
        for item in &self.global_scope {
            match item {
                Item::Function {
                    id: fn_id,
                    arguments: fn_arguments,
                    ..
                } => {
                    if fn_id == &id && argument_eq(arguments, fn_arguments) {
                        return Ok(item.clone());
                    }
                }
            }
        }
        Err(())
    }
}

fn argument_eq(args1: &[Type], args2: &[Type]) -> bool {
    if args1.len() != args2.len() {
        return false;
    }
    for (t1, t2) in args1.iter().zip(args2) {
        if t1 != t2 {
            return false;
        }
    }
    true
}
