use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use crate::chunk::{Chunk, Opcode};

pub struct Parser<'src> {
    scanner: Scanner<'src>
    current: Token<'src>,
    previous: Token<'src>,
    had_error: Bool,
    panic_mode: Bool,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'a str) -> Parser<'src> {
        Parser {
            scanner: Scanner::new(),
            current: Token {
                kind: TokenType::Eof,
                lexeme: "",
                line: 0,
            },
            previous: Token {
                kind: TokenType::Eof,
                lexeme: "",
                line: 0,
            },
            had_error: false,
            panic_mode: false,
        }
    }
        
    pub fn compile(&mut self) -> bool {
        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression."); // TODO
        self.emit(Opcode::Return);
        if !self.had_error {
            self.chunk.disassemble("code");
        }
        self.had_error
    }

    fn advance(&self) {
        self.previous = self.current;
        
        loop {
            self.current = self.scanner.scan_toke();
            if self.current.kind != TokenType::Error {
                break;
            } else {
                self.error_at_current(self.current.lexeme);
            }
        }
    }
}
