use chumsky::{select, span::SimpleSpan, Parser};

use crate::{
    environment::{RunTimeEnv, TypeCheckEnv},
    error::{RunTimeErr, TypeCheckErr, TypeCheckErrKind},
    lexer::Token,
    statement::{Instruction, Value},
    types::Type,
    TescArgs,
};

use super::ParserExtra;

#[derive(Clone, Debug)]
pub struct Ident {
    pub span: SimpleSpan,
    pub ident: String,
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)
    }
}

impl Instruction for Ident {
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
        select! { Token::Ident(ident) => ident.to_string() }.map_with(|ident, e| Ident {
            ident,
            span: e.span(),
        })
    }

    fn check(&self, _opts: &TescArgs, env: &mut TypeCheckEnv) -> Result<Type, TypeCheckErr> {
        match env.get(&self.ident) {
            Some(v) => Ok(Type::Reference(Box::new(v))),
            None => Err(TypeCheckErr {
                span: self.span,
                kind: TypeCheckErrKind::UndefinedIdentifier(self.ident.clone()),
            }),
        }
    }

    fn eval(&self, _opts: &TescArgs, env: &mut RunTimeEnv) -> Result<Value, RunTimeErr> {
        match env.get(&self.ident) {
            Some(v) => Ok(Value::Reference(v)),
            None => unreachable!(),
        }
    }
}
