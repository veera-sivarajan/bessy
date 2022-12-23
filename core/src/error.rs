use std::error::Error;
use std::{fmt, io};

macro_rules! lex_error {
    ($message:expr, $line:expr) => {
        Err(BessyError::Lexer(String::from($message), $line))
    };
}

macro_rules! parse_error {
    ($message:expr, $line:expr) => {
        Err(BessyError::Parser(String::from($message), $line))
    };
}

macro_rules! runtime_error {
    ($message:expr, $line:expr) => {
        Err(BessyError::Runtime(String::from($message), $line))
    };
}

#[derive(Debug)]
pub enum BessyError {
    Lexer(String, u16), // Error message and line number
    Parser(String, u16),
    Runtime(String, u16),
    IoError(io::Error),
}

impl Error for BessyError {}

impl fmt::Display for BessyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BessyError::Lexer(msg, line) => write!(f, "[line {}] Lex error: {}", line, msg),
            BessyError::Parser(msg, line) => write!(f, "[line {}] Parse error: {}", line, msg),
            BessyError::Runtime(msg, line) => write!(f, "[line {}] Runtime error: {}", line, msg),
            BessyError::IoError(err) => write!(f, "Cannot write to stdout. {}", err), 
        }
    }
}

impl From<io::Error> for BessyError {
    fn from(error: io::Error) -> Self {
        BessyError::IoError(error)
    }
}
