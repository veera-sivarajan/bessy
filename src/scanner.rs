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

impl<'a> Iterator for Scanner<'a> {
    type Item = u8;

    // while this method is supposed to iterate though a collection,
    // here, it also skips unnecessary characters. 
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            None
        } else {
            let c = self.source.as_bytes()[self.current];
            self.current += 1;
            match c {
                b' '|b'\r'|b'\t' => self.next(),
                b'\n' => {
                    self.line += 1;
                    self.next()
                },
                b'/' => {
                    // peekable does not have double peek method
                    if self.source.as_bytes()[self.current] == b'/' {
                        self.skip_while(|c| *c != b'\n').next()
                    } else {
                        Some(c)
                    }
                }
                _ => Some(c),
            }
        }
    }
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner { source, start: 0, current: 0, line: 1 }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn next_eq(&mut self, expected: u8) -> bool {
        self.peekable().next_if_eq(&expected).is_some()
    }

    pub fn scan_token(&mut self) -> Result<Token> {
        self.start = self.current;

        if let Some(c) = self.next() {
            match c {
                b'(' => Ok(Token::new(TokenType::LeftParen, self.line)),
                b')' => Ok(Token::new(TokenType::RightParen, self.line)),
                b'{' => Ok(Token::new(TokenType::LeftBrace, self.line)),
                b'}' => Ok(Token::new(TokenType::RightBrace, self.line)),
                b';' => Ok(Token::new(TokenType::Semicolon, self.line)),
                b',' => Ok(Token::new(TokenType::Comma, self.line)),
                b'.' => Ok(Token::new(TokenType::Dot, self.line)),
                b'-' => Ok(Token::new(TokenType::Minus, self.line)),
                b'+' => Ok(Token::new(TokenType::Plus, self.line)),
                b'/' => Ok(Token::new(TokenType::Slash, self.line)),
                b'*' => Ok(Token::new(TokenType::Star, self.line)),
                b'!' => {
                    if self.next_eq(b'=') {
                        Ok(Token::new(TokenType::BangEqual, self.line))
                    } else {
                        Ok(Token::new(TokenType::Bang, self.line))
                    }
                },
                b'=' => {
                    if self.next_eq(b'=') {
                        Ok(Token::new(TokenType::EqualEqual, self.line))
                    } else {
                        Ok(Token::new(TokenType::Equal, self.line))
                    }
                },
                b'<' => {
                    if self.next_eq(b'=') {
                        Ok(Token::new(TokenType::LessEqual, self.line))
                    } else {
                        Ok(Token::new(TokenType::Less, self.line))
                    }
                },
                b'>' => {
                    if self.next_eq(b'=') {
                        Ok(Token::new(TokenType::GreaterEqual, self.line))
                    } else {
                        Ok(Token::new(TokenType::Greater, self.line))
                    }
                },
                _ => scan_error!()
            }
        } else {
            Ok(Token::new(TokenType::Eof, self.line))
        }
    }
}
        
