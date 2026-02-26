use pest::iterators::Pair;

use crate::{
    environment::Environment,
    parser::Rule,
    statement::{Instruction, Statement, Value},
    test_error::TestError,
    TescOptions,
};

#[derive(Clone, Debug)]
pub struct Module {
    ast: Vec<Statement>,
}

impl Instruction for Module {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut ast = Vec::new();
        for pair in pair.into_inner() {
            if pair.as_rule() != Rule::EOI {
                ast.push(Statement::parse(pair));
            }
        }
        Module { ast }
    }

    fn eval(&self, opts: &TescOptions, _env: &mut Environment) -> Result<Value, TestError> {
        let mut failures = Vec::new();
        for node in &self.ast {
            match node.eval(opts, _env) {
                Ok(_) => (),
                Err(e) => failures.push(e),
            }
        }
        if failures.is_empty() {
            Ok(Value::Void)
        } else {
            Err(TestError::Multiple(failures))
        }
    }
}

impl Module {
    pub fn run(&self, _opts: &TescOptions, _env: &mut Environment) -> Result<Value, TestError> {
        self.eval(_opts, _env)
    }
}
