use crate::chunk::{Chunk, Value, OpCode};
use crate::error::BessyError;

pub struct VM<'c> {
    chunk: &'c Chunk,
    ip: usize,
    stack: Vec<Value>,
}


impl Value {
    pub fn is_number(&self) -> Option<f64> {
        if let Value::Number(n) = self {
            Some(*n) 
        } else {
            None
        }
    }
}

impl<'c> VM<'c> {
    pub fn new(chunk: &'c Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Tried to pop an empty stack.")
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn run(&mut self) -> Result<(), BessyError> {
        loop {
            let instruction = self.chunk.code[self.ip];
            self.ip += 1;
            match instruction {
                OpCode::Constant(index) => { 
                    self.push(self.chunk.constants[index]);
                }
                OpCode::Negate => {
                    if let Value::Number(n) = self.pop() {
                        self.push(Value::Number(-n));
                    } else {
                        return runtime_error!("Operand to `-` should be a number.", self.chunk.lines[self.ip - 1]);
                    }
                }
                OpCode::Return => {
                    let v = self.pop();
                    println!("{}", v);
                    return Ok(());
                }
                OpCode::Add | OpCode::Subtract |
                OpCode::Multiply | OpCode::Divide => {
                    let left = self.pop().is_number();
                    let right = self.pop().is_number();
                    if let Some((l, r)) = left.zip(right) { // using zip because if let chains are unstable
                        let result = match instruction {
                            OpCode::Add => l + r,
                            OpCode::Subtract => r - l,
                            OpCode::Multiply => l * r,
                            OpCode::Divide => r / l,
                            _ => unreachable!(),
                        };
                        self.push(Value::Number(result));
                    } else {
                        return runtime_error!("Operands should be number.", self.chunk.lines[self.ip - 1]);
                    }
                }
                _ => todo!()
            }
        }
    }
}
        
