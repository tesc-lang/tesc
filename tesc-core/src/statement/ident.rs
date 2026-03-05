use chumsky::{select, Parser};

use crate::{
    environment::Environment, lexer::Token, statement::Value, test_error::TestError, TescOptions,
};

use super::{ParserExtra, Spanned};

#[derive(Clone, Debug)]
pub struct Ident(pub String);

impl super::Instruction for Ident {
    fn parser<'tokens, 'src: 'tokens, I>(
    ) -> impl Parser<'tokens, I, Spanned<Self>, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: chumsky::input::ValueInput<
            'tokens,
            Token = crate::lexer::Token<'src>,
            Span = chumsky::prelude::SimpleSpan,
        >,
    {
        select! { Token::Ident(ident) => ident.to_string() }
            .map_with(|ident, e| (Ident(ident), e.span()))
    }
    fn eval(&self, _opts: &TescOptions, env: &mut Environment) -> Result<Value, TestError> {
        match env.get(&self.0) {
            Some(v) => Ok(Value::Reference(v)),
            None => Err(TestError::UndefinedIdentifier(self.0.clone())),
        }
    }
}
