use std::io::{BufRead, Write};

use chumsky::{span::SimpleSpan, Parser};

use crate::{
    environment::{RunTimeEnv, TypeCheckEnv},
    error::{expect, RunTimeErr, RunTimeErrKind, TypeCheckErr, TypeCheckErrKind},
    statement::{
        expression::{Expression, ExpressionKind},
        ident::Ident,
        Instruction, ParserExtra, Value,
    },
    types::Type,
    TescArgs,
};

#[derive(Debug)]
pub struct MethodCall {
    receiver: Expression,
    method: Ident,
    argument: Expression,
    span: SimpleSpan,
}

impl MethodCall {
    pub fn parser<'tokens, 'src: 'tokens, I>(
        expr: impl Parser<'tokens, I, Expression, ParserExtra<'tokens, 'src>> + Clone + 'tokens,
    ) -> impl Parser<'tokens, I, Expression, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: chumsky::input::ValueInput<
            'tokens,
            Token = crate::lexer::Token<'src>,
            Span = chumsky::prelude::SimpleSpan,
        >,
    {
        expr.clone().foldl_with(
            Ident::parser().then(expr).repeated().at_least(1),
            |receiver, (method, argument), e| Expression {
                kind: ExpressionKind::MethodCall(Box::new(MethodCall {
                    receiver,
                    method,
                    argument,
                    span: e.span(),
                })),
                span: e.span(),
            },
        )
    }

    pub fn check(&self, _opts: &TescArgs, env: &mut TypeCheckEnv) -> Result<Type, TypeCheckErr> {
        match self.receiver.check(_opts, env)? {
            Type::Reference(r) => expect(Type::Process, *r, self.receiver.span)?,
            t => expect(
                Type::Reference(Box::new(Type::Process)),
                t,
                self.receiver.span,
            )?,
        };

        expect(
            Type::String,
            self.argument.check(_opts, env)?,
            self.argument.span,
        )?;

        match self.method.ident.as_str() {
            "send" | "expect" => (),
            method => Err(TypeCheckErr {
                kind: TypeCheckErrKind::UndefinedMethod(method.to_string()),
                span: self.method.span,
            })?,
        };

        Ok(Type::Reference(Box::new(Type::Process)))
    }

    pub fn eval(&self, _opts: &TescArgs, env: &mut RunTimeEnv) -> Result<Value, RunTimeErr> {
        let receiver = self.receiver.eval(_opts, env)?;
        let argument = match self.argument.eval(_opts, env) {
            Ok(v) => match v {
                Value::String(s) => s + "\n",
                _ => unreachable!(),
            },
            Err(_) => todo!(),
        };

        match receiver {
            Value::Reference(ref r) => match &mut *r.borrow_mut() {
                Value::String(_) => todo!(),
                Value::Process(ref mut child) => match self.method.ident.as_str() {
                    "send" => {
                        child
                            .stdin
                            .as_mut()
                            .unwrap()
                            .write_all(argument.as_bytes())
                            .unwrap();
                    }
                    "expect" => {
                        let mut buf = String::new();
                        let mut reader = std::io::BufReader::new(child.stdout.as_mut().unwrap());
                        reader.read_line(&mut buf).unwrap();

                        if argument.clone() != buf {
                            return Err(RunTimeErr {
                                kind: RunTimeErrKind::OutputMissmatch {
                                    actual: buf.clone(),
                                    expected: argument.strip_suffix("\n").unwrap().to_string(),
                                },
                                span: SimpleSpan {
                                    start: self.method.span.start,
                                    end: self.argument.span.end,
                                    context: self.span.context,
                                },
                            });
                        }
                    }
                    _ => todo!(),
                },
                Value::Reference(_ref_cell) => todo!(),
                Value::Void => todo!(),
            },
            _ => todo!(),
        }

        Ok(receiver)
    }
}
