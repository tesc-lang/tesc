use pest::iterators::Pair;

use crate::{
    environment::Environment,
    parser::Rule,
    statement::{expression::Expression, Instruction, Statement, Value},
    test_error::TestError,
    TescOptions,
};

#[derive(Debug, Clone)]
pub struct Test {
    name: String,
    command: Box<Expression>,
    body: Box<Statement>,
}

impl Instruction for Test {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let command = Box::new(Expression::parse(inner.next().unwrap()));
        let body = Box::new(Statement::parse(inner.next().unwrap()));
        Test {
            name,
            command,
            body,
        }
    }

    fn eval(&self, opts: &TescOptions, env: &mut Environment) -> Result<Value, TestError> {
        println!("Testing: {}", self.name);
        if let Value::String(command) = self.command.eval(opts, env)? {
            env.test = Some(self.name.clone());
            env.spawn(command);
        }

        let result = (*self.body).eval(opts, env);
        env.kill_child();
        result
    }
}
