use clap::Parser;
use magnetite as mag;

fn main() {
    let args = mag::Args::parse();

    if let Err(e) = mag::run(args) {
        eprintln!("{}", e);
    };
}
