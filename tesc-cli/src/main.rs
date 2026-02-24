use clap::{Parser, command};

use tesc_core::{TescOptions, parser::parse};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    source: String,
}

fn main() {
    let args = Args::parse();
    let module = parse(args.source);
    let result = match module {
        Ok(v) => v.eval(&TescOptions),
        Err(e) => panic!("{e}"),
    };
    if let Err(errors) = result {
        for e in errors {
            eprintln!("{e}");
        }
    } else {
        println!("All tests passed");
    }
}
