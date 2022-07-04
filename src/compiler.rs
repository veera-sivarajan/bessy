use crate::chunk::{Chunk, OpCode, Value};
use crate::error::BessyError;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

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

type ParseRule<'a> = (
    Option<fn(&mut Compiler<'a>) -> Result<()>>,
    Option<fn(&mut Compiler<'a>) -> Result<()>>,
    Precedence,
);

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

    fn emit(&mut self, op: OpCode) {
        self.chunk.emit_byte(op, self.previous.line);
    }

    fn emits(&mut self, a: OpCode, b: OpCode) {
        self.emit(a);
        self.emit(b);
    }

    fn next_eq(&mut self, kind: TokenType<'a>) -> bool {
        if self.current.kind == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    // compiles the entire source code to a chunk
    pub fn compile(&mut self) -> Result<&Chunk> {
        self.advance();
        while !self.next_eq(TokenType::Eof) {
            self.declaration()?;
        }
        self.consume(TokenType::Eof, "Expect end of expression.")?;
        self.emit(OpCode::Return);
        Ok(&self.chunk)
    }

    fn declaration(&mut self) -> Result<()> {
        if self.next_eq(TokenType::Var) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<()> {
        let name_index = self.parse_variable("Expect variable name.")?;
        if self.next_eq(TokenType::Equal) {
            self.expression()?;
        } else {
            self.emit(OpCode::Nil);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        self.emit(OpCode::DefineGlobal(name_index));
        Ok(())
    }

    fn parse_variable(&mut self, error_msg: &str) -> Result<usize> {
        if let TokenType::Identifier(lexeme) = self.current.kind {
            self.advance();
            Ok(self.chunk.add_constant(Value::String(lexeme.to_owned())))
        } else {
            parse_error!(error_msg, self.previous.line)
        }
    }

    fn statement(&mut self) -> Result<()> {
        if self.next_eq(TokenType::Print) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        self.emit(OpCode::Print);
        Ok(())
    }

    fn expression_statement(&mut self) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        self.emit(OpCode::Pop);
        Ok(())
    }

    fn expression(&mut self) -> Result<()> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn consume(&mut self, kind: TokenType<'a>, msg: &str) -> Result<()> {
        if self.current.kind == kind {
            self.advance();
            Ok(())
        } else {
            parse_error!(msg, self.previous.line)
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
            parse_error!("Expected expression!", self.previous.line)
        }
    }

    fn number(&mut self) -> Result<()> {
        if let TokenType::Number(value) = self.previous.kind {
            let index = self.chunk.add_constant(Value::Number(value));
            Ok(self.emit(OpCode::Constant(index)))
        } else {
            parse_error!("Expected Number!", self.previous.line)
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
            TokenType::Minus => Ok(self.emit(OpCode::Negate)),
            TokenType::Bang => Ok(self.emit(OpCode::Not)),
            _ => Ok(()),
        }
    }

    fn binary(&mut self) -> Result<()> {
        let operator = self.previous.kind;
        let rule = self.get_rule(operator).2;
        self.parse_precedence(rule.next())?;
        match operator {
            TokenType::Plus => Ok(self.emit(OpCode::Add)),
            TokenType::Minus => Ok(self.emit(OpCode::Subtract)),
            TokenType::Star => Ok(self.emit(OpCode::Multiply)),
            TokenType::Slash => Ok(self.emit(OpCode::Divide)),
            TokenType::BangEqual => Ok(self.emits(OpCode::Equal, OpCode::Not)),
            TokenType::EqualEqual => Ok(self.emit(OpCode::Equal)),
            TokenType::Greater => Ok(self.emit(OpCode::Greater)),
            TokenType::GreaterEqual => Ok(self.emits(OpCode::Less, OpCode::Not)),
            TokenType::Less => Ok(self.emit(OpCode::Less)),
            TokenType::LessEqual => Ok(self.emits(OpCode::Greater, OpCode::Not)),
            _ => Ok(()),
        }
    }

    fn literal(&mut self) -> Result<()> {
        match self.previous.kind {
            TokenType::False => Ok(self.emit(OpCode::False)),
            TokenType::True => Ok(self.emit(OpCode::True)),
            TokenType::Nil => Ok(self.emit(OpCode::Nil)),
            _ => unreachable!(),
        }
    }

    fn string(&mut self) -> Result<()> {
        if let TokenType::StrLit(lexeme) = self.previous.kind {
            let index = self.chunk.add_constant(Value::String(String::from(lexeme)));
            Ok(self.emit(OpCode::Constant(index)))
        } else {
            parse_error!("Expected String literal.", self.previous.line)
        }
    }
                                            

    fn get_rule(&self, kind: TokenType<'a>) -> ParseRule<'a> {
        match kind {
            TokenType::LeftParen => (Some(Compiler::grouping), None, Precedence::None),
            TokenType::RightParen => (None, None, Precedence::None),
            TokenType::Dot => (None, None, Precedence::None),
            TokenType::Minus => (
                Some(Compiler::unary),
                Some(Compiler::binary),
                Precedence::Term,
            ),
            TokenType::Plus => (None, Some(Compiler::binary), Precedence::Term),
            TokenType::Slash => (None, Some(Compiler::binary), Precedence::Factor),
            TokenType::Star => (None, Some(Compiler::binary), Precedence::Factor),
            TokenType::Semicolon => (None, None, Precedence::None),
            TokenType::Eof => (None, None, Precedence::None),
            TokenType::LeftBrace => (None, None, Precedence::None),
            TokenType::RightBrace => (None, None, Precedence::None),
            TokenType::Comma => (None, None, Precedence::None),
            TokenType::Bang => (Some(Compiler::unary), None, Precedence::None),
            TokenType::BangEqual => (None, Some(Compiler::binary), Precedence::Equality),
            TokenType::Equal => (None, None, Precedence::None),
            TokenType::EqualEqual => (None, Some(Compiler::binary), Precedence::Equality),
            TokenType::Greater => (None, Some(Compiler::binary), Precedence::Comparison),
            TokenType::GreaterEqual => (None, Some(Compiler::binary), Precedence::Comparison),
            TokenType::Less => (None, Some(Compiler::binary), Precedence::Comparison),
            TokenType::LessEqual => (None, Some(Compiler::binary), Precedence::Comparison),
            TokenType::Number(_) => (Some(Compiler::number), None, Precedence::None),
            TokenType::True => (Some(Compiler::literal), None, Precedence::None),
            TokenType::False => (Some(Compiler::literal), None, Precedence::None),
            TokenType::Identifier(_) => (None, None, Precedence::None),
            TokenType::StrLit(_) => (Some(Compiler::string), None, Precedence::None),
            TokenType::Print => (None, None, Precedence::None),
            TokenType::Var => (None, None, Precedence::None),
            TokenType::Nil => (Some(Compiler::literal), None, Precedence::None),
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
