use std::{
    error::Error,
    fs,
    io::{self, Write},
    path::PathBuf,
};

use crate::Args;

pub struct Interpreter {
    pub file_path: Option<PathBuf>,
    had_error: bool,
}

impl Interpreter {
    pub fn new(args: Args) -> Self {
        let file_path = match &args.file_name {
            None => None,
            Some(file_name) => Some(PathBuf::from(file_name)),
        };

        Interpreter {
            file_path,
            had_error: false,
        }
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
        let input = io::stdin();
        let mut output = io::stdout();
        let mut line = String::new();
        let mut bytes_read: usize;

        loop {
            line.clear();

            print!("> ");
            let _ = output.flush()?;

            bytes_read = input.read_line(&mut line)?;
            if bytes_read == 0 {
                println!();
                break;
            }

            self.run_source(&line);
            self.had_error = false;
        }

        Ok(())
    }

    pub fn run_file(&mut self) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(match &self.file_path {
            None => return Err("no file provided".into()),
            Some(file_path) => file_path,
        })?;
        self.run_source(&content);

        if self.had_error {
            return Err("unexpected error in source code".into());
        }

        Ok(())
    }

    fn run_source(&mut self, source: &str) {
        println!("{}", source.trim());
    }

    fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message)
    }

    fn report(&mut self, line: u32, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
        self.had_error = false;
    }
}
