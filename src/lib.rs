pub fn run(args: Args) {
    println!("{}", args.file_name);
}

/// An interpreter for the Iron programming language
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File name to read
    pub file_name: String,
}
