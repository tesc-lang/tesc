use pest::iterators::Pair;

use crate::{
    environment::Environment,
    parser::Rule,
    statement::{
        block::Block, builtin::Builtin, function_call::FunctionCall, string_literal::StringLiteral,
        Instruction, Value,
    },
    test_error::TestError,
    TescOptions,
};

#[derive(Clone, Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
}

#[derive(Clone, Debug)]
pub enum ExpressionKind {
    Block(Block),

    FunctionCall(FunctionCall),
    Builtin(Builtin),

    StringLiteral(StringLiteral),
}

impl Instruction for Expression {
    fn parse(pair: Pair<Rule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::builtin => Self {
                kind: ExpressionKind::Builtin(Builtin::parse(pair)),
            },
            Rule::function_call => Self {
                kind: ExpressionKind::FunctionCall(FunctionCall::parse(pair)),
            },

            Rule::string_literal => Self {
                kind: ExpressionKind::StringLiteral(StringLiteral::parse(pair)),
            },

            Rule::block => Self {
                kind: ExpressionKind::Block(Block::parse(pair)),
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
            | Rule::expression
            | Rule::test => unreachable!(),
        }
    }

    fn eval(&self, _opts: &TescOptions, _env: &mut Environment) -> Result<Value, TestError> {
        match &self.kind {
            ExpressionKind::Block(block) => block.eval(_opts, _env),
            ExpressionKind::FunctionCall(function_call) => function_call.eval(_opts, _env),
            ExpressionKind::Builtin(builtin) => builtin.eval(_opts, _env),
            ExpressionKind::StringLiteral(string) => string.eval(_opts, _env),
        }
    }
}
