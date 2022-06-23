use std::error::Error;
use std::fmt;

macro_rules! lex_error {
    ($message:expr) => {
        Err(BessyError::Lexer($message))
    };
}

macro_rules! parse_error {
    ($message:expr) => {
        Err(BessyError::Parser($message))
    };
}

#[derive(Debug)]
pub enum BessyError {
    Lexer(&'static str), // static str because this err msg will never be mutated
    Parser(&'static str),
}

impl Error for BessyError {}

impl fmt::Display for BessyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BessyError::Lexer(msg) => write!(f, "Lex error: {}",  msg),
            BessyError::Parser(msg) => write!(f, "Parser error: {}",  msg),
        }
    }
}
