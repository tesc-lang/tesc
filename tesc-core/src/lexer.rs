use chumsky::prelude::*;

use crate::statement::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'src> {
    String(&'src str),

    Keyword(&'src str),
    Ident(&'src str),

    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,

    Semicolon,
}

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>, extra::Err<Rich<'src, char, SimpleSpan>>>
{
    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    let string = choice((none_of("\\\""), just("\\").ignore_then(any())))
        .repeated()
        .to_slice()
        .map(|s: &str| Token::String(s))
        .delimited_by(just('"').ignored(), just('"').ignored());

    let ident = any()
        .filter(|c: &char| {
            !c.is_ascii_digit()
                && !c.is_whitespace()
                && !matches!(c, '(' | ')' | '[' | ']' | '{' | '}' | '"' | ';')
        })
        .then(
            any()
                .filter(|c: &char| {
                    !c.is_whitespace()
                        && !matches!(c, '(' | ')' | '[' | ']' | '{' | '}' | '"' | ';')
                })
                .repeated(),
        )
        .to_slice()
        .map(|s: &str| Token::Ident(s));

    let keyword = text::keyword("test").map(|s: &str| Token::Keyword(s));

    let open_paren = just('(').ignored().map(|()| Token::OpenParen);
    let close_paren = just(')').ignored().map(|()| Token::CloseParen);

    let open_curly = just('{').ignored().map(|()| Token::OpenCurly);
    let close_curly = just('}').ignored().map(|()| Token::CloseCurly);

    let semicolon = just(';').ignored().map(|()| Token::Semicolon);

    choice((
        string,
        keyword,
        ident,
        open_paren,
        close_paren,
        open_curly,
        close_curly,
        semicolon,
    ))
    .map_with(|tok, e| (tok, e.span()))
    .padded_by(comment.repeated())
    .padded()
    .recover_with(skip_then_retry_until(any().ignored(), end()))
    .repeated()
    .collect()
}
