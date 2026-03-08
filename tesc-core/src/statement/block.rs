use chumsky::{input::ValueInput, prelude::*, Parser};

use crate::{
    environment::{RunTimeEnv, TypeCheckEnv},
    error::{RunTimeErr, TypeCheckErr, TypeCheckErrKind},
    lexer::Token,
    statement::{expression::Expression, Instruction, ParserExtra, Statement, Value},
    types::Type,
    TescArgs,
};

#[derive(Debug)]
pub struct Block {
    body: Vec<Statement>,
    tail: Option<Expression>,
    pub span: SimpleSpan,
}

impl Block {
    pub fn parser<'tokens, 'src: 'tokens, I>(
        stmt: impl Parser<'tokens, I, Statement, ParserExtra<'tokens, 'src>> + Clone,
        expr: impl Parser<'tokens, I, Expression, ParserExtra<'tokens, 'src>> + Clone,
    ) -> impl Parser<'tokens, I, Self, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
    {
        just(Token::OpenCurly)
            .ignore_then(stmt.repeated().collect::<Vec<_>>())
            .then(expr.or_not())
            .then_ignore(just(Token::CloseCurly))
            .map_with(|(body, tail), e| Block {
                body,
                tail,
                span: e.span(),
            })
    }

    pub fn check(&self, _opts: &TescArgs, env: &mut TypeCheckEnv) -> Result<Type, TypeCheckErr> {
        let mut errors = Vec::new();
        env.push_frame();
        for stmt in &self.body {
            match stmt.check(_opts, env) {
                Ok(_) => (),
                Err(e) => errors.push(e),
            }
        }
        let tail = if let Some(expr) = &self.tail {
            match expr.check(_opts, env) {
                Ok(t) => t,
                Err(e) => {
                    errors.push(e);
                    Type::Void
                }
            }
        } else {
            Type::Void
        };

        if errors.is_empty() {
            Ok(tail)
        } else {
            Err(TypeCheckErr {
                kind: TypeCheckErrKind::Multiple(errors),
                span: self.span,
            }
            .flatten())
        }
    }

    pub fn eval(&self, _opts: &TescArgs, env: &mut RunTimeEnv) -> Result<Value, RunTimeErr> {
        env.push_frame();
        for stmt in &self.body {
            stmt.eval(_opts, env)?;
        }
        if let Some(expr) = &self.tail {
            expr.eval(_opts, env)
        } else {
            Ok(Value::Void)
        }
    }
}
