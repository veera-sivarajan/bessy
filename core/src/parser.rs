// a parser takens in a stream of tokens and turns them into a
// intermediate representation in the form of an abstract syntax tree
use crate::error::BessyError;
use crate::expr::Expr;
use crate::lexer::{Span, Token, TokenType};
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

    fn consume_identifier(
        &mut self,
        error_msg: &str,
    ) -> Result<Token, BessyError> {
        self.cursor
            .next_if(|token| token.is_identifier())
            .ok_or(self.error(error_msg))
    }

    fn consume(
        &mut self,
        expected: TokenType,
        error_msg: &str,
    ) -> Result<Token, BessyError> {
        self.cursor
            .next_if(|t| t.kind == expected)
            .ok_or(self.error(error_msg))
    }

    // TODO: Make return type Result<!, BessyError>
    // once the feature is stabilized
    fn error(&mut self, message: &str) -> BessyError {
        BessyError::Unexpected {
            msg: message.into(),
            span: self.cursor.peek().map(|t| t.span),
        }
    }

    fn error_with_span(&mut self, message: &str, span: Span) -> BessyError {
        BessyError::Unexpected {
            msg: message.into(),
            span: Some(span),
        }
    }

    fn declaration(&mut self) -> Result<Stmt, BessyError> {
        if self.next_eq(TokenType::Var) {
            self.variable_declaration()
        } else if self.next_eq(TokenType::Fun) {
            self.function()
        } else {
            self.statement()
        }
    }

    fn variable_declaration(&mut self) -> Result<Stmt, BessyError> {
        let name = self.consume_identifier("Expect variable name.")?;
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

    fn function(&mut self) -> Result<Stmt, BessyError> {
        let name = self.consume_identifier("Expect function name.")?;
        self.consume(TokenType::LeftParen, "Expect '(' after function name.")?;
        // parse all parameters
        let mut params: Vec<Token> = vec![];
        if !self.peek_check(TokenType::RightParen) {
            params.push(self.consume_identifier("Expect parameter name.")?);
            while self.next_eq(TokenType::Comma) {
                if params.len() < 255 {
                    params.push(
                        self.consume_identifier("Expect parameter name.")?,
                    );
                } else {
                    return Err(
                        self.error("Can't have more than 255 paramters.")
                    );
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(TokenType::LeftBrace, "Expect '{' before function body")?;
        let body = self.block_stmt()?;
        Ok(Stmt::Function { name, params, body })
    }

    fn statement(&mut self) -> Result<Stmt, BessyError> {
        if self.next_eq(TokenType::Print) {
            self.print_stmt()
        } else if self.next_eq(TokenType::LeftBrace) {
            let stmts = self.block_stmt()?;
            Ok(Stmt::Block(stmts))
        } else if self.next_eq(TokenType::If) {
            self.if_stmt()
        } else if self.next_eq(TokenType::While) {
            self.while_stmt()
        } else if self.next_eq(TokenType::For) {
            self.for_stmt()
        } else if let Some(token) = next_eq!(self, TokenType::Return) {
            self.return_stmt(token)
        } else {
            self.expression_stmt()
        }
    }

    fn return_stmt(&mut self, keyword: Token) -> Result<Stmt, BessyError> {
        let mut value = None;
        if !self.peek_check(TokenType::Semicolon) {
            value = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return { keyword, value })
    }

    fn for_stmt(&mut self) -> Result<Stmt, BessyError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let init;
        if self.next_eq(TokenType::Semicolon) {
            init = None;
        } else if self.next_eq(TokenType::Var) {
            init = Some(self.variable_declaration()?);
        } else {
            init = Some(self.expression_stmt()?);
        }

        let mut condition = None;
        if !self.peek_check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment = None;
        if !self.peek_check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;
        let mut body = self.statement()?;
        if let Some(increment_expression) = increment {
            let increment_stmt = Stmt::Expression(increment_expression);
            body = Stmt::Block(vec![body, increment_stmt]);
        }
        body = Stmt::While {
            condition: condition.unwrap_or(Expr::Boolean(true)),
            body: Box::new(body),
        };
        if let Some(init_statement) = init {
            body = Stmt::Block(vec![init_statement, body]);
        }
        Ok(body)
    }

    fn while_stmt(&mut self) -> Result<Stmt, BessyError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;
        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn if_stmt(&mut self) -> Result<Stmt, BessyError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let then_branch = self.statement()?;
        let mut elze = None;
        if self.next_eq(TokenType::Else) {
            elze = Some(Box::new(self.statement()?));
        }
        Ok(Stmt::If {
            condition,
            then: Box::new(then_branch),
            elze,
        })
    }

    fn block_stmt(&mut self) -> Result<Vec<Stmt>, BessyError> {
        let mut stmts = Vec::new();
        while !self.peek_check(TokenType::RightParen) {
            stmts.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(stmts)
    }

    fn print_stmt(&mut self) -> Result<Stmt, BessyError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect semicolon.")?;
        Ok(Stmt::Print(expr))
    }

    fn expression_stmt(&mut self) -> Result<Stmt, BessyError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect semicolon.")?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, BessyError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, BessyError> {
        let expr = self.equality()?;
        if let Some(equals) = next_eq!(self, TokenType::Equal) {
            let value = self.assignment()?;
            match expr {
                Expr::Variable(variable_name) => Ok(Expr::Assign {
                    name: variable_name,
                    value: Box::new(value),
                }),
                _ => Err(self.error_with_span(
                    "Invalid assignment target.",
                    equals.span,
                )),
            }
        } else {
            Ok(expr)
        }
    }

    fn equality(&mut self) -> Result<Expr, BessyError> {
        let mut expr = self.comparison()?;
        while let Some(oper) =
            next_eq!(self, TokenType::BangEqual, TokenType::EqualEqual)
        {
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                oper,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, BessyError> {
        let mut expr = self.term()?;
        while let Some(oper) = next_eq!(
            self,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ) {
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                oper,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, BessyError> {
        let mut expr = self.factor()?;
        while let Some(oper) = next_eq!(self, TokenType::Minus, TokenType::Plus)
        {
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                oper,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, BessyError> {
        let mut expr = self.unary()?;
        while let Some(oper) = next_eq!(
            self,
            TokenType::Slash,
            TokenType::Star,
            TokenType::Percent
        ) {
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                oper,
                right: Box::new(right),
            };
        }
        Ok(expr)
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

    // FIXME: Try not move .next() when expr.kind is not of
    // the expected expression type
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
                other => Err(self.error_with_span(
                    format!(
                        "Expected a primary expression but found `{other}`"
                    )
                    .as_str(),
                    expr.span,
                )),
            }
        } else {
            Err(self.error("Expected a primary expression"))
        }
    }
}
