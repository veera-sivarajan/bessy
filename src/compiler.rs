use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use crate::chunk::{Chunk, Opcode};
use crate::debug;

pub struct Parser<'src> {
    pub chunk: Chunk,
    scanner: Scanner<'src>,
    current: Token<'src>,
    previous: Token<'src>,
    had_error: bool,
    panic_mode: bool,
}

impl<'src> Parser<'src> {
    pub fn new(source: & str) -> Parser {
        Parser {
            scanner: Scanner::new(source),
            chunk: Chunk::new(),
            current: Token::default(),
            previous: Token::default(),
            had_error: false,
            panic_mode: false,
        }
    }
        
    pub fn compile(&mut self) -> bool {
        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression."); // TODO
        self.emit_byte(Opcode::Return);
        if !self.had_error {
            // self.chunk.disassemble("code");
        }
        self.end_compiler();
        !self.had_error
    }

    fn advance(&self) {
        self.previous = self.current;
        
        loop {
            self.current = self.scanner.scan_token();
            if self.current.kind != TokenType::Error {
                break;
            } else {
                self.error_at_current(self.current.lexeme);
            }
        }
    }

    fn consume(&self, kind: TokenType, message: &str) {
        if self.current.kind == kind {
            self.advance()
        } else {
            self.error_at_current(message);
        }
    }

    fn end_compiler(&self) {
        self.emit_return();
    }

    fn emit_return(&self) {
        self.emit_byte(Opcode::Return);
    }
    
    fn emit_bytes(&self, ins_a: Opcode, ins_b: Opcode) {
        self.emit_byte(ins_a);
        self.emit_byte(ins_b);
    }

    fn emit_byte(&self, instruction: Opcode) {
        self.chunk.write_opcode(instruction, self.previous.line as usize);
    }

    fn error_at_current(&self, message: &str) {
        self.error_at(self.current, message)
    }

    fn error(&self, message: &str) {
        self.error_at(self.previous, message)
    }

    fn error_at(&self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        } else {
            self.panic_mode = true;
            eprint!("[line {}] Error", token.line);
            match token.kind {
                TokenType::Eof => eprint!(" at end"),
                TokenType::Error => (),
                _ => eprint!(" at line {}", token.line),
            }
            eprintln!(": {}", message);
            self.had_error = true;
        }
    }
            
            
        
}
