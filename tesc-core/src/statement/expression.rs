use chumsky::{
    input::ValueInput,
    prelude::{SimpleSpan, *},
};

use crate::{
    environment::{RunTimeEnv, TypeCheckEnv},
    error::{RunTimeErr, TypeCheckErr},
    lexer::Token,
    statement::{
        block::Block, ident::Ident, method_call::MethodCall, string_literal::StringLiteral,
        Instruction, ParserExtra, Statement, Value,
    },
    types::Type,
    TescArgs,
};

#[derive(Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: SimpleSpan,
}

#[derive(Debug)]
pub enum ExpressionKind {
    Block(Box<Block>),
    MethodCall(Box<MethodCall>),

    Ident(Ident),
    StringLiteral(StringLiteral),
}

impl Expression {
    pub fn parser<'tokens, 'src: 'tokens, I>(
        stmt: impl Parser<'tokens, I, Statement, ParserExtra<'tokens, 'src>> + Clone + 'tokens,
    ) -> impl Parser<'tokens, I, Self, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
    {
        recursive(|expr| {
            let string = StringLiteral::parser().map_with(|s, e| Expression {
                kind: ExpressionKind::StringLiteral(s),
                span: e.span(),
            });
            let ident = Ident::parser().map_with(|ident, e| Expression {
                kind: ExpressionKind::Ident(ident),
                span: e.span(),
            });

            let block = Block::parser(stmt, expr.clone()).map_with(|block, e| Expression {
                kind: ExpressionKind::Block(Box::new(block)),
                span: e.span(),
            });

            let base = choice((string, ident, block));

            let method_call = MethodCall::parser(base.clone());

            choice((method_call, base))
        })
    }

    pub fn check(&self, _opts: &TescArgs, _env: &mut TypeCheckEnv) -> Result<Type, TypeCheckErr> {
        match &self.kind {
            ExpressionKind::Block(block) => block.check(_opts, _env),
            ExpressionKind::MethodCall(method_call) => method_call.check(_opts, _env),
            ExpressionKind::Ident(ident) => ident.check(_opts, _env),
            ExpressionKind::StringLiteral(string) => string.check(_opts, _env),
        }
    }

    pub fn eval(&self, _opts: &TescArgs, _env: &mut RunTimeEnv) -> Result<Value, RunTimeErr> {
        match &self.kind {
            ExpressionKind::Block(block) => block.eval(_opts, _env),
            ExpressionKind::MethodCall(method_call) => method_call.eval(_opts, _env),
            ExpressionKind::Ident(ident) => ident.eval(_opts, _env),
            ExpressionKind::StringLiteral(string) => string.eval(_opts, _env),
        }
    }
}
