use ariadne::{sources, Color, Label, Report, ReportKind};
use chumsky::Parser;

use crate::{environment::Environment, statement::Instruction};

mod lexer;
mod parser;

mod environment;

mod statement;

mod test_error;

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

pub fn run(path: String) {
    let src = std::fs::read_to_string(&path).unwrap();
    let (tokens, errs) = lexer::lexer().parse(&src).into_output_errors();

    if let Some(tokens) = &tokens {
        let (ast, parse_errs) = parser::parse(tokens.as_slice(), src.len()).into_output_errors();

        errs.into_iter()
            .map(|e| e.map_token(|c| c.to_string()))
            .chain(
                parse_errs
                    .into_iter()
                    .map(|e| e.map_token(|tok| format!("{:?}", tok))),
            )
            .for_each(|e| {
                Report::build(ReportKind::Error, (path.clone(), e.span().into_range()))
                    .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                    .with_message(e.to_string())
                    .with_label(
                        Label::new((path.clone(), e.span().into_range()))
                            .with_message(e.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .with_labels(e.contexts().map(|(label, span)| {
                        Label::new((path.clone(), span.into_range()))
                            .with_message(format!("while parsing this {label}"))
                            .with_color(Color::Yellow)
                    }))
                    .finish()
                    .print(sources([(path.clone(), src.clone())]))
                    .unwrap()
            });

        if let Some((ast, _)) = ast {
            let opts = TescOptions::new();
            let mut env = Environment::new();
            match ast.eval(&opts, &mut env) {
                Ok(_) => println!("All tests passed!"),
                Err(e) => eprintln!("{}", e),
            }
        }
    }
}
