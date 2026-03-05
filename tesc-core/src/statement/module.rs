use chumsky::{input::ValueInput, span::SimpleSpan, IterParser, Parser};

use crate::{
    environment::Environment,
    lexer::Token,
    statement::{Instruction, ParserExtra, Spanned, Statement, Value},
    test_error::TestError,
    TescOptions,
};

#[derive(Clone, Debug)]
pub struct Module {
    ast: Vec<Statement>,
}

impl Instruction for Module {
    fn parser<'tokens, 'src: 'tokens, I>(
    ) -> impl Parser<'tokens, I, Spanned<Self>, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
    {
        Statement::parser()
            .repeated()
            .collect()
            .map_with(|ast: Vec<_>, e| {
                (
                    Module {
                        ast: ast.into_iter().map(|(e, _)| e).collect(),
                    },
                    e.span(),
                )
            })
    }

    fn eval(&self, opts: &TescOptions, _env: &mut Environment) -> Result<Value, TestError> {
        let mut failures = Vec::new();
        for node in &self.ast {
            match node.eval(opts, _env) {
                Ok(_) => (),
                Err(e) => failures.push(e),
            }
        }
        if failures.is_empty() {
            Ok(Value::Void)
        } else {
            Err(TestError::Multiple(failures))
        }
    }
}
