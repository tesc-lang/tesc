use chumsky::input::ValueInput;
use chumsky::prelude::*;
use chumsky::Parser;

use crate::error::RunTimeErr;
use crate::{
    environment::{RunTimeEnv, TypeCheckEnv},
    error::{expect, TypeCheckErr},
    lexer::Token,
    statement::{expression::Expression, ident::Ident, Instruction, ParserExtra, Statement, Value},
    types::Type,
    TescArgs,
};

#[derive(Debug)]
pub struct Test {
    name: Ident,
    command: Expression,
    body: Box<Statement>,
    span: SimpleSpan,
}

impl Test {
    pub fn parser<'tokens, 'src: 'tokens, I>(
        stmt: impl Parser<'tokens, I, Statement, ParserExtra<'tokens, 'src>> + Clone + 'tokens,
    ) -> impl Parser<'tokens, I, Self, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
    {
        let ident = Ident::parser();
        let expr = Expression::parser(stmt.clone());

        just(Token::Keyword("test"))
            .ignore_then(ident)
            .then_ignore(just(Token::OpenParen))
            .then(expr)
            .then_ignore(just(Token::CloseParen).ignored())
            .then(stmt)
            .map_with(move |((name, command), body), e| Test {
                name,
                command,
                body: Box::new(body),
                span: e.span(),
            })
    }

    pub fn check(&self, opts: &TescArgs, env: &mut TypeCheckEnv) -> Result<Type, TypeCheckErr> {
        env.push_frame();

        expect(
            Type::String,
            self.command.check(opts, env)?,
            self.command.span,
        )?;
        env.insert("self".to_string(), Type::Process);

        let result = (*self.body).check(opts, env);
        env.pop_frame();
        result
    }

    pub fn eval(&self, opts: &TescArgs, env: &mut RunTimeEnv) -> Result<Value, RunTimeErr> {
        println!("Testing: {}", self.name);
        env.push_frame();

        if let Value::String(command) = self.command.eval(opts, env)? {
            env.spawn("self".to_string(), command);
        }

        let result = (*self.body).eval(opts, env);
        env.pop_frame();
        result
    }
}
