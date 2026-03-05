use chumsky::{
    input::ValueInput,
    prelude::{SimpleSpan, *},
};

use crate::{
    environment::Environment,
    lexer::Token,
    statement::{
        block::Block, ident::Ident, method_call::MethodCall, string_literal::StringLiteral,
        Instruction, ParserExtra, Spanned, Statement, Value,
    },
    test_error::TestError,
    TescOptions,
};

#[derive(Clone, Debug)]
pub enum Expression {
    Block(Box<Block>),
    // FunctionCall(FunctionCall),
    // Builtin(Builtin),
    MethodCall(Box<MethodCall>),

    Ident(Ident),
    StringLiteral(StringLiteral),
}

impl Expression {
    pub fn parser<'tokens, 'src: 'tokens, I>(
        stmt: impl Parser<'tokens, I, Spanned<Statement>, ParserExtra<'tokens, 'src>> + Clone + 'tokens,
    ) -> impl Parser<'tokens, I, Spanned<Self>, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
    {
        recursive(|expr| {
            let string =
                StringLiteral::parser().map(|(string, _)| Expression::StringLiteral(string));
            let ident = Ident::parser().map(|(ident, _)| Expression::Ident(ident));

            let block = Block::parser(stmt, expr.clone())
                .map(|(block, _)| Expression::Block(Box::new(block)));

            let base = choice((string, ident, block)).map_with(|expr, e| (expr, e.span()));

            let method_call = MethodCall::parser(base.clone())
                .map(|(method_call, _)| Expression::MethodCall(Box::new(method_call)))
                .map_with(|expr, e| (expr, e.span()));

            choice((method_call, base))
        })
    }

    pub fn eval(&self, _opts: &TescOptions, _env: &mut Environment) -> Result<Value, TestError> {
        match &self {
            Expression::Block(block) => block.eval(_opts, _env),
            Expression::MethodCall(method_call) => method_call.eval(_opts, _env),
            // ExpressionKind::Builtin(builtin) => builtin.eval(_opts, _env),
            Expression::Ident(ident) => ident.eval(_opts, _env),
            Expression::StringLiteral(string) => string.eval(_opts, _env),
        }
    }
}
