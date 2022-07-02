use std::fmt;

#[derive(Copy, Clone)]
pub enum OpCode {
    Add,
    Negate,
    Subtract,
    Multiply,
    Divide,
    Return,
    Nil,
    True,
    False,
    Constant(usize), // usize holds the index to constants vector
    Not,
    Equal,
    Greater,
    Less,
}

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
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Nil => write!(f, "Nil"),
            Value::Bool(b) => {
                if *b {
                    write!(f, "True")
                } else {
                    write!(f, "False")
                }
            }
        }
    }
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<u16>,
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

    pub fn emit_byte(&mut self, code: OpCode, line: u16) {
        self.code.push(code);
        self.lines.push(line);
    }

    pub fn print(&self) {
        let mut offset: usize = 0;
        println!("== BYTECODE ==");
        for (index, code) in self.code.iter().enumerate() {
            if let OpCode::Constant(i) = code {
                println!(
                    "{offset:04} {} {code:?} {}",
                    self.lines[index], self.constants[*i]
                );
            } else {
                println!("{offset:04} {} {code:?}", self.lines[index]);
            }
            offset += 1;
        }
        println!("== END ==");
    }
}
