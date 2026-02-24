use pest::iterators::Pair;

use crate::{
    environment::Environment,
    parser::Rule,
    statement::{expression::Expression, Instruction, Value},
    test_error::TestError,
    TescOptions,
};

#[derive(Debug, Clone)]
pub struct Input(Vec<Expression>);

impl Instruction for Input {
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
                panic!("Input: Arg was not of type string");
            }
        }

        evaluated_args.last_mut().unwrap().push('\n');
        env.send_to_child(evaluated_args.join(" ").as_bytes());

        Ok(Value::Void)
    }
}
