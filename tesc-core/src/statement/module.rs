use chumsky::{input::ValueInput, span::SimpleSpan, IterParser, Parser};

use crate::{
    environment::{RunTimeEnv, TypeCheckEnv},
    error::{RunTimeErr, RunTimeErrKind, TypeCheckErr, TypeCheckErrKind},
    lexer::Token,
    statement::{Instruction, ParserExtra, Statement, Value},
    types::Type,
    TescArgs,
};

#[derive(Debug)]
pub struct Module {
    ast: Vec<Statement>,
    span: SimpleSpan,
}

impl Instruction for Module {
    fn parser<'tokens, 'src: 'tokens, I>(
    ) -> impl Parser<'tokens, I, Self, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
    {
        Statement::parser()
            .repeated()
            .collect()
            .map_with(|ast: Vec<_>, e| Module {
                ast: ast.into_iter().collect(),
                span: e.span(),
            })
    }

    fn check(&self, _opts: &TescArgs, _env: &mut TypeCheckEnv) -> Result<Type, TypeCheckErr> {
        let mut failures = Vec::new();
        for node in &self.ast {
            match node.check(_opts, _env) {
                Ok(_) => (),
                Err(e) => failures.push(e),
            }
        }
        if failures.is_empty() {
            Ok(Type::Void)
        } else {
            Err(TypeCheckErr {
                kind: TypeCheckErrKind::Multiple(failures),
                span: SimpleSpan {
                    start: 0,
                    end: 0,
                    context: (),
                },
            }
            .flatten())
        }
    }

    fn eval(&self, opts: &TescArgs, _env: &mut RunTimeEnv) -> Result<Value, RunTimeErr> {
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
            Err(RunTimeErr {
                kind: RunTimeErrKind::Multiple(failures),
                span: SimpleSpan {
                    start: 0,
                    end: 0,
                    context: (),
                },
            }
            .flatten())
        }
    }
}
