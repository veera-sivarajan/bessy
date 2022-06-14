use crate::token::{Token, TokenType};

// Takes a heap allocated string and generates tokens on demand. 

pub struct Scanner<'a> {
    source: &'a str, 
    start: usize,
    current: usize,
    line: u16,
}

impl<'a> Iterator for Scanner<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.is_at_end() {
            self.current += 1;
            Some(self.source.as_bytes()[self.current - 1])
        } else {
            None
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

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;

        if self.is_at_end() {
            Token::new(TokenType::Eof, self.line)
        } else {
            match self.next().unwrap() {
                b'(' => Token::new(TokenType::LeftParen, self.line),
                b')' => Token::new(TokenType::LeftParen, self.line),
                _ => Token::new(TokenType::Unknown, self.line),
            }
        }
    }
}
        
