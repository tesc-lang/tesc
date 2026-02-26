use pest::Parser;

use crate::{module::Module, statement::Instruction};

#[derive(pest_derive::Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct TescParser;

pub fn parse(file_name: String) -> Result<Module, pest::error::Error<Rule>> {
    let source = std::fs::read_to_string(&file_name).unwrap();

    let mut pairs = TescParser::parse(Rule::module, &source)?;
    Ok(Module::parse(pairs.next().unwrap()))
}
