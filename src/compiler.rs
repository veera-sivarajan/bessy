use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use crate::chunk::{Chunk, Opcode};
use crate::value::Value;
use std::collections::HashMap;
use crate::debug::disassemble_chunk;

#[derive(Copy, Clone)]
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
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::None,
        }
    }
}

struct ParseRule<'src> {
    prefix: Option<fn(&mut Parser<'src>)>,
    infix: Option<fn(&mut Parser<'src>)>,
    precedence: Precedence,
}

impl<'src> ParseRule<'src> {
    fn new(prefix: Option<fn(&mut Parser<'src>)>,
           infix: Option<fn(&mut Parser<'src>)>,
           prec: Precedence) -> ParseRule<'src> {
        ParseRule { prefix, infix, precedence: prec }
    }
}

pub struct Parser<'src> {
    pub chunk: Chunk,
    scanner: Scanner<'src>,
    current: Token<'src>,
    previous: Token<'src>,
    had_error: bool,
    panic_mode: bool,
    rules: HashMap<TokenType, ParseRule<'src>>,
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
            rules: Parser::init_rules(),
        }
    }

    fn init_rules() -> HashMap<TokenType, ParseRule<'src>> {
        let mut buffer = HashMap::new();
        buffer.insert(TokenType::LeftParen, ParseRule::new(Some(Parser::grouping), None, Precedence::None));
        buffer.insert(TokenType::RightParen, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::LeftBrace, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::RightBrace, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Comma, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Dot, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Minus, ParseRule::new(Some(Parser::unary), Some(Parser::binary), Precedence::Term));
        buffer.insert(TokenType::Plus, ParseRule::new(None, Some(Parser::binary), Precedence::Term));
        buffer.insert(TokenType::Semicolon, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Slash, ParseRule::new(None, Some(Parser::binary), Precedence::Factor));
        buffer.insert(TokenType::Star, ParseRule::new(None, Some(Parser::binary), Precedence::Factor));
        buffer.insert(TokenType::Bang, ParseRule::new(Some(Parser::unary), None, Precedence::None));
        buffer.insert(TokenType::BangEqual, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Equal, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::EqualEqual, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Greater, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::GreaterEqual, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Less, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::LessEqual, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Identifier, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::String, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Number, ParseRule::new(Some(Parser::number), None, Precedence::None));
        buffer.insert(TokenType::And, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Class, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Else, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::False, ParseRule::new(Some(Parser::literal), None, Precedence::None));
        buffer.insert(TokenType::For, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Fun, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::If, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Nil, ParseRule::new(Some(Parser::literal), None, Precedence::None));
        buffer.insert(TokenType::Or, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Print, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Return, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::True, ParseRule::new(Some(Parser::literal), None, Precedence::None));
        buffer.insert(TokenType::Var, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::While, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Error, ParseRule::new(None, None, Precedence::None));
        buffer.insert(TokenType::Eof, ParseRule::new(None, None, Precedence::None));
        buffer
    }
        
    pub fn compile(&mut self) -> bool {
        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression."); 
        self.emit_opcode(Opcode::Return);
        if !self.had_error {
            disassemble_chunk(&self.chunk, "code");
        }
        self.end_compiler();
        !self.had_error
    }

    fn advance(&mut self) {
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

    fn consume(&mut self, kind: TokenType, message: &str) {
        if self.current.kind == kind {
            self.advance()
        } else {
            self.error_at_current(message);
        }
    }

    fn get_rule(&self, kind: TokenType) -> &ParseRule<'src> {
        self.rules.get(&kind).unwrap()
    }

    fn is_lower_precedence(&self, precedence: Precedence) -> bool {
        let current_precedence = self.get_rule(self.current.kind).precedence;
        precedence as u8 <= current_precedence as u8
    }

    // NOTE Parsing functions

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.get_rule(self.previous.kind).prefix;
        if prefix_rule.is_none() {
            self.error("Expect expression.");
        } else if let Some(rule) = prefix_rule {
            rule(self);
            while self.is_lower_precedence(precedence) {
                self.advance();
                let infix_rule = self.get_rule(self.previous.kind) .infix
                    .unwrap();
                infix_rule(self);
            }
        } else {
            unreachable!();
        }
    }

    fn number(&mut self) {
        let value: f64 = self.previous.lexeme
            .parse().expect("Failed to convert str to f64");
        self.emit_constant(Value::Number(value));
    }

    fn literal(&mut self) {
        match self.previous.kind {
            TokenType::False => self.emit_opcode(Opcode::False),
            TokenType::True => self.emit_opcode(Opcode::True),
            TokenType::Nil => self.emit_opcode(Opcode::Nil),
            _ => unreachable!(),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operator = self.previous.kind;
        self.parse_precedence(Precedence::Unary); // compile the operand
        match operator {
            TokenType::Minus => self.emit_opcode(Opcode::Negate),
            TokenType::Bang => self.emit_opcode(Opcode::Not),
            _ => unreachable!(),
        }
    }

    fn binary(&mut self) {
        let operator = self.previous.kind;
        let rule = self.get_rule(operator); 
        let next_precedence = rule.precedence.next();
        self.parse_precedence(next_precedence);
        match operator {
            TokenType::Plus => self.emit_opcode(Opcode::Add),
            TokenType::Minus => self.emit_opcode(Opcode::Subtract),
            TokenType::Star => self.emit_opcode(Opcode::Multiply),
            TokenType::Slash => self.emit_opcode(Opcode::Divide),
            _ => unreachable!(),
        }
    }

    // NOTE Auxiliary functions for writing into chunk
    
    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn emit_constant(&mut self, value: Value) {
        let index = self.chunk.add_constant(value);
        self.emit_opcode(Opcode::Constant(index));
    }
        
    fn emit_return(&mut self) {
        self.emit_opcode(Opcode::Return);
    }
    
    // fn emit_bytes(&mut self, ins_a: Opcode, ins_b: Opcode) {
    //     self.emit_opcode(ins_a);
    //     self.emit_opcode(ins_b);
    // }

    fn emit_opcode(&mut self, instruction: Opcode) {
        self.chunk.write_opcode(instruction, self.previous.line);
    }

    // NOTE Auxiliary functions for throwing errors
    
    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current, message)
    }

    fn error(&mut self, message: &str) {
        self.error_at(self.previous, message)
    }

    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic_mode {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compiles_successfully() {
        let mut parser = Parser::new("1");
        assert_eq!(true, parser.compile());
    }

    #[test]
    fn compile_fails() {
        let mut parser = Parser::new("1;");
        assert_eq!(false, parser.compile());
    }

    #[test]
    fn grouping_without_rightparen() {
        let mut parser = Parser::new("(1 + 2");
        assert_eq!(false, parser.compile());
    }
}
