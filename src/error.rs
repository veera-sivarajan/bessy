use std::error::Error;
use std::fmt;

macro_rules! lex_error {
    ($message:expr, $line:expr) => {
        Err(BessyError::Lexer($message, $line))
    };
}

macro_rules! parse_error {
    ($message:expr) => {
        Err(BessyError::Parser($message))
    };
}

#[derive(Debug)]
pub enum BessyError {
    Lexer(&'static str, u16), // static str because it will never be mutated
    Parser(&'static str),
}

impl Error for BessyError {}

impl fmt::Display for BessyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BessyError::Lexer(msg, line) => write!(f, "[line {}] Lex error: {}",  line, msg),
            BessyError::Parser(msg) => write!(f, "Parser error: {}",  msg),
        }
    }
}
