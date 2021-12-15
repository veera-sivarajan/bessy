pub enum Opcode {
    Return,
    Add,
    Sub,
}

pub struct Chunk {
    pub code: Vec<Opcode>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: Vec::new() }
    }

    pub fn write(&mut self, instruction: Opcode) {
        self.code.push(instruction);
    }
}
