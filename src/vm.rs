use crate::chunk::{Chunk, Value, OpCode};
use crate::error::BessyError;

pub struct VM<'c> {
    chunk: &'c Chunk,
    ip: usize,
    stack: Vec<Value>,
}


impl Value {
    pub fn is_number(&self) -> bool {
        if let Value::Number(_) = self {
            true
        } else {
            false
        }
    }

    pub fn add(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
            _ => unreachable!(),
        }
    }

    pub fn subtract(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l - r),
            _ => unreachable!(),
        }
    }

    pub fn multiply(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l * r),
            _ => unreachable!(),
        }
    }

    pub fn divide(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(l), Value::Number(r)) => {
                Value::Number(r)
            }
            _ => unreachable!(),
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
                OpCode::Add | OpCode::Subtract |
                OpCode::Multiply | OpCode::Divide => {
                    let left = self.stack.pop().unwrap();
                    let right = self.stack.pop().unwrap();
                    if left.is_number() && right.is_number() {
                        let result = match instruction {
                            OpCode::Add => left.add(right),
                            OpCode::Subtract => right.subtract(left),
                            OpCode::Multiply => left.multiply(right),
                            OpCode::Divide => right.divide(left),
                            _ => unreachable!(),
                        };
                        self.stack.push(result);
                    } else {
                        return runtime_error!("Operands should be number.",
                                       self.chunk.lines[self.ip - 1]);
                    }
                }
                _ => todo!()
            }
        }
    }
}
        
