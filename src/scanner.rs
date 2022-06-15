use crate::token::{Token, TokenType};
use crate::error::BessyError;

// Takes a heap allocated string and generates tokens on demand. 

type Result<T> = std::result::Result<T, BessyError>;

pub struct Scanner<'a> {
    source: &'a str, 
    start: usize,
    current: usize,
    line: u16,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner { source, start: 0, current: 0, line: 1 }
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
        if let Some(expected) = self.peek() { 
            self.advance();
            true
        } else {
            false
        }
    }

    fn make_token(&self, kind: TokenType) -> Result<Token> {
        Ok(Token::new(kind, self.line))
    }

    fn skip_needless(&mut self) {
        loop {
            if let Some(c) = self.peek() {
                match c {
                    b' '|b'\r'|b'\t' => {self.advance();}
                    b'\n' => {
                        self.advance();
                        self.line += 1;
                    }
                    b'/' if self.double_peek().map_or(false, |c| c == b'/') => {
                        while self.peek().map_or(false, |c| c != b'\n') {
                            self.advance();
                        }
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
    }

    pub fn scan_token(&mut self) -> Result<Token> {
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
                },
                b'=' => {
                    if self.next_eq(b'=') {
                        self.make_token(TokenType::EqualEqual)
                    } else {
                        self.make_token(TokenType::Equal)
                    }
                },
                b'<' => {
                    if self.next_eq(b'=') {
                        self.make_token(TokenType::LessEqual)
                    } else {
                        self.make_token(TokenType::Less)
                    }
                },
                b'>' => {
                    if self.next_eq(b'=') {
                        self.make_token(TokenType::GreaterEqual)
                    } else {
                        self.make_token(TokenType::Greater)
                    }
                },
                b'"' => self.scan_string(),
                n if n.is_ascii_digit() => self.scan_number(),
                _ => {
                    eprintln!("Unknown character!");
                    scan_error!()
                }
            }
        } else {
            self.make_token(TokenType::Eof)
        }
    }

    fn scan_number(&mut self) -> Result<Token> {
        while self.peek().unwrap().is_ascii_digit() {
            self.advance();
        }
        self.make_token(TokenType::Number(1))
    }

    fn scan_string(&mut self) -> Result<Token> {
        while self.peek().unwrap() != b'"' {
            self.advance();
        }
        self.advance();
        self.make_token(TokenType::StrLit)
    }
}
        
