use crate::{token::{Token, TokenType}};

pub struct Scanner<'src> {
    source: &'src str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'src> Scanner<'src> {
    pub fn new(source: &str) -> Scanner {
        Scanner { source, start: 0, current: 0, line: 1 }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let curr_char = self.peek();
        self.current += 1;
        curr_char
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            0
        } else {
            self.source.as_bytes()[self.current]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.current > self.source.len() - 2 {
            b'\0'
        } else {
            self.source.as_bytes()[self.current + 1]
        }
    }

    fn matches(&mut self, expected: u8) -> bool {
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
                b' ' | b'\r' | b'\t' => self.current += 1,
                b'\n' => {
                    self.line += 1;
                    self.current += 1;
                },
                b'/' if self.peek_next() == b'/' => {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return
            }
        }
    }
                
    pub fn scan_token(&mut self) -> Token<'src> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            self.make_token(TokenType::Eof)
        } else {
            match self.advance() {
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
                    let new_type = if self.matches(b'=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.make_token(new_type)
                },
                b'=' => {
                    let new_type = if self.matches(b'=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.make_token(new_type)
                },
                b'<' => {
                    let new_type = if self.matches(b'=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.make_token(new_type)
                },
                b'>' => {
                    let new_type = if self.matches(b'=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.make_token(new_type)
                },
                b'"' => self.scan_string(),
                c if c.is_ascii_digit() => self.scan_number(),
                c if c.is_ascii_alphabetic() || c == b'_' => self.identifier(),
                _ => self.error_token("Unexpected character."),
            }
        }
    }

    fn scan_number(&mut self) -> Token<'src> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            self.advance();
            
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn scan_string(&mut self) -> Token<'src> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated String.")
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    fn identifier(&mut self) -> Token<'src> {
        while self.peek().is_ascii_alphabetic() || self.peek().is_ascii_digit() {
            self.advance();
        }
        let id_type = self.identifier_type();
        self.make_token(id_type)
    }

    fn identifier_type(&mut self) -> TokenType {
        match self.source.as_bytes()[self.start] {
            b'a' => self.check_rest(1, 2, "nd", TokenType::And),
            b'e' => self.check_rest(1, 3, "lse", TokenType::Else),
            b'i' => self.check_rest(1, 1, "f", TokenType::If),
            b'n' => self.check_rest(1, 2, "il", TokenType::Nil),
            b'o' => self.check_rest(1, 1, "r", TokenType::Or),
            b'p' => self.check_rest(1, 4, "rint", TokenType::Print),
            b'r' => self.check_rest(1, 5, "eturn", TokenType::Return),
            b'v' => self.check_rest(1, 2, "ar", TokenType::Var),
            b'w' => self.check_rest(1, 4, "hile", TokenType::While),
            b't' => self.check_rest(1, 3, "rue", TokenType::True),
            b'f' => {
                match self.source.as_bytes()[self.start + 1] {
                    b'a' => self.check_rest(2, 3, "lse", TokenType::False),
                    b'o' => self.check_rest(2, 1, "r", TokenType::For),
                    b'u' => self.check_rest(2, 1, "n", TokenType::Fun),
                    _ => TokenType::Identifier,
                }
            },
            _ => TokenType::Identifier,
        }
    }

    fn check_rest(&self,
                  start: usize,
                  len: usize,
                  rest: &str, kind: TokenType) -> TokenType {
        let lexeme = &self.source[self.start..self.current];
        if lexeme.len() == start + len && &lexeme[start..] == rest {
            kind
        } else {
            TokenType::Identifier
        }
    }
        
    fn make_token(&self, kind: TokenType) -> Token<'src> {
        Token {
            kind,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    fn error_token(&self, message: &'static str) -> Token<'static> {
        Token {
            kind: TokenType::Error,
            lexeme: message,
            line: self.line,
        }
    }
}
        
    
