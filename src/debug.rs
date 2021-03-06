use crate::chunk::{Chunk, OpCode, Value};
use std::fmt;

impl fmt::Debug for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Add => write!(f, "ADD"),
            OpCode::Negate => write!(f, "NEGATE"),
            OpCode::Multiply => write!(f, "MULTIPLY"),
            OpCode::Divide => write!(f, "DIVIDE"),
            OpCode::Subtract => write!(f, "SUBTRACT"),
            OpCode::Return => write!(f, "RETURN"),
            OpCode::Constant(index) => write!(f, "CONSTANT({})", index),
            OpCode::True => write!(f, "TRUE"),
            OpCode::False => write!(f, "FALSE"),
            OpCode::Nil => write!(f, "NIL"),
            OpCode::Not => write!(f, "NOT"),
            OpCode::Equal => write!(f, "EQUAL"),
            OpCode::Greater => write!(f, "GREATER"),
            OpCode::Less => write!(f, "LESS"),
            OpCode::Print => write!(f, "PRINT"),
            OpCode::Pop => write!(f, "POP"),
            OpCode::DefineGlobal(index) => write!(f, "DEFINE_GLOBAL({})", index),
            OpCode::GetGlobal(index) => write!(f, "GET_GLOBAL({})", index),
            OpCode::SetGlobal(index) => write!(f, "SET_GLOBAL({})", index),
            OpCode::GetLocal(index) => write!(f, "GET_LOCAL({})", index),
            OpCode::SetLocal(index) => write!(f, "SET_LOCAL({})", index),
            OpCode::JumpIfFalse(index) => write!(f, "JUMP_IF_FALSE({})", index),
            OpCode::Jump(index) => write!(f, "JUMP({})", index),
            OpCode::Loop(index) => write!(f, "LOOP({})", index),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Add => write!(f, "+"),
            OpCode::Negate => write!(f, "-"),
            OpCode::Multiply => write!(f, "*"),
            OpCode::Divide => write!(f, "/"),
            OpCode::Subtract => write!(f, "-"),
            OpCode::Return => write!(f, "return"),
            OpCode::Constant(index) => write!(f, "{}", index),
            OpCode::True => write!(f, "true"),
            OpCode::False => write!(f, "false"),
            OpCode::Nil => write!(f, "nil"),
            OpCode::Not => write!(f, "not"),
            OpCode::Equal => write!(f, "equal"),
            OpCode::Greater => write!(f, "greater"),
            OpCode::Less => write!(f, "less"),
            OpCode::Print => write!(f, "print"),
            OpCode::Pop => write!(f, "pop"),
            OpCode::DefineGlobal(index) => write!(f, "{}", index),
            OpCode::GetGlobal(index) => write!(f, "{}", index),
            OpCode::SetGlobal(index) => write!(f, "{}", index),
            OpCode::GetLocal(index) => write!(f, "{}", index),
            OpCode::SetLocal(index) => write!(f, "{}", index),
            OpCode::JumpIfFalse(index) => write!(f, "{}", index),
            OpCode::Jump(index) => write!(f, "{}", index),
            OpCode::Loop(index) => write!(f, "{}", index),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Nil => write!(f, "Nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

impl Chunk {
    pub fn print(&self) {
        let mut offset: usize = 0;
        println!("== BYTECODE ==");
        for (index, code) in self.code.iter().enumerate() {
            if let OpCode::Constant(i) = code {
                println!(
                    "{offset:04} {} {code:?} {:->7} {}",
                    self.lines[index], "", self.constants[*i]
                );
                offset += 2
            } else {
                println!("{offset:04} {} {code:?}", self.lines[index]);
                offset += 1;
            }
        }
        println!("== END ==");
    }
}
