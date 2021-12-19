use crate::chunk::{Chunk, Opcode};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    ip: usize, // instruction pointer
}

impl VM {
    pub fn new(chunk: Chunk, ip: usize) -> VM {
        VM { chunk, ip }
    }
    
    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        // self.chunk = chunk;
        // self.ip = 0;
        self.run()
    }

    fn next_index(&mut self) -> usize {
        let curr_index = self.ip;
        self.ip += 1;
        curr_index
    }

    fn next_instruction(&mut self) -> Opcode {
        let index = self.next_index();
        self.chunk.code[index]
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            match self.next_instruction() {
                Opcode::Return => return InterpretResult::Ok,
                Opcode::Constant(index) => {
                    println!("{}", self.chunk.get_constant(index));
                    continue;
                },
            }
        }
    }
}
