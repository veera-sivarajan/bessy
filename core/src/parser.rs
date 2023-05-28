// a parser takens in a stream of tokens and turns them into a
// intermediate representation in the form of an abstract syntax tree

use crate::error::BessyError;
use crate::expr::Expr;
use crate::lexer::{Token, TokenType};
use crate::stmt::Stmt;
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, BessyError> {
        while self.cursor.peek().is_some() {
            let stmt = self.declaration()?;
            self.statements.push(stmt);
        }
        Ok(self.statements.clone())
    }

    fn next_eq(&mut self, expected: TokenType) -> bool {
        if let Some(token) = self.cursor.peek() {
            if token.kind == expected {
                self.cursor.next();
                return true;
            }
        }
        false
    }

    fn declaration(&mut self) -> Result<Stmt, BessyError> {
        if self.next_eq(TokenType::Var) {
            self.variable_declaration()
        } else if self.next_eq(TokenType::Fun) {
            todo!();
        } else {
            todo!();
        }
    }

    fn consume_if(
        &mut self,
        predicate: impl FnOnce(&Token) -> bool,
        error_msg: &str,
    ) -> Result<Token, BessyError> {
        self.cursor
            .next_if(predicate)
            .ok_or(BessyError::Unexpected {
                msg: error_msg.into(),
                span: self.cursor.peek().map(|t| t.index),
            })
    }

    fn consume(
        &mut self,
        expected: TokenType,
        error_msg: &str,
    ) -> Result<Token, BessyError> {
        self.consume_if(|t| t.kind == expected, error_msg)
    }

    fn variable_declaration(&mut self) -> Result<Stmt, BessyError> {
        let name = self.consume_if(
            |token| token.is_identifier(),
            "Expect variable name.",
        )?;
        if self.next_eq(TokenType::Equal) {
            let init = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect semicolon.")?;
            Ok(Stmt::Var {
                name,
                init: Some(init),
            })
        } else {
            self.consume(TokenType::Semicolon, "Expect semicolon.")?;
            Ok(Stmt::Var { name, init: None })
        }
    }

    fn expression(&mut self) -> Result<Expr, BessyError> {
        self.primary()
    }

    // TODO: Make return type Result<!, BessyError>
    // once the feature is stabilized
    fn error(&mut self, message: &str) -> BessyError {
        BessyError::Unexpected {
            msg: message.into(),
            span: self.cursor.peek().map(|t| t.index),
        }
    }

    fn primary(&mut self) -> Result<Expr, BessyError> {
        if self.next_eq(TokenType::Nil) {
            Ok(Expr::Nil)
        } else if self.next_eq(TokenType::False) {
            Ok(Expr::Boolean(false))
        } else if self.next_eq(TokenType::True) {
            Ok(Expr::Boolean(true))
        } else if let Some(Token {
            index: _,
            kind: TokenType::Number(num),
            line: _,
        }) = self.cursor.next_if(|t| t.is_number())
        {
            Ok(Expr::Number(num))
        } else if let Some(Token {
            index: _,
            kind: TokenType::StrLit(literal),
            line: _,
        }) = self.cursor.next_if(|t| t.is_string())
        {
            Ok(Expr::String(literal))
        } else if self.next_eq(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            Ok(Expr::Group(Box::new(expr)))
        } else if let Some(token) = self.cursor.next_if(|t| t.is_identifier()) {
            Ok(Expr::Variable(token))
        } else {
            Err(self.error("Expect expression."))
        }
    }
}
