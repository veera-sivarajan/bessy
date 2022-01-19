use crate::value::Value;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Opcode {
    Constant(usize), // Constant opcode followed by index to the constant
    Return,
    Nil,
    True,
    False,
    // Binary operators
    Add,
    Subtract,
    Multiply,
    Divide,
    // Comparison operators
    Equal,
    Greater,
    Less,

    // unary operators
    Negate,
    Not,

    // statements
    Print,
}

#[derive(Clone)]
pub struct Chunk {
    pub code: Vec<Opcode>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_opcode(&mut self, instruction: Opcode, line: usize) {
        self.code.push(instruction);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, index: usize) -> Value {
        self.constants[index].clone()
    }
}
