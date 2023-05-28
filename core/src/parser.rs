// a parser takens in a stream of tokens and turns them into a
// intermediate representation in the form of an abstract syntax tree


use std::iter::Peekable;
use crate::lexer::Token;
use crate::stmt::Stmt;
use crate::expr::Expr;

pub struct Parser<'src> {
    cursor: Peekable<std::slice::Iter<'src, Token>>,
    statements: Vec<Stmt>,
}

impl<'src> Parser<'src> {
    pub fn new(tokens: &'src [Token]) -> Parser<'src> {
        Parser {
            cursor: tokens.iter().peekable(),
            statements: vec![],
        }
    }
}
