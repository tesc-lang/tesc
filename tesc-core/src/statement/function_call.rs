use pest::iterators::Pair;

use super::{Instruction, Value};

use crate::{
    environment::Environment, parser::Rule, statement::expression::Expression,
    test_error::TestError, TescOptions,
};

#[derive(Debug, Clone)]
pub struct FunctionCall {
    name: String,
    arguments: Vec<Expression>,
}

impl Instruction for FunctionCall {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let mut arguments = Vec::new();
        if let Some(argument_pairs) = inner.next() {
            for pair in argument_pairs.into_inner() {
                arguments.push(Expression::parse(pair));
            }
        }
        FunctionCall { name, arguments }
    }

    fn eval(&self, _opts: &TescOptions, _env: &mut Environment) -> Result<Value, TestError> {
        todo!();
    }
}
