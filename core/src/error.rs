use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Index {
    pub row: u16,
    pub column: u16,
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, column {}", self.row, self.column)
    }
}

#[derive(Debug)]
pub enum BessyError {
    UnterminatedString(Index),
    Unexpected { msg: Box<str>, span: Option<Index> },
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
