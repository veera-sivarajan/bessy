use crate::strings::IStrings;

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
    Print,
    Pop,
    DefineGlobal(usize),
    GetGlobal(usize),
    SetGlobal(usize),
    GetLocal(usize),
    SetLocal(usize),
}

#[derive(Clone, PartialEq, Copy)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(usize), // A String type will contain a index to it's stored location in IStrings::list
    Nil,
}

#[derive(Default)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<u16>,
    pub strings: IStrings,
}

impl Chunk {
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1 // index of added element
    }

    pub fn emit_byte(&mut self, code: OpCode, line: u16) -> usize {
        self.code.push(code);
        self.lines.push(line);
        self.code.len() - 1
    }
}
