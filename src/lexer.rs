use crate::error::BessyError;
use crate::token::{Token, TokenType};

// Takes a heap allocated string and generates tokens on demand.

type Result<T> = std::result::Result<T, BessyError>;

pub struct Lexer<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: u16,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<u8> {
        let c = self.peek();
        self.current += 1;
        c
    }

    fn peek(&self) -> Option<u8> {
        if !self.is_at_end() {
            Some(self.source.as_bytes()[self.current])
        } else {
            None
        }
    }

    fn double_peek(&self) -> Option<u8> {
        if (self.current + 1) < self.source.len() {
            Some(self.source.as_bytes()[self.current + 1])
        } else {
            None
        }
    }

    fn next_eq(&mut self, expected: u8) -> bool {
        self.peek().map_or(false, |c| {
            self.advance();
            c == expected
        })
    }

    // fn peek_eq(&self, check_fn: fn(u8) -> bool) -> bool {
    fn peek_eq(&self, expected: u8) -> bool {
        self.peek().map_or(false, |c| c == expected) 
    }

    fn peek_ne(&self, expected: u8) -> bool {
        self.peek().map_or(false, |c| c != expected) 
    }
    
    fn make_token(&self, kind: TokenType<'a>) -> Result<Token> {
        Ok(Token::new(kind, self.line))
    }

    fn skip_needless(&mut self) {
       while let Some(c) = self.peek() {
           match c {
               b' ' | b'\r' | b'\t' => {
                   self.advance();
               }
               b'\n' => {
                   self.advance();
                   self.line += 1;
               }
               b'/' if self.double_peek().map_or(false, |c| c == b'/') => {
                   while self.peek_ne(b'\n') {
                       self.advance();
                   }
               }
               _ => break,
           }
       }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_needless();
        self.start = self.current;

        let a = self.advance();
        if let Some(c) = a {
            match c {
                b'(' => self.make_token(TokenType::LeftParen),
                b')' => self.make_token(TokenType::RightParen),
                b'{' => self.make_token(TokenType::LeftBrace),
                b'}' => self.make_token(TokenType::RightBrace),
                b';' => self.make_token(TokenType::Semicolon),
                b',' => self.make_token(TokenType::Comma),
                b'.' => self.make_token(TokenType::Dot),
                b'-' => self.make_token(TokenType::Minus),
                b'+' => self.make_token(TokenType::Plus),
                b'/' => self.make_token(TokenType::Slash),
                b'*' => self.make_token(TokenType::Star),
                b'!' => {
                    if self.next_eq(b'=') {
                        self.make_token(TokenType::BangEqual)
                    } else {
                        self.make_token(TokenType::Bang)
                    }
                }
                b'=' => {
                    if self.next_eq(b'=') {
                        self.make_token(TokenType::EqualEqual)
                    } else {
                        self.make_token(TokenType::Equal)
                    }
                }
                b'<' => {
                    if self.next_eq(b'=') {
                        self.make_token(TokenType::LessEqual)
                    } else {
                        self.make_token(TokenType::Less)
                    }
                }
                b'>' => {
                    if self.next_eq(b'=') {
                        self.make_token(TokenType::GreaterEqual)
                    } else {
                        self.make_token(TokenType::Greater)
                    }
                }
                b'"' => self.eat_string(),
                n if n.is_ascii_digit() => self.eat_number(),
                c if c.is_ascii_alphabetic() => self.eat_identifier(),
                _ => {
                    lex_error!("Unknown character.")
                }
            }
        } else {
            self.make_token(TokenType::Eof)
        }
    }

    fn eat_string(&mut self) -> Result<Token> {
        while self.peek_ne(b'"') {
            self.advance();
        }
        self.advance();
        // slice without quotes
        let lexeme = &self.source[self.start + 1..self.current - 1];
        self.make_token(TokenType::StrLit(lexeme))
    }

    fn dnext_is_number(&self) -> bool {
        self.double_peek().map_or(false, |c| c.is_ascii_digit())
    }

    fn next_is_number(&self) -> bool {
        self.peek().map_or(false, |c| c.is_ascii_digit())
    }

    fn eat_number(&mut self) -> Result<Token> {
        while self.next_is_number() {
            self.advance();
        }
        if self.peek_eq(b'.') && self.dnext_is_number() {
            self.advance();
            while self.next_is_number() {
                self.advance();
            }
        }
        let number = &self.source[self.start..self.current];
        if let Ok(n) = number.parse::<f64>() {
            self.make_token(TokenType::Number(n))
        } else {
            lex_error!("Unable to convert number lexeme to f64.")
        }
    }

    fn eat_identifier(&mut self) -> Result<Token> {
        while self.peek().map_or(false, |c| c.is_ascii_alphanumeric()) {
            self.advance();
        }
        let lexeme = &self.source[self.start..self.current];
        match lexeme {
            "and" => self.make_token(TokenType::And),
            "class" => self.make_token(TokenType::Class),
            "else" => self.make_token(TokenType::Else),
            "false" => self.make_token(TokenType::False),
            "for" => self.make_token(TokenType::For),
            "fun" => self.make_token(TokenType::Fun),
            "if" => self.make_token(TokenType::If),
            "nil" => self.make_token(TokenType::Nil),
            "or" => self.make_token(TokenType::Or),
            "print" => self.make_token(TokenType::Print),
            "return" => self.make_token(TokenType::Return),
            "super" => self.make_token(TokenType::Super),
            "this" => self.make_token(TokenType::This),
            "true" => self.make_token(TokenType::True),
            "var" => self.make_token(TokenType::Var),
            "while" => self.make_token(TokenType::While),
            _ => self.make_token(TokenType::Identifier(lexeme))
        }
    }
}
