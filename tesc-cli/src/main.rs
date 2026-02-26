use clap::{Parser, command};

use tesc_core::{TescOptions, environment::Environment, parser::parse, test_error::TestError};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    source: String,
}

fn main() {
    let args = Args::parse();
    let module = parse(args.source);
    let result = match module {
        Ok(v) => v.run(&TescOptions, &mut Environment::default()),
        Err(e) => panic!("{e}"),
    };
    if let Err(e) = result {
        match e {
            TestError::Multiple(test_errors) => {
                for error in test_errors {
                    eprintln!("{error}")
                }
            }
            _ => unreachable!(),
        }
    } else {
        println!("All tests passed");
    }
}
