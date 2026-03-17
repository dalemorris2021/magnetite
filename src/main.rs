use clap::Parser;
use magnetite as mag;

fn main() {
    let args = mag::Args::parse();

    mag::run(args);
}
