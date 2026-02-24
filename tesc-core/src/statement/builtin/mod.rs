use crate::{
    environment::Environment, parser::Rule, statement::Value, test_error::TestError, TescOptions,
};

use super::Instruction;

mod input;
use input::Input;

mod output;
use output::Output;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub enum Builtin {
    Input(Input),
    Output(Output),
}

impl Instruction for Builtin {
    fn parse(pair: Pair<Rule>) -> Self {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::input => Self::Input(Input::parse(inner)),
            Rule::output => Self::Output(Output::parse(inner)),
            _ => unreachable!(),
        }
    }

    fn eval(&self, opts: &TescOptions, env: &mut Environment) -> Result<Value, TestError> {
        match self {
            Builtin::Input(input) => input.eval(opts, env),
            Builtin::Output(output) => output.eval(opts, env),
        }
    }
}
