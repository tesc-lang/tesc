use std::cmp::Ordering;

use ariadne::{sources, Color, Label, Report, ReportKind};
use clap::Parser as ClapParser;

use chumsky::{error::Rich, span::SimpleSpan, Parser};

use crate::{
    environment::{RunTimeEnv, TypeCheckEnv},
    error::{RunTimeErr, RunTimeErrKind, TypeCheckErr, TypeCheckErrKind},
    lexer::Token,
    statement::Instruction,
};

mod lexer;
mod parser;

mod environment;
mod types;

mod statement;

mod error;

#[derive(PartialEq)]
struct TescErr<'a> {
    span: SimpleSpan,
    kind: TescErrKind<'a>,
}

impl<'a> std::fmt::Display for TescErr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TescErrKind::Lex(rich) => write!(f, "{rich}"),
            TescErrKind::Parse(rich) => write!(f, "{rich}"),
            TescErrKind::TypeCheck(type_check_err) => write!(f, "{type_check_err}"),
            TescErrKind::RunTime(run_time_err) => write!(f, "{run_time_err}"),
        }
    }
}

impl<'a> std::cmp::PartialOrd for TescErr<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.span.partial_cmp(&other.span)
    }
}

impl<'a> TescErr<'a> {
    fn reason(&self) -> String {
        match &self.kind {
            TescErrKind::Lex(rich) => rich.reason().to_string(),
            TescErrKind::Parse(rich) => rich.reason().to_string(),
            TescErrKind::TypeCheck(type_check_err) => type_check_err.reason(),
            TescErrKind::RunTime(run_time_err) => run_time_err.reason(),
        }
    }
}

#[derive(PartialEq)]
enum TescErrKind<'a> {
    Lex(Rich<'a, char>),
    Parse(Rich<'a, Token<'a>>),
    TypeCheck(TypeCheckErr),
    RunTime(RunTimeErr),
}

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
pub struct TescArgs {
    path: String,
}

pub fn run(args: TescArgs) {
    let src = std::fs::read_to_string(&args.path).unwrap();
    let (tokens, errs) = lexer::lexer().parse(&src).into_output_errors();
    let mut errs: Vec<TescErr> = errs
        .into_iter()
        .map(|e| TescErr {
            span: *e.span(),
            kind: TescErrKind::Lex(e),
        })
        .collect();

    if let Some(tokens) = &tokens {
        let (ast, parse_errs) = parser::parse(tokens.as_slice(), src.len()).into_output_errors();
        errs.append(
            &mut parse_errs
                .into_iter()
                .map(|e| TescErr {
                    span: *e.span(),
                    kind: TescErrKind::Parse(e),
                })
                .collect(),
        );

        if let Some(ast) = ast {
            let mut typecheck_env = TypeCheckEnv::new();
            if let Err(TypeCheckErr {
                kind: TypeCheckErrKind::Multiple(e),
                ..
            }) = ast.check(&args, &mut typecheck_env)
            {
                errs.append(
                    &mut e
                        .into_iter()
                        .map(|e| TescErr {
                            kind: TescErrKind::TypeCheck(e.clone()),
                            span: e.span,
                        })
                        .collect(),
                );
            } else {
                let mut runtime_env = RunTimeEnv::new();
                match ast.eval(&args, &mut runtime_env) {
                    Ok(_) => println!("All tests passed!"),
                    Err(RunTimeErr {
                        kind: RunTimeErrKind::Multiple(e),
                        ..
                    }) => print_errors(
                        &args,
                        &src,
                        e.into_iter()
                            .map(|e| TescErr {
                                kind: TescErrKind::RunTime(e.clone()),
                                span: e.span,
                            })
                            .collect(),
                    ),
                    _ => unreachable!(),
                }
            }
        }
    }
    print_errors(&args, &src, errs);
}

fn print_errors(args: &TescArgs, src: &str, mut errs: Vec<TescErr>) {
    errs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    errs.into_iter().for_each(|e| {
        Report::build(ReportKind::Error, (args.path.clone(), e.span.into_range()))
            .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
            .with_message(e.to_string())
            .with_label(
                Label::new((args.path.clone(), e.span.into_range()))
                    .with_message(e.reason())
                    .with_color(Color::Red),
            )
            // .with_labels(e.contexts().map(|(label, span)| {
            //     Label::new((args.path.clone(), span.into_range()))
            //         .with_message(format!("while parsing this {label}"))
            //         .with_color(Color::Yellow)
            // }))
            .finish()
            .print(sources([(args.path.clone(), src)]))
            .unwrap()
    });
}
