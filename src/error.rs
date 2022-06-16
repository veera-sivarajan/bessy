use std::error::Error;
use std::fmt;

macro_rules! lex_error {
    () => {
        Err(BessyError::Lex)
    };
}

#[derive(Debug)]
pub enum BessyError {
    Lex,
}

impl Error for BessyError {}

impl fmt::Display for BessyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BessyError::Lex => write!(f, "Scan error!"),
        }
    }
}
