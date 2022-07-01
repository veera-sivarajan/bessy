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

    fn is_falsey(&self) -> bool {
        match self {
            Value::Nil => true,
            Value::Bool(b) => !b,
            _ => false,
        }
    }

    fn equal(&self, other: Value) -> bool {
        match (*self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            _ => false,
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

    fn peek(&self, depth: usize) -> &Value {
        self.stack.get(self.stack.len() - (depth + 1)).expect("Tried to peek at an empty stack.")
    }

    pub fn run(&mut self) -> Result<(), BessyError> {
        loop {
            let opcode = self.chunk.code[self.ip];
            self.ip += 1;
            match opcode {
                OpCode::Nil => self.push(Value::Nil),
                OpCode::True => self.push(Value::Bool(true)),
                OpCode::False => self.push(Value::Bool(false)),
                OpCode::Constant(index) => self.push(self.chunk.constants[index]),
                OpCode::Negate => {
                    if let Value::Number(n) = self.pop() {
                        self.push(Value::Number(-n));
                    } else {
                        return runtime_error!("Operand to `-` should be a number.", self.chunk.lines[self.ip - 1]);
                    }
                }
                OpCode::Not => {
                    let value = self.pop().is_falsey();
                    self.push(Value::Bool(value));
                }
                OpCode::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = a.equal(b);
                    self.push(Value::Bool(result));
                }
                OpCode::Return => {
                    let v = self.pop();
                    println!("{}", v);
                    return Ok(());
                }
                OpCode::Add | OpCode::Subtract |
                OpCode::Multiply | OpCode::Divide => {
                    let left = self.peek(0).is_number(); // convert Value::Number(n) to Some(n) to use zip
                    let right = self.peek(0).is_number();
                    if let Some((l, r)) = left.zip(right) { // using zip because if let chains are unstable
                        self.pop();
                        self.pop();
                        let result = match opcode {
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
        
