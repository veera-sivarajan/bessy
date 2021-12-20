use crate::{token::{Token, TokenType}};

pub struct Scanner {
    source: &str,
    start: usize,
    current: usize,
    pub line: u32,
}

impl Scanner {
    pub fn init_scanner(source: &str) -> Scanner {
        Scanner { source, start: 0, current: 0, line: 1 }
    }

    pub fn scan_token(&mut self) -> Token {
        self.scanner.start = self.scanner.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        } else {
            // TODO
        }

        return self.error_token("Unexpected character.");
    }

    fn make_token(&self) -> Token {
        // TODO
    }

    fn error_token(&self, message: &str) -> Token {
        // TODO
    }

    fn is_at_end(&self) -> bool {
        // TODO
    }
}
        
    
