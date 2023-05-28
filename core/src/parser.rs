// a parser takens in a stream of tokens and turns them into a
// intermediate representation in the form of an abstract syntax tree

use crate::expr::Expr;
use crate::lexer::Token;
use crate::stmt::Stmt;
use crate::error::BessyError;
use std::iter::Peekable;

pub struct Parser<T>
where
    T: Iterator<Item = Token>,
{
    cursor: Peekable<T>,
    statements: Vec<Stmt>,
}

impl<T: Iterator<Item = Token>> Parser<T> {
    pub fn new(tokens: T) -> Parser<T> {
        Parser {
            cursor: tokens.peekable(),
            statements: vec![],
        }
    }

    pub fn parse() -> Result<Vec<Stmt>, BessyError> {
        todo!()
    }
}
