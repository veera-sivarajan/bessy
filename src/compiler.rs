use crate::token::{Token, TokenType};
use crate::error::BessyError;
use crate::lexer::Lexer;
use crate::chunk::{Chunk, Value, OpCode};


type Result<T> = std::result::Result<T, BessyError>;

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    fn next(&self) -> Self {
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

pub struct Compiler<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    lexer: Lexer<'a>,
    chunk: Chunk,
}

type ParseRule<'a> = (Option<fn(&mut Compiler<'a>) -> Result<()>>,
                  Option<fn(&mut Compiler<'a>) -> Result<()>>,
                  Precedence);

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Compiler {
            current: Token::new(TokenType::Eof, 0),
            previous: Token::new(TokenType::Eof, 0),
            lexer: Lexer::new(source),
            chunk: Chunk::new(),
        }
    }

    // driving function for the scanner
    // handles all errors from the scanner
    fn advance(&mut self) {
        self.previous = self.current;
        loop {
            match self.lexer.next_token() {
                Err(msg) => eprintln!("{}", msg),
                Ok(t) => {
                    self.current = t;
                    break;
                }
            }
        }
    }

    fn emit_byte(&mut self, op: OpCode) {
        self.chunk.emit_byte(op, self.previous.line);
    }

    // compiles the entire source code to a chunk
    pub fn compile(&mut self) -> Result<&Chunk> {
        self.advance();
        self.expression()?;
        self.consume(TokenType::Eof, "Expect end of expression.")?;
        self.emit_byte(OpCode::Return);
        Ok(&self.chunk)
    }

    fn expression(&mut self) -> Result<()> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn consume(&mut self, kind: TokenType<'a>, msg: &'static str) -> Result<()> {
        if self.current.kind == kind {
            self.advance();
            Ok(())
        } else {
            parse_error!(msg, self.current.line)
        }
    }

    fn parse_precedence(&mut self, bp: Precedence) -> Result<()> {
        self.advance();
        if let Some(p_rule) = self.get_rule(self.previous.kind).0 {
            p_rule(self)?;
            while bp <= self.get_rule(self.current.kind).2 {
                self.advance();
                let infix_rule = self.get_rule(self.previous.kind).1;
                infix_rule.unwrap()(self)?;
            }
            Ok(())
        } else {
            parse_error!("Expected expression!", self.current.line)
        }
    }

    fn number(&mut self) -> Result<()> {
        if let TokenType::Number(value) = self.previous.kind {
            let index = self.chunk.add_constant(Value::Number(value));
            Ok(self.emit_byte(OpCode::Constant(index)))
        } else {
            parse_error!("Expected Number!", self.current.line)
        }
    }

    fn grouping(&mut self) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after expression")?;
        Ok(())
    }

    fn unary(&mut self) -> Result<()> {
        let operator = self.previous.kind;
        self.parse_precedence(Precedence::Unary)?;
        match operator {
            TokenType::Minus => Ok(self.emit_byte(OpCode::Negate)),
            _ => Ok(())
        }
    }

    fn binary(&mut self) -> Result<()> {
        let operator = self.previous.kind;
        let rule = self.get_rule(operator).2;
        self.parse_precedence(rule.next())?;
        match operator {
            TokenType::Plus => Ok(self.emit_byte(OpCode::Add)),
            TokenType::Minus => Ok(self.emit_byte(OpCode::Subtract)),
            TokenType::Star => Ok(self.emit_byte(OpCode::Multiply)),
            TokenType::Slash => Ok(self.emit_byte(OpCode::Divide)),
            _ => Ok(()),
        }
    }

    fn get_rule(&self, kind: TokenType<'a>) -> ParseRule<'a> {
        match kind {
            TokenType::LeftParen => (Some(Compiler::grouping), None, Precedence::None), 
            TokenType::RightParen => (None, None, Precedence::None),
            TokenType::Dot => (None, None, Precedence::None),
            TokenType::Minus => (Some(Compiler::unary), Some(Compiler::binary), Precedence::Term),
            TokenType::Plus => (None, Some(Compiler::binary), Precedence::Term),
            TokenType::Slash => (None, Some(Compiler::binary), Precedence::Factor),
            TokenType::Star => (None, Some(Compiler::binary), Precedence::Factor),
            TokenType::Semicolon => (None, None, Precedence::None),
            TokenType::Eof => (None, None, Precedence::None),
            TokenType::LeftBrace => (None, None, Precedence::None),
            TokenType::RightBrace => (None, None, Precedence::None),
            TokenType::Comma => (None, None, Precedence::None),

            TokenType::Bang => (None, None, Precedence::None),
            TokenType::BangEqual => (None, None, Precedence::None),
            TokenType::Equal => (None, None, Precedence::None),
            TokenType::EqualEqual => (None, None, Precedence::None),
            TokenType::Greater => (None, None, Precedence::None),
            TokenType::GreaterEqual => (None, None, Precedence::None),
            TokenType::Less => (None, None, Precedence::None),
            TokenType::LessEqual => (None, None, Precedence::None),

            TokenType::Number(_) => (Some(Compiler::number), None, Precedence::None),
            TokenType::True => (None, None, Precedence::None),
            TokenType::False => (None, None, Precedence::None),
            TokenType::Identifier(_) => (None, None, Precedence::None),
            TokenType::StrLit(_) => (None, None, Precedence::None),
            TokenType::Print => (None, None, Precedence::None),
            TokenType::Var => (None, None, Precedence::None),
            TokenType::Nil => (None, None, Precedence::None),
            TokenType::If => (None, None, Precedence::None),
            TokenType::Else => (None, None, Precedence::None),
            TokenType::While => (None, None, Precedence::None),
            TokenType::For => (None, None, Precedence::None),
            TokenType::Fun => (None, None, Precedence::None),
            TokenType::Return => (None, None, Precedence::None),
            TokenType::And => (None, None, Precedence::None),
            TokenType::Or => (None, None, Precedence::None),
            TokenType::Class => (None, None, Precedence::None),
            TokenType::Super => (None, None, Precedence::None),
            TokenType::This => (None, None, Precedence::None),
        }
    }
}
        
