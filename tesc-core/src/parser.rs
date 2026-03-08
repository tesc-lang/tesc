use chumsky::prelude::*;

use crate::{
    lexer::{Spanned, Token},
    statement::{module::Module, Instruction},
};

pub fn parse<'tokens, 'src>(
    tokens: &'tokens [Spanned<Token<'src>>],
    src_len: usize,
) -> ParseResult<Module, Rich<'tokens, Token<'src>>> {
    let eof_span: SimpleSpan = (src_len..src_len).into();
    let token_stream = tokens.map(eof_span, |(t, s)| (t, s));

    Module::parser().parse(token_stream)
}
