use pest::iterators::Pair;

use crate::{
    environment::Environment, parser::Rule, statement::Value, test_error::TestError, TescOptions,
};

pub type StringLiteral = String;

impl super::Instruction for StringLiteral {
    fn parse(pair: Pair<Rule>) -> Self {
        let value = pair.as_str();
        value[1..value.len() - 1].to_string()
    }

    fn eval(&self, _opts: &TescOptions, _env: &mut Environment) -> Result<Value, TestError> {
        Ok(Value::String(self.clone()))
    }
}
