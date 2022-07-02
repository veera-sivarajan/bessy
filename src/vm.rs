use crate::chunk::{Chunk, OpCode, Value};
use crate::error::BessyError;

pub struct VM<'c> {
    chunk: &'c Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl Value {
    fn is_falsey(&self) -> bool {
        match self {
            Value::Nil => true,
            Value::Bool(b) => !b,
            _ => false,
        }
    }

    fn equal(&self, other: Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => *a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => *a == b,
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
        self.stack
            .get(self.stack.len() - (depth + 1))
            .expect("Tried to peek at an empty stack.")
    }

    pub fn run(&mut self) -> Result<Value, BessyError> {
        loop {
            let opcode = self.chunk.code[self.ip];
            self.ip += 1;
            match opcode {
                OpCode::Nil => self.push(Value::Nil),
                OpCode::True => self.push(Value::Bool(true)),
                OpCode::False => self.push(Value::Bool(false)),
                OpCode::Constant(index) => self.push(self.chunk.constants[index].clone()),
                OpCode::Negate => {
                    if let Value::Number(n) = self.pop() {
                        self.push(Value::Number(-n));
                    } else {
                        return runtime_error!(
                            "Operand to '-' should be of type number.",
                            self.chunk.lines[self.ip - 1]
                        );
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
                OpCode::Return => return Ok(self.pop()),
                OpCode::Add
                | OpCode::Subtract
                | OpCode::Multiply
                | OpCode::Divide
                | OpCode::Greater
                | OpCode::Less => {
                    match (self.peek(0), self.peek(1)) {
                        (Value::Number(r), Value::Number(l)) => {
                            let result = match opcode {
                                OpCode::Add => Value::Number(l + r),
                                OpCode::Subtract => Value::Number(l - r),
                                OpCode::Multiply => Value::Number(l * r),
                                OpCode::Divide => Value::Number(l / r),
                                OpCode::Greater => Value::Bool(l > r),
                                OpCode::Less => Value::Bool(l < r),
                                _ => unreachable!(),
                            };
                            self.pop(); // pop the operands
                            self.pop();
                            self.push(result);
                        }
                        (Value::String(r), Value::String(l)) => {
                            let result = match opcode {
                                OpCode::Add => Value::String(format!("{}{}",l, r)),
                                _ => return runtime_error!("Unsupported operation for Strings.", self.chunk.lines[self.ip - 1]),
                            };
                            self.pop();
                            self.pop();
                            self.push(result);
                        }
                        _ => {
                            let msg = format!("Operands to '{}' should be of type number.", opcode);
                            return runtime_error!(msg, self.chunk.lines[self.ip - 1]);
                        }
                    }
                }
            }
        }
    }
}
