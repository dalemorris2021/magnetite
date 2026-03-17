mod interpreter;
mod scanner;

use std::error::Error;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut interpreter = interpreter::Interpreter::new(args);

    match interpreter.file_path {
        None => Ok(interpreter.run_prompt()?),
        Some(_) => Ok(interpreter.run_file()?),
    }
}

/// An interpreter for the Iron programming language
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File name to read
    pub file_name: Option<String>,
}

pub enum Literal {}

pub enum TokenType {
    // Single character
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two characters
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literal
    Identifier,
    String,
    Number,

    // Keyword
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: u32,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        todo!()
    }
}
