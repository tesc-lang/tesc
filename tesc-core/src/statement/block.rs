use chumsky::{input::ValueInput, prelude::*, Parser};

use crate::{
    environment::Environment,
    lexer::Token,
    statement::{expression::Expression, Instruction, ParserExtra, Spanned, Statement, Value},
    test_error::TestError,
    TescOptions,
};

#[derive(Clone, Debug)]
pub struct Block {
    body: Vec<Statement>,
    tail: Option<Expression>,
}

impl Block {
    pub fn parser<'tokens, 'src: 'tokens, I>(
        stmt: impl Parser<'tokens, I, Spanned<Statement>, ParserExtra<'tokens, 'src>> + Clone,
        expr: impl Parser<'tokens, I, Spanned<Expression>, ParserExtra<'tokens, 'src>> + Clone,
    ) -> impl Parser<'tokens, I, Spanned<Self>, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
    {
        just(Token::OpenCurly)
            .ignore_then(stmt.repeated().collect::<Vec<_>>())
            .then(expr.or_not())
            .then_ignore(just(Token::CloseCurly))
            .map(|(body, tail)| Block {
                body: body.into_iter().map(|(s, _)| s).collect::<Vec<_>>(),
                tail: if let Some((tail, _)) = tail {
                    Some(tail)
                } else {
                    None
                },
            })
            .map_with(|block, e| (block, e.span()))
    }

    pub fn eval(&self, _opts: &TescOptions, env: &mut Environment) -> Result<Value, TestError> {
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
