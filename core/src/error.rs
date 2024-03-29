use crate::lexer::{Position, Span};
use std::fmt;

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "line {}, column {}",
            self.line_number, self.column_number
        )
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "line {}, columns {} - {}",
            self.start.line_number,
            self.start.column_number,
            self.end.column_number
        )
    }
}

#[derive(Debug)]
pub enum BessyError {
    UnterminatedString(Span),
    Unexpected { msg: Box<str>, span: Option<Span> },
}

impl fmt::Display for BessyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BessyError::*;
        match self {
            UnterminatedString(span) => {
                write!(
                    f,
                    "Syntax Error: Unterminated string literal at {span}."
                )
            }
            Unexpected { msg, span } => {
                if let Some(span) = span {
                    write!(f, "Parse error: {msg} at {span}.")
                } else {
                    write!(f, "Parse error: {msg} at end of file.")
                }
            }
        }
    }
}

impl std::error::Error for BessyError {}
