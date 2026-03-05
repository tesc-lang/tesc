use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    src: String,
}

fn main() {
    let args = Args::parse();
    tesc_core::run(args.src);
}
