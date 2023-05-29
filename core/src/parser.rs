// a parser takens in a stream of tokens and turns them into a
// intermediate representation in the form of an abstract syntax tree

use crate::error::BessyError;
use crate::expr::Expr;
use crate::lexer::{Token, TokenType};
use crate::stmt::Stmt;
use std::iter::Peekable;

macro_rules! next_eq {
    ( $parser: ident, $( $x: expr ), *) => {
        {
            $parser.cursor.next_if(|t| $(t.kind == $x) || *)
        }
    };
}

pub struct Parser<T: Iterator<Item = Token>> {
    cursor: Peekable<T>,
    statements: Vec<Stmt>,
}

impl<T: Iterator<Item = Token>> Parser<T> {
    pub fn new(tokens: T) -> Parser<T> {
        Parser {
            statements: Vec::with_capacity(tokens.size_hint().0),
            cursor: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, BessyError> {
        while self.cursor.peek().is_some() {
            let stmt = self.declaration()?;
            self.statements.push(stmt);
        }
        Ok(self.statements.clone())
    }

    fn peek_check(&mut self, expected: TokenType) -> bool {
        self.cursor
            .peek()
            .map_or(false, |token| token.kind == expected)
    }

    fn next_eq(&mut self, expected: TokenType) -> bool {
        self.cursor
            .next_if(|token| token.kind == expected)
            .is_some()
    }

    fn consume_if(
        &mut self,
        predicate: impl FnOnce(&Token) -> bool,
        error_msg: &str,
    ) -> Result<Token, BessyError> {
        self.cursor.next_if(predicate).ok_or(self.error(error_msg))
    }

    fn consume(
        &mut self,
        expected: TokenType,
        error_msg: &str,
    ) -> Result<Token, BessyError> {
        self.consume_if(|t| t.kind == expected, error_msg)
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
        self.unary()
    }

    // TODO: Make return type Result<!, BessyError>
    // once the feature is stabilized
    fn error(&mut self, message: &str) -> BessyError {
        BessyError::Unexpected {
            msg: message.into(),
            span: self.cursor.peek().map(|t| t.span),
        }
    }

    fn unary(&mut self) -> Result<Expr, BessyError> {
        if let Some(oper) = next_eq!(self, TokenType::Bang, TokenType::Minus) {
            let right = Box::new(self.unary()?);
            Ok(Expr::Unary { oper, right })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, BessyError> {
        let mut expr = self.primary()?;
        loop {
            if self.next_eq(TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, BessyError> {
        let mut args = Vec::with_capacity(255);
        if !self.peek_check(TokenType::RightParen) {
            args.push(self.expression()?);
            while self.next_eq(TokenType::Comma) {
                if args.len() > 255 {
                    return Err(
                        self.error("Can't have more than 255 arguments.")
                    );
                }
                args.push(self.expression()?);
            }
        }

        let paren =
            self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            args,
        })
    }

    fn primary(&mut self) -> Result<Expr, BessyError> {
        if let Some(expr) = self.cursor.next() {
            match expr.kind {
                TokenType::Nil => Ok(Expr::Nil),
                TokenType::Boolean(value) => Ok(Expr::Boolean(value)),
                TokenType::Number(num) => Ok(Expr::Number(num)),
                TokenType::StringLiteral(lexeme) => Ok(Expr::String(lexeme)),
                TokenType::LeftParen => {
                    let expr = self.expression()?;
                    self.consume(
                        TokenType::RightParen,
                        "Expect ')' after expression.",
                    )?;
                    Ok(Expr::Group(Box::new(expr)))
                }
                TokenType::Identifier(_) => Ok(Expr::Variable(expr)),
                _ => Err(self.error("Expect expression.")),
            }
        } else {
            Err(self.error("Expect expression but reached end of file."))
        }
    }
}
