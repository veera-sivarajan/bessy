use crate::chunk::{Chunk, Opcode};
use crate::value::Value;

macro_rules! debug_instruction {
    ( $chunk:ident, $instruction:expr ) => {
        {
            use crate::debug;
            for ele in &$chunk.stack {
                print!("[ {} ]", ele);
            }
            println!();
            debug::disassemble_instruction(&$chunk.chunk, &$chunk.ip - 1, $instruction);  
        }
    };
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    ip: usize, // instruction pointer points to next instruction to be interpreted
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::with_capacity(256),
        }
    }
    
    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
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
            let instruction = self.next_instruction();
            debug_instruction!(self, instruction);
            match instruction {
                Opcode::Return => {
                    println!("{}", self.stack.pop().unwrap());
                    return InterpretResult::Ok;
                }
                Opcode::Constant(index) => {
                    let constant_value = self.chunk.get_constant(index);
                    self.stack.push(constant_value);
                    continue;
                },
            }
        }
    }
}
