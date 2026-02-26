use std::{
    io::{Read, Write},
    process::Stdio,
};

pub struct Frame(Vec<Item>);

#[derive(Clone, Debug)]
pub enum Item {}

pub struct Environment {
    pub test: Option<String>,
    pub child: Option<std::process::Child>,
    _stack: Vec<Frame>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            test: None,
            child: None,
            _stack: Vec::new(),
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
}
