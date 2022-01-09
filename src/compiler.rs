use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use crate::chunk::{Chunk, Opcode};
use crate::value::Value;
use crate::debug;

enum Precedence {
    None, // lowest precedence
    Assignment, // =
    Or, // or
    And, //and
    Equality, // ==, !=
    Comparison, // <, >, <=, >=
    Term, // +, -
    Factor, // *, /
    Unary, // !, -
    Call, // ., ()
    Primary // higher precedence
}

impl Precedence {
    fn next(&self) -> Precedence {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Primary,
            Precedence::Primary => Precedence::None,
        }
    }
}

pub struct Parser<'src> {
    pub chunk: Chunk,
    scanner: Scanner<'src>,
    current: Token<'src>,
    previous: Token<'src>,
    had_error: bool,
    panic_mode: bool,
}

impl<'src> Parser<'src> {
    // NOTE Auxiliary functions for parser
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
        self.emit_opcode(Opcode::Return);
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

    // NOTE Parsing functions

    fn expression(&self) {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(&self, precedence: Precedence) {

    }

    fn number(&self) {
        let value: f64 = self.previous.lexeme
            .parse().expect("Failed to convert str to f64");
        self.emit_constant(value);
    }

    fn grouping(&self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&self) {
        let operator = self.previous.kind;
        self.parse_precedence(Precedence::Unary); // compile the operand
        match operator {
            TokenType::Minus => self.emit_opcode(Opcode::Subtract),
            _ => unreachable!(),
        }
    }

    fn binary(&self) {
        let operator = self.previous.kind;
        let rule = self.get_rule(operator); 
        self.parse_precedence(rule.precedence.next());
        match operator {
            TokenType::Plus => self.emit_opcode(Opcode::Plus),
            TokenType::Minus => self.emit_opcode(Opcode::Subtract),
            TokenType::Star => self.emit_opcode(Opcode::Multiply),
            TokenType::Slash => self.emit_opcode(Opcode::Divide),
            _ => unreachable!(),
        }
    }

    // NOTE Auxiliary functions for writing into chunk
    
    fn end_compiler(&self) {
        self.emit_return();
    }

    fn emit_constant(&self, value: Value) {
        let index = self.chunk.add_constant(value);
        if index > usize::MAX { 
            self.error("Too many constants in one chunk.");
            return;
        } else {
            self.emit_opcode(Opcode::Constant(index));
        }
    }
        
    fn emit_return(&self) {
        self.emit_opcode(Opcode::Return);
    }
    
    fn emit_bytes(&self, ins_a: Opcode, ins_b: Opcode) {
        self.emit_opcode(ins_a);
        self.emit_opcode(ins_b);
    }

    fn emit_opcode(&self, instruction: Opcode) {
        self.chunk.write_opcode(instruction, self.previous.line);
    }

    // NOTE Auxiliary functions for throwing errors
    
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
