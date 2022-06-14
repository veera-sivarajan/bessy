use crate::token::{Token, TokenType};

// Takes a heap allocated string and generates tokens on demand. 

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

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source.as_bytes()[self.current - 1]
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;

        if self.is_at_end() {
            Token::new(TokenType::Eof, self.line)
        } else {
            match self.advance() {
                b'(' => Token::new(TokenType::LeftParen, self.line),
                b')' => Token::new(TokenType::LeftParen, self.line),
                b'{' => Token::new(TokenType::LeftBrace, self.line),
                b'}' => Token::new(TokenType::RightBrace, self.line),
                _ => Token::new(TokenType::Unknown, self.line),
            }
        }
    }
}
        
