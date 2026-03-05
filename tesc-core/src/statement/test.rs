use chumsky::input::ValueInput;
use chumsky::prelude::*;
use chumsky::Parser;

use crate::statement::expression::Expression;
use crate::{
    environment::Environment,
    lexer::Token,
    statement::{Instruction, ParserExtra, Spanned, Statement, Value},
    test_error::TestError,
    TescOptions,
};

#[derive(Debug, Clone)]
pub struct Test {
    name: String,
    command: Expression,
    body: Box<Statement>,
}

impl Test {
    pub fn parser<'tokens, 'src: 'tokens, I>(
        stmt: impl Parser<'tokens, I, Spanned<Statement>, ParserExtra<'tokens, 'src>> + Clone + 'tokens,
    ) -> impl Parser<'tokens, I, Spanned<Self>, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
    {
        let ident = select! { Token::Ident(ident) => ident };
        let expr = Expression::parser(stmt.clone());

        just(Token::Keyword("test"))
            .ignore_then(ident)
            .then_ignore(just(Token::OpenParen))
            .then(expr)
            .then_ignore(just(Token::CloseParen).ignored())
            .then(stmt)
            .map_with(move |((name, (command, _)), (body, _)), e| {
                (
                    Test {
                        name: name.to_string(),
                        command,
                        body: Box::new(body),
                    },
                    e.span(),
                )
            })
    }

    pub fn eval(&self, opts: &TescOptions, env: &mut Environment) -> Result<Value, TestError> {
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
