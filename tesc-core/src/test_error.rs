use std::fmt::{self, Display, Formatter};

use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum TestError {
    OutputMissmatch {
        test: String,
        actual: String,
        expected: String,
    },
    Multiple(Vec<TestError>),
}

impl Display for TestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TestError::OutputMissmatch {
                test,
                actual,
                expected,
            } => {
                writeln!(f, "test `{test}` failed: Output missmatch")?;
                writeln!(f, "Recieved `{actual}`")?;
                writeln!(f, "Expected `{expected}`")?;
            }
            TestError::Multiple(test_errors) => {
                for error in test_errors {
                    writeln!(f, "{error}")?;
                }
            }
        }
        Ok(())
    }
}
