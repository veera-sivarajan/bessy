use std::fmt;

pub enum OpCode {
    Add,
    Negate,
    Subtract,
    Multiply,
    Divide,
    Return,
    Constant(usize), // usize holds the index to constants vector
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Add => write!(f, "ADD"),
            OpCode::Negate => write!(f, "NEGATE"),
            OpCode::Multiply => write!(f, "MULTIPLY"),
            OpCode::Divide => write!(f, "DIVIDE"),
            OpCode::Subtract => write!(f, "SUBTRACT"),
            OpCode::Return => write!(f, "RETURN"),
            OpCode::Constant(index) => write!(f, "CONSTANT({})", index),
        }
    }
}

pub enum Value {
    Number(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}

pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1 // index of added element
    }

    pub fn emit_byte(&mut self, code: OpCode) {
        self.code.push(code);
    }

    pub fn print(&self) {
        let mut offset: usize = 0;
        for i in &self.code {
            if let OpCode::Constant(index) = i {
                println!("{:04} {} {}", offset, i, self.constants[*index]);
            } else {
                println!("{:04} {}", offset, i);
            }
            offset += 1;
        }
    }
}
