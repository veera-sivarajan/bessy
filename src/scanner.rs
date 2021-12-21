use crate::{token::{Token, TokenType}};

pub struct Scanner<'a> {
    source: &'a str,
    chars: Vec<char>,
    start: usize,
    current: usize,
    pub line: u32,
}

impl Scanner {
    pub fn init_scanner(source: &str) -> Scanner {
        let chars: Vec<char> = source.chars().collect();
        Scanner { source, chars, start: 0, current: 0, line: 1 }
    }

    fn is_at_end(&self) -> bool {
        self.source.as_bytes()[self.current] == b'\0'
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current - 1]
    }

    fn peek(&self) -> char {
        self.chars[self.current]
    }

    fn peek_next(&self) -> char {
        self.chars[self.current + 1]
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => self.advance() += 1;
                '\n' => {
                    self.line += 1;
                    self.advance() += 1;
                },
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return
                    }
                }
                _ => return
            }
        }
    }
                
    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        } else {
            let c = self.advance();
            match c {
                '(' => self.make_token(TokenType::LeftParen),
                ')' => self.make_token(TokenType::RightParen),
                '{' => self.make_token(TokenType::LeftBrace),
                '}' => self.make_token(TokenType::RightBrace),
                ';' => self.make_token(TokenType::Semicolon),
                ',' => self.make_token(TokenType::Comma),
                '.' => self.make_token(TokenType::Dot),
                '-' => self.make_token(TokenType::Minus),
                '+' => self.make_token(TokenType::Plus),
                '/' => self.make_token(TokenType::Slash),
                '*' => self.make_token(TokenType::Star),
                '!' => {
                    let new_type = if self.matches('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                    self.make_token(new_type)
                },
                '=' => {
                    let new_type = if self.match('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                    self.make_token(new_type)
                },
                '<' => {
                    let new_type = if self.match('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                    self.make_token(new_type)
                },
                '>' => {
                    let new_type = if self.match('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                    self.make_token(new_type)
                },
                
            }
        }

        return self.error_token("Unexpected character.");
    }

    fn make_token(&self, kind: TokenType) -> Token {
        Token {
            kind,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    fn error_token<'c>(&self, message: &<'c> str) -> Token<'c> {
        Token {
            kind: TokenType::Error,
            lexeme: message,
            line: self.line,
        }
    }

}
        
    
