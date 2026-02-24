use pest::iterators::Pair;

use crate::{
    environment::Environment,
    parser::Rule,
    statement::{Expression, Instruction, Value},
    test_error::TestError,
    TescOptions,
};

#[derive(Debug, Clone)]
pub struct Output(Vec<Expression>);

impl Instruction for Output {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut inner = pair.into_inner();
        let mut arguments = Vec::new();
        if let Some(argument_pairs) = inner.next() {
            for pair in argument_pairs.into_inner() {
                arguments.push(Expression::parse(pair));
            }
        }
        Self(arguments)
    }

    fn eval(&self, opts: &TescOptions, env: &mut Environment) -> Result<Value, TestError> {
        let mut evaluated_args = Vec::new();
        for arg in &self.0 {
            let arg = arg.eval(opts, env)?;
            if let Value::String(arg) = arg {
                evaluated_args.push(arg);
            } else {
                panic!("Output: Arg was not of type string");
            }
        }

        let mut buf = String::new();
        env.recieve_from_child(&mut buf);
        if evaluated_args.join(" ") + "\n" == buf {
            Ok(Value::Void)
        } else {
            Err(TestError::OutputMissmatch {
                test: env.test.clone().unwrap(),
                actual: buf.clone().strip_suffix("\n").unwrap().to_string(),
                expected: evaluated_args.join(" "),
            })
        }
    }
}
