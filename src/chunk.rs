use std::fmt;

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum Value {
    Number(f64),
}

// impl Value {
//     pub fn is_number(&self) -> bool {
//         if let Value::Number(_) = self {
//             true
//         } else {
//             false
//         }
//     }

//     pub fn add(&self, other: Value) -> Value {
//         match (self, other) {
//             (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
//             _ => unreachable!(),
//         }
//     }
// }


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
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
                    "{offset:04} {} {code} {}",
                    self.lines[index], self.constants[*i]
                );
            } else {
                println!("{offset:04} {} {code}", self.lines[index]);
            }
            offset += 1;
        }
        println!("== END ==");
    }
}
