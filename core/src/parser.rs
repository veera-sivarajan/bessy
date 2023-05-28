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
            if $($parser.check($x)) || * {
                $parser.cursor.next()
            } else {
                None
            }
        }
    };
}

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

    fn check(&mut self, expected: TokenType) -> bool {
        if let Some(token) = self.cursor.peek() {
            return token.kind == expected;
        }
        false
    }

    fn declaration(&mut self) -> Result<Stmt, BessyError> {
        if next_eq!(self, TokenType::Var).is_some() {
            self.variable_declaration()
        } else if next_eq!(self, TokenType::Fun).is_some() {
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
            .ok_or(self.error(error_msg))
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
        if next_eq!(self, TokenType::Equal).is_some() {
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

    fn unary(&mut self) -> Result<Expr, BessyError> {
        if let Some(oper) = next_eq!(self, TokenType::Bang, TokenType::Minus) {
            let right = Box::new(self.unary()?);
            Ok(Expr::Unary {
                oper,
                right,
            })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, BessyError> {
        let mut expr = self.primary()?;
        loop {
            if next_eq!(self, TokenType::LeftParen).is_some() {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, BessyError> {
        let mut args = Vec::with_capacity(255);
        if !self.check(TokenType::RightParen) {
            args.push(self.expression()?);
            while next_eq!(self, TokenType::Comma).is_some() {
                if args.len() > 255 {
                    return Err(self.error("Can't have more than 255 arguments."));
                }
                args.push(self.expression()?);
            }
        }
        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            args,
        })
    }

    fn primary(&mut self) -> Result<Expr, BessyError> {
        if next_eq!(self, TokenType::Nil).is_some() {
            Ok(Expr::Nil)
        } else if next_eq!(self, TokenType::False).is_some() {
            Ok(Expr::Boolean(false))
        } else if next_eq!(self, TokenType::True).is_some() {
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
        } else if next_eq!(self, TokenType::LeftParen).is_some() {
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
