use std::error::Error;
use std::fmt;

macro_rules! lex_error {
    ($message:expr) => {
        Err(BessyError::Lex($message))
    };
}

#[derive(Debug)]
pub enum BessyError {
    Lex(&'static str), // static str because this err msg will never be mutated
}

impl Error for BessyError {}

impl fmt::Display for BessyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BessyError::Lex(msg) => write!(f, "Lex error: {}",  msg),
        }
    }
}
