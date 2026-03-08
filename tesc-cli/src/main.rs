use clap::Parser;
use tesc_core::TescArgs;

fn main() {
    let args = TescArgs::parse();
    tesc_core::run(args);
}
