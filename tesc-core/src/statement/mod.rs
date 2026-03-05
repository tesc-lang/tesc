use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment, lexer::Token, statement::test::Test, test_error::TestError,
    TescOptions,
};

use chumsky::{input::ValueInput, prelude::*};

pub type ParserExtra<'tokens, 'src> = extra::Err<Rich<'tokens, Token<'src>>>;
pub type Spanned<T> = (T, SimpleSpan);

pub mod module;

mod expression;
use expression::Expression;

pub mod test;

// mod builtin;
mod block;

// mod function_call;
mod method_call;

mod ident;
mod string_literal;

#[derive(Clone, Debug)]
pub enum Statement {
    Test(Test),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Value {
    String(String),

    Process(std::process::Child),

    Reference(Rc<RefCell<Value>>),
    Void,
}

pub trait Instruction {
    fn parser<'tokens, 'src: 'tokens, I>(
    ) -> impl Parser<'tokens, I, Spanned<Self>, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>;

    // fn check(&self, type_checker: &mut TypeChecker) -> Result<Type, TypeCheckError>;
    fn eval(&self, opts: &TescOptions, env: &mut Environment) -> Result<Value, TestError>;
}

impl Instruction for Statement {
    fn parser<'tokens, 'src: 'tokens, I>(
    ) -> impl Parser<'tokens, I, Spanned<Self>, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
    {
        recursive(|stmt| {
            let test = Test::parser(stmt.clone()).map(|(test, _)| Statement::Test(test));
            let expression = Expression::parser(stmt)
                .then_ignore(just(Token::Semicolon))
                .map(|(expr, _)| Statement::Expression(expr));

            choice((test, expression)).map_with(|stmt, e| (stmt, e.span()))
        })
    }

    fn eval(&self, _opts: &TescOptions, _env: &mut Environment) -> Result<Value, TestError> {
        match &self {
            Statement::Test(test) => test.eval(_opts, _env),
            // StatementKind::Block(block) => block.eval(_opts, _env),
            Statement::Expression(expression) => expression.eval(_opts, _env),
        }?;
        Ok(Value::Void)
    }
}
