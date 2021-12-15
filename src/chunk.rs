use crate::value::ValueArray;

pub enum Opcode {
    Return,
    Add,
    Sub,
}

pub struct Chunk {
    pub code: Vec<Opcode>,
    constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write_chunk(&mut self, instruction: Opcode) {
        self.code.push(instruction);
    }
}
