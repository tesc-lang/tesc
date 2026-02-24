mod environment;
mod module;
pub mod parser;
mod statement;
pub mod test_error;
mod r#type;

pub struct TescOptions;

impl Default for TescOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl TescOptions {
    pub fn new() -> Self {
        Self
    }
}
