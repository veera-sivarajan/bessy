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

#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Nil,
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
}
