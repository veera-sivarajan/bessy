use crate::chunk::{Chunk, Value, OpCode};
use crate::error::BessyError;

pub struct VM<'c> {
    chunk: &'c Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl<'c> VM<'c> {
    pub fn new(chunk: &'c Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), BessyError> {
        loop {
            let instruction = self.chunk.code[self.ip];
            self.ip += 1;
            match instruction {
                OpCode::Constant(index) => { 
                    self.stack.push(self.chunk.constants[index]);
                }
                OpCode::Negate => {
                    if let Some(Value::Number(n)) = self.stack.pop() {
                        self.stack.push(Value::Number(-n));
                    } else {
                        return runtime_error!("Operand to `-` should be a number.", self.chunk.lines[self.ip - 1]);
                    }
                }
                OpCode::Return => {
                    if let Some(v) = self.stack.pop() {
                        println!("Output: {}", v);
                        return Ok(());
                    } else {
                        return runtime_error!("Expected a operand.", self.chunk.lines[self.ip - 1]);
                    }
                }
                _ => todo!()
            }
        }
    }
}
        
