use pest::iterators::Pair;

use crate::{
    environment::Environment,
    parser::Rule,
    statement::{test::Test, Instruction, Statement, StatementKind},
    test_error::TestError,
    TescOptions,
};

#[derive(Clone, Debug)]
pub struct Module {
    source: String,
    ast: Vec<Statement>,
}

impl Module {
    pub fn parse(source: String, pair: Pair<Rule>) -> Module {
        let mut ast = Vec::new();
        for pair in pair.into_inner() {
            if pair.as_rule() != Rule::EOI {
                ast.push(Statement::parse(pair));
            }
        }
        Module { source, ast }
    }

    pub fn eval(&self, opts: &TescOptions) -> Result<(), Vec<TestError>> {
        let mut failures = Vec::new();
        let mut env = Environment::new();
        for node in &self.ast {
            match node.eval(opts, &mut env) {
                Ok(_) => (),
                Err(e) => failures.push(e),
            }
        }
        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures)
        }
    }
}
