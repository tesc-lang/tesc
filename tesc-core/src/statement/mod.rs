use crate::{
    environment::{RunTimeEnv, TypeCheckEnv},
    error::{RunTimeErr, TypeCheckErr},
    lexer::Token,
    statement::test::Test,
    types::{Type, Value},
    TescArgs,
};

use chumsky::{input::ValueInput, prelude::*};

pub type ParserExtra<'tokens, 'src> = extra::Err<Rich<'tokens, Token<'src>>>;

pub(crate) mod module;

mod expression;
use expression::Expression;

mod test;

mod block;

mod method_call;

mod ident;
mod string_literal;

#[derive(Debug)]
pub struct Statement {
    kind: StatementKind,
    span: SimpleSpan,
}

#[derive(Debug)]
pub enum StatementKind {
    Test(Test),
    Expression(Expression),
}

pub trait Instruction {
    fn parser<'tokens, 'src: 'tokens, I>(
    ) -> impl Parser<'tokens, I, Self, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>;

    fn check(&self, opts: &TescArgs, env: &mut TypeCheckEnv) -> Result<Type, TypeCheckErr>;
    fn eval(&self, opts: &TescArgs, env: &mut RunTimeEnv) -> Result<Value, RunTimeErr>;
}

impl Instruction for Statement {
    fn parser<'tokens, 'src: 'tokens, I>(
    ) -> impl Parser<'tokens, I, Self, ParserExtra<'tokens, 'src>> + Clone
    where
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
    {
        recursive(|stmt| {
            let test = Test::parser(stmt.clone()).map_with(|test, e| Statement {
                kind: StatementKind::Test(test),
                span: e.span(),
            });
            let expression = Expression::parser(stmt)
                .then_ignore(just(Token::Semicolon))
                .map_with(|expr, e| Statement {
                    kind: StatementKind::Expression(expr),
                    span: e.span(),
                });

            choice((test, expression))
        })
    }

    fn check(&self, _opts: &TescArgs, _env: &mut TypeCheckEnv) -> Result<Type, TypeCheckErr> {
        match &self.kind {
            StatementKind::Test(test) => test.check(_opts, _env),
            StatementKind::Expression(expression) => expression.check(_opts, _env),
        }?;
        Ok(Type::Void)
    }

    fn eval(&self, _opts: &TescArgs, _env: &mut RunTimeEnv) -> Result<Value, RunTimeErr> {
        match &self.kind {
            StatementKind::Test(test) => test.eval(_opts, _env),
            StatementKind::Expression(expression) => expression.eval(_opts, _env),
        }?;
        Ok(Value::Void)
    }
}
