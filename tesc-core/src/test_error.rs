use std::fmt::{self, Display, Formatter};

use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum TestError {
    OutputMissmatch { actual: String, expected: String },
    Multiple(Vec<TestError>),
    UndefinedIdentifier(String),
}

impl Display for TestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TestError::OutputMissmatch { actual, expected } => {
                writeln!(f, "Test failed: Output missmatch")?;
                writeln!(f, "Recieved `{actual}`")?;
                writeln!(f, "Expected `{expected}`")?;
            }
            TestError::Multiple(test_errors) => {
                for error in test_errors {
                    writeln!(f, "{error}")?;
                }
            }
            TestError::UndefinedIdentifier(ident) => {
                writeln!(f, "{ident}")?;
            }
        }
        Ok(())
    }
}
