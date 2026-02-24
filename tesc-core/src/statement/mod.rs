use pest::iterators::Pair;

use crate::{
    environment::Environment,
    parser::Rule,
    statement::{expression::ExpressionKind, test::Test},
    test_error::TestError,
    TescOptions,
};

mod expression;
use expression::Expression;

pub mod test;

mod block;
use block::Block;

mod builtin;

mod function_call;

mod string_literal;

#[derive(Clone, Debug)]
pub struct Statement {
    pub kind: StatementKind,
}

#[derive(Clone, Debug)]
pub enum StatementKind {
    Test(Test),
    Expression(Expression),
    Block(Block),
}

impl Statement {
    pub fn expression(kind: ExpressionKind) -> Self {
        Self {
            kind: StatementKind::Expression(Expression { kind }),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    String(String),

    TestResult(Result<(), TestError>),

    Void,
}

pub trait Instruction
where
    Self: std::marker::Sized,
{
    fn parse(pair: Pair<Rule>) -> Self;
    // fn check(&self, type_checker: &mut TypeChecker) -> Result<Type, TypeCheckError>;
    fn eval(&self, opts: &TescOptions, env: &mut Environment) -> Result<Value, TestError>;
}

impl Instruction for Statement {
    fn parse(pair: Pair<Rule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::test => Statement {
                kind: StatementKind::Test(Test::parse(pair)),
            },

            Rule::block => Statement {
                kind: StatementKind::Block(Block::parse(pair)),
            },

            Rule::expression => Statement {
                kind: StatementKind::Expression(Expression::parse(pair)),
            },

            Rule::module
            | Rule::EOI
            | Rule::WHITESPACE
            | Rule::alpha
            | Rule::digit
            | Rule::ident
            | Rule::arguments
            | Rule::input
            | Rule::output
            | Rule::statement
            | Rule::function_call
            | Rule::builtin
            | Rule::string_literal => unreachable!(),
        }
    }

    fn eval(&self, _opts: &TescOptions, _env: &mut Environment) -> Result<Value, TestError> {
        match &self.kind {
            StatementKind::Test(test) => test.eval(_opts, _env),

            StatementKind::Block(block) => block.eval(_opts, _env),
            StatementKind::Expression(expression) => expression.eval(_opts, _env),
        }?;
        Ok(Value::Void)
    }
}
