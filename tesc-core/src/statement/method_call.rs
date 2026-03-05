use std::io::{BufRead, Write};

use chumsky::Parser;

use crate::{
    environment::Environment,
    statement::{expression::Expression, ident::Ident, Instruction, ParserExtra, Spanned, Value},
    test_error::TestError,
    TescOptions,
};

#[derive(Clone, Debug)]
pub struct MethodCall {
    receiver: Expression,
    method: Ident,
    argument: Expression,
}

impl MethodCall {
    pub fn parser<'tokens, 'src: 'tokens, I>(
        expr: impl Parser<'tokens, I, Spanned<Expression>, ParserExtra<'tokens, 'src>> + Clone + 'tokens,
    ) -> impl Parser<'tokens, I, Spanned<Self>, ParserExtra<'tokens, 'src>> + Clone
    where
        Self: std::marker::Sized,
        I: chumsky::input::ValueInput<
            'tokens,
            Token = crate::lexer::Token<'src>,
            Span = chumsky::prelude::SimpleSpan,
        >,
    {
        expr.clone()
            .then(Ident::parser())
            .then(expr)
            .map(|(((receiver, _), (method, _)), (argument, _))| MethodCall {
                receiver,
                method,
                argument,
            })
            .map_with(|expr, e| (expr, e.span()))
    }

    pub fn eval(&self, _opts: &TescOptions, _env: &mut Environment) -> Result<Value, TestError> {
        let receiver = self.receiver.eval(_opts, _env)?;
        let argument = match self.argument.eval(_opts, _env) {
            Ok(v) => match v {
                Value::String(s) => s + "\n",
                Value::Process(_ref_cell) => todo!(),
                Value::Reference(_ref_cell) => todo!(),
                Value::Void => todo!(),
            },
            Err(_) => todo!(),
        };

        match receiver {
            Value::Reference(r) => match &mut *r.borrow_mut() {
                Value::String(_) => todo!(),
                Value::Process(ref mut child) => match self.method.0.as_str() {
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
                            return Err(TestError::OutputMissmatch {
                                actual: buf.clone(),
                                expected: argument,
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

        Ok(Value::Void)
    }
}
