use std::fmt;

#[derive(Debug)]
pub struct Index {
    row: u16,
    column: u16,
}

impl From<(u16, u16)> for Index {
    fn from(index: (u16, u16)) -> Index {
        Index {
            row: index.0,
            column: index.1,
        }
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at line {}, column {}", self.row, self.column)
    }
}

#[derive(Debug)]
pub enum BessyError {
    UnterminatedString(Index),
    Unexpected {
        msg: Box<str>,
        span: Index,
    },
}

impl fmt::Display for BessyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BessyError::*;
        match self {
            UnterminatedString(span) => write!(f, "Lex error: Unterminated String Literal at {span}."),
            Unexpected{msg, span} => write!(f, "Parse error: {msg} at {span}."),
        }
    }
}

impl std::error::Error for BessyError {}


