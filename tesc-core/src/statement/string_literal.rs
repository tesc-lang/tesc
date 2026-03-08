use chumsky::{select, span::SimpleSpan, Parser};

use crate::{
    environment::{RunTimeEnv, TypeCheckEnv},
    error::{RunTimeErr, TypeCheckErr},
    lexer::Token,
    statement::{Instruction, Value},
    types::Type,
    TescArgs,
};

use super::ParserExtra;

#[derive(Debug)]
pub struct StringLiteral {
    pub string: String,
    pub span: SimpleSpan,
}

impl Instruction for StringLiteral {
    fn parser<'tokens, 'src: 'tokens, I>(
    ) -> impl Parser<'tokens, I, Self, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: chumsky::input::ValueInput<
            'tokens,
            Token = crate::lexer::Token<'src>,
            Span = chumsky::prelude::SimpleSpan,
        >,
    {
        select! { Token::String(string) => string.to_string() }.map_with(|string, e| {
            StringLiteral {
                string,
                span: e.span(),
            }
        })
    }

    fn check(&self, _opts: &TescArgs, _env: &mut TypeCheckEnv) -> Result<Type, TypeCheckErr> {
        Ok(Type::String)
    }

    fn eval(&self, _opts: &TescArgs, _env: &mut RunTimeEnv) -> Result<Value, RunTimeErr> {
        Ok(Value::String(self.string.clone()))
    }
}
