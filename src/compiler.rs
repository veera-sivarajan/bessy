use crate::chunk::{Chunk, OpCode, Value};
use crate::error::BessyError;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::convert::TryFrom;

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

#[derive(Debug)]
struct Local<'a> {
    name: Token<'a>,
    depth: Option<u32>,
}

enum Resolved {
    Local(usize),
    Global,
    Nope,
}

#[derive(Default)]
pub struct Compiler<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    lexer: Lexer<'a>,
    chunk: Chunk,
    locals: Vec<Local<'a>>,
    scope_depth: u32,
}

type ParseRule<'a> = (
    Option<fn(&mut Compiler<'a>, bool) -> Result<()>>,
    Option<fn(&mut Compiler<'a>, bool) -> Result<()>>,
    Precedence,
);

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Compiler {
            lexer: Lexer::new(source),
            locals: Vec::with_capacity(u8::MAX as usize), // immediately provide a vector with capacity instead of doing multiple allocations
            ..Default::default()
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
        let _ = self.chunk.emit_byte(op, self.previous.line);
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

    fn create_string(&mut self, lexeme: &str) -> usize {
        let str_index = self.chunk.strings.intern(lexeme);
        self.chunk.add_constant(Value::String(str_index))
    }

    // compiles the entire source code to a chunk
    pub fn compile(&mut self) -> Result<&mut Chunk> {
        self.advance();
        while !self.next_eq(TokenType::Eof) {
            self.declaration()?;
        }
        self.consume(TokenType::Eof, "Expect end of expression.")?;
        self.emit(OpCode::Return);
        Ok(&mut self.chunk)
    }

    fn declaration(&mut self) -> Result<()> {
        if self.next_eq(TokenType::Var) {
            if self.scope_depth == 0 {
                self.global_var()
            } else {
                self.local_var()
            }
        } else {
            self.statement()
        }
    }

    fn init_variable(&mut self) -> Result<()> {
        if self.next_eq(TokenType::Equal) {
            self.expression()?;
        } else {
            self.emit(OpCode::Nil);
        }
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )
    }

    fn local_var(&mut self) -> Result<()> {
        if let TokenType::Identifier(_) = self.current.kind {
            self.advance();
            if self.locals.len() == u8::MAX as usize {
                parse_error!("Too many local variables.", self.previous.line)
            } else {
                let token = self.previous;
                self.is_unique(token)?;
                let local = Local {
                    name: token,
                    depth: None,
                };
                self.locals.push(local);
                self.init_variable()?;
                self.mark_initialized();
                Ok(())
            }
        } else {
            parse_error!("Expected variable identifier.", self.previous.line)
        }
    }

    fn mark_initialized(&mut self) {
        let last = self
            .locals
            .last_mut()
            .expect("Tried to pop out of empty locals vector.");
        last.depth = Some(self.scope_depth);
    }

    fn is_unique(&mut self, given: Token<'a>) -> Result<()> {
        for l in self
            .locals
            .iter()
            .rev()
            .filter(|l| l.depth.is_some() && l.depth >= Some(self.scope_depth)) {
            if l.name == given {
                return parse_error!(
                    "Already a variable with this name in this scope.",
                    self.previous.line
                );
            }
        }
        Ok(())
    }

    fn global_var(&mut self) -> Result<()> {
        let name_index = self.parse_variable("Expect variable name.")?;
        self.init_variable()?;
        self.emit(OpCode::DefineGlobal(name_index));
        Ok(())
    }

    fn parse_variable(&mut self, error_msg: &str) -> Result<usize> {
        if let TokenType::Identifier(lexeme) = self.current.kind {
            self.advance();
            Ok(self.create_string(lexeme))
        } else {
            parse_error!(error_msg, self.previous.line)
        }
    }

    fn statement(&mut self) -> Result<()> {
        if self.next_eq(TokenType::Print) {
            self.print_stmt()
        } else if self.next_eq(TokenType::LeftBrace) {
            self.begin_scope();
            self.block()?;
            self.end_scope();
            Ok(())
        } else if self.next_eq(TokenType::If) {
            self.if_stmt()
        } else if self.next_eq(TokenType::While) {
            self.while_stmt()
        } else if self.next_eq(TokenType::For) {
            self.for_stmt()
        } else {
            self.expression_stmt()
        }
    }

    fn print_stmt(&mut self) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        self.emit(OpCode::Print);
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn block(&mut self) -> Result<()> {
        while self.current.kind != TokenType::RightBrace && self.current.kind != TokenType::Eof {
            self.declaration()?;
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        for i in (0..self.locals.len()).rev() {
            if self.locals[i].depth > Some(self.scope_depth) {
                self.emit(OpCode::Pop);
                self.locals.pop();
            }
        }
    }

    fn if_stmt(&mut self) -> Result<()> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        self.expression()?; // condition expression
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let then_jump = self.emit_jump(OpCode::JumpIfFalse(0)); // jump to else if condition expression is false
        self.emit(OpCode::Pop); // pop condition expression before executing then branch
        self.statement()?; // then branch
        let else_jump = self.emit_jump(OpCode::Jump(0)); // jump over else branch after executing then branch
        self.patch_jump(then_jump)?; // backpatch then_jump to right before else branch
        self.emit(OpCode::Pop); // pop condition expression before executing else branch
        if self.next_eq(TokenType::Else) {
            self.statement()?; // else branch
        }
        self.patch_jump(else_jump)?; // backpatch else_jump to right after else branch
        Ok(())
    }

    fn emit_jump(&mut self, op: OpCode) -> usize {
        self.chunk.emit_byte(op, self.previous.line)
    }

    fn patch_jump(&mut self, pos: usize) -> Result<()> {
        let index = self.chunk.code.len() - 1 - pos;
        let new_index = match u16::try_from(index) {
            Ok(i) => i,
            Err(_) => return parse_error!("Too much code to skip over.", self.previous.line),
        };
        match self.chunk.code[pos] {
            OpCode::JumpIfFalse(ref mut index)
            | OpCode::Jump(ref mut index) => *index = new_index,
            _ => unreachable!(),
        }
        Ok(())
    }

    fn or(&mut self, _can_assign: bool) -> Result<()> {
        let else_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        let end_jump = self.emit_jump(OpCode::Jump(0));
        self.patch_jump(else_jump)?;
        self.emit(OpCode::Pop);
        self.parse_precedence(Precedence::Or)?;
        self.patch_jump(end_jump)?;
        Ok(())
    }

    fn and(&mut self, _can_assign: bool) -> Result<()> {
        let end_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        self.emit(OpCode::Pop);
        self.parse_precedence(Precedence::And)?;
        self.patch_jump(end_jump)?;
        Ok(())
    }

    fn while_stmt(&mut self) -> Result<()> {
        let loop_start = self.chunk.code.len() - 1;
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        self.emit(OpCode::Pop);
        self.statement()?;
        self.emit_loop(loop_start)?;
        self.patch_jump(body_jump)?;
        self.emit(OpCode::Pop);
        Ok(())
    }

    fn emit_loop(&mut self, start: usize) -> Result<()> {
        let offset = self.chunk.code.len() - start;
        let offset = match u16::try_from(offset) {
            Ok(i) => i,
            Err(_) => return parse_error!(
                "Loop body too large.",
                self.previous.line
            ),
        };
        self.emit(OpCode::Loop(offset));
        Ok(())
    }

    // for (var i = 0; i < 10; i = i + 1) {
    //      print i;
    // }
    fn for_stmt(&mut self) -> Result<()> {
        self.begin_scope();
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
        
        // initializer caluse
        if self.next_eq(TokenType::Semicolon) {
            // no initializer
        } else if self.next_eq(TokenType::Var) {
            self.local_var()?;
        } else {
            self.expression_stmt()?;
        }
        
        let mut loop_start = self.chunk.code.len() - 1;

        // condition clause
        let mut exit_jump = None;
        if !self.next_eq(TokenType::Semicolon) {
            self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;
            exit_jump = Some(self.emit_jump(OpCode::JumpIfFalse(0)));
            self.emit(OpCode::Pop);
        }

        // increment expression
        if !self.next_eq(TokenType::RightParen) {
            let body_jump = self.emit_jump(OpCode::Jump(0));
            let increment_start = self.chunk.code.len() - 1;
            self.expression()?;
            self.emit(OpCode::Pop);
            self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

            self.emit_loop(loop_start)?;
            loop_start = increment_start;
            self.patch_jump(body_jump)?;
        }
            
        self.statement()?;
        self.emit_loop(loop_start)?;
        if let Some(value) = exit_jump {
            self.patch_jump(value)?;
            self.emit(OpCode::Pop);
        }
        self.end_scope();
        Ok(())
    }

    fn expression_stmt(&mut self) -> Result<()> {
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
        if let Some(prefix_rule) = self.get_rule(self.previous.kind).0 {
            let can_assign = bp <= Precedence::Assignment;
            prefix_rule(self, can_assign)?;
            while bp <= self.get_rule(self.current.kind).2 {
                self.advance();
                let infix_rule = self.get_rule(self.previous.kind).1;
                infix_rule.unwrap()(self, can_assign)?;
            }
            if can_assign && self.next_eq(TokenType::Equal) {
                parse_error!("Invalid assignment target.", self.previous.line)
            } else {
                Ok(())
            }
        } else {
            parse_error!("Expected expression!", self.previous.line)
        }
    }

    fn number(&mut self, _can_assign: bool) -> Result<()> {
        if let TokenType::Number(value) = self.previous.kind {
            let index = self.chunk.add_constant(Value::Number(value));
            self.emit(OpCode::Constant(index));
            Ok(())
        } else {
            parse_error!("Expected Number!", self.previous.line)
        }
    }

    fn grouping(&mut self, _can_assign: bool) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after expression")?;
        Ok(())
    }

    fn unary(&mut self, _can_assign: bool) -> Result<()> {
        let operator = self.previous.kind;
        self.parse_precedence(Precedence::Unary)?;
        match operator {
            TokenType::Minus => {
                self.emit(OpCode::Negate);
                Ok(())
            }
            TokenType::Bang => {
                self.emit(OpCode::Not);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn binary(&mut self, _can_assign: bool) -> Result<()> {
        let operator = self.previous.kind;
        let rule = self.get_rule(operator).2;
        self.parse_precedence(rule.next())?;
        match operator {
            TokenType::Plus => {
                self.emit(OpCode::Add);
                Ok(())
            }
            TokenType::Minus => {
                self.emit(OpCode::Subtract);
                Ok(())
            }
            TokenType::Star => {
                self.emit(OpCode::Multiply);
                Ok(())
            }
            TokenType::Slash => {
                self.emit(OpCode::Divide);
                Ok(())
            }
            TokenType::BangEqual => {
                self.emits(OpCode::Equal, OpCode::Not);
                Ok(())
            }
            TokenType::EqualEqual => {
                self.emit(OpCode::Equal);
                Ok(())
            }
            TokenType::Greater => {
                self.emit(OpCode::Greater);
                Ok(())
            }
            TokenType::GreaterEqual => {
                self.emits(OpCode::Less, OpCode::Not);
                Ok(())
            }
            TokenType::Less => {
                self.emit(OpCode::Less);
                Ok(())
            }
            TokenType::LessEqual => {
                self.emits(OpCode::Greater, OpCode::Not);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn literal(&mut self, _can_assign: bool) -> Result<()> {
        match self.previous.kind {
            TokenType::False => {
                self.emit(OpCode::False);
                Ok(())
            }
            TokenType::True => {
                self.emit(OpCode::True);
                Ok(())
            }
            TokenType::Nil => {
                self.emit(OpCode::Nil);
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn string(&mut self, _can_assign: bool) -> Result<()> {
        if let TokenType::StrLit(lexeme) = self.previous.kind {
            let index = self.create_string(lexeme);
            self.emit(OpCode::Constant(index));
            Ok(())
        } else {
            parse_error!("Expected String literal.", self.previous.line)
        }
    }

    fn variable(&mut self, can_assign: bool) -> Result<()> {
        self.named_variable(self.previous, can_assign)
    }

    fn named_variable(&mut self, name: Token<'a>, can_assign: bool) -> Result<()> {
        if let TokenType::Identifier(lexeme) = name.kind {
            let get_op;
            let set_op;
            match self.resolve_local(name) {
                Resolved::Global => {
                    let index = self.create_string(lexeme);
                    get_op = OpCode::GetGlobal(index);
                    set_op = OpCode::SetGlobal(index);
                }
                Resolved::Local(i) => {
                    get_op = OpCode::GetLocal(i);
                    set_op = OpCode::SetLocal(i);
                }
                Resolved::Nope => {
                    return parse_error!(
                        "Can't read local variable in its own initializer.",
                        self.previous.line
                    )
                }
            }

            if self.next_eq(TokenType::Equal) && can_assign {
                // l-value
                self.expression()?;
                self.emit(set_op);
                Ok(())
            } else {
                // r-value
                self.emit(get_op);
                Ok(())
            }
        } else {
            parse_error!("Expected variable name.", self.previous.line)
        }
    }

    fn resolve_local(&mut self, name: Token<'a>) -> Resolved {
        for (i, l) in self.locals.iter().enumerate().rev() {
            if l.name == name {
                if l.depth.is_none() {
                    return Resolved::Nope;
                } else {
                    return Resolved::Local(i);
                }
            }
        }
        Resolved::Global
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
            TokenType::Identifier(_) => (Some(Compiler::variable), None, Precedence::None),
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
            TokenType::And => (None, Some(Compiler::and), Precedence::And),
            TokenType::Or => (None, Some(Compiler::or), Precedence::Or),
            TokenType::Class => (None, None, Precedence::None),
            TokenType::Super => (None, None, Precedence::None),
            TokenType::This => (None, None, Precedence::None),
        }
    }
}
