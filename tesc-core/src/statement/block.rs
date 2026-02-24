use pest::iterators::Pair;

use crate::{
    environment::Environment,
    parser::Rule,
    statement::{expression::Expression, Statement, Value},
    test_error::TestError,
    TescOptions,
};

pub type Block = Vec<BlockItem>;

#[derive(Clone, Debug)]
pub enum BlockItem {
    Statement(Statement),
    Expression(Expression),
}

impl super::Instruction for Block {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut block = Block::new();
        for inner in pair.into_inner() {
            block.push(BlockItem::parse(inner));
        }
        block
    }

    fn eval(&self, _opts: &TescOptions, _env: &mut Environment) -> Result<Value, TestError> {
        let mut result = Value::Void;
        for item in self {
            result = item.eval(_opts, _env)?;
        }
        Ok(result)
    }
}

impl super::Instruction for BlockItem {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::statement => BlockItem::Statement(Statement::parse(pair)),
            Rule::expression => BlockItem::Expression(Expression::parse(pair)),

            _ => unreachable!(),
        }
    }

    fn eval(&self, _opts: &TescOptions, _env: &mut Environment) -> Result<Value, TestError> {
        match self {
            BlockItem::Statement(statement) => statement.eval(_opts, _env),
            BlockItem::Expression(expression) => expression.eval(_opts, _env),
        }
    }
}
