use crate::chunk::{Chunk, OpCode, Value};
use crate::error::BessyError;
use std::collections::HashMap;
use std::fmt::Write;

pub struct VM<'c> {
    chunk: &'c mut Chunk,
    ip: usize,
    stack: Vec<Value>,
    globals: HashMap<usize, Value>,
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
            (Value::Nil, Value::Nil) => true,
            (Value::Number(a), Value::Number(b)) => *a == b,
            (Value::Bool(a), Value::Bool(b)) => *a == b,
            (Value::String(a), Value::String(b)) => *a == b,
            _ => false,
        }
    }
}

impl<'c> VM<'c> {
    pub fn new(chunk: &'c mut Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: Vec::new(),
            globals: HashMap::new(),
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

    // TODO should vm.run() really return a Result?
    // would it be more efficient if it was a function with no return values instead aka returning `()`
    // if there is an error, it will print the error message on screen and terminate the interpreter after setting the appropriate shell code
    #[allow(clippy::map_entry)]
    pub fn run(&mut self, output: &mut impl Write) -> Result<(), BessyError> {
        while self.ip < self.chunk.code.len() {
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
                OpCode::Return => {
                    assert!(self.stack.is_empty());
                    return Ok(());
                }
                OpCode::Jump(offset) => self.ip += offset as usize,
                OpCode::Loop(offset) => self.ip -= offset as usize,
                OpCode::JumpIfFalse(offset) => {
                    if self.peek(0).is_falsey() {
                        self.ip +=  offset as usize;
                    }
                }
                OpCode::Print => {
                    let value = self.pop();
                    if let Value::String(index) = value {
                        let mut newline_buf: [u8; 1] = [0; 1]; // newline character needs 1 byte
                        let newline_str = '\n'.encode_utf8(&mut newline_buf);
                        let result = self.chunk.strings.lookup(index);
                        writeln!(output, "{}", result);
                    } else {
                        let result = format!("{}\n", value);
                        writeln!(output, "{}", result);
                    }
                }
                OpCode::Pop => {
                    let _ = self.pop();
                }
                OpCode::DefineGlobal(index) => {
                    if let Value::String(str_index) = self.chunk.constants[index] {
                        let value = self.pop();
                        let _ = self.globals.insert(str_index, value);
                    } else {
                        unreachable!()
                    }
                }
                OpCode::GetGlobal(index) => {
                    if let Value::String(str_index) = self.chunk.constants[index] {
                        if let Some(value) = self.globals.get(&str_index) {
                            self.push(value.to_owned());
                        } else {
                            let msg = format!(
                                "Undefined variable '{}'.",
                                self.chunk.strings.lookup(str_index)
                            );
                            return runtime_error!(msg, self.chunk.lines[self.ip - 1]);
                        }
                    } else {
                        unreachable!()
                    }
                }
                OpCode::SetGlobal(index) => {
                    if let Value::String(str_index) = self.chunk.constants[index] {
                        if self.globals.contains_key(&str_index) {
                            // not popping the value here because assignment is an expression
                            let _ = self.globals.insert(str_index, self.peek(0).to_owned());
                        } else {
                            let msg = format!(
                                "Cannot assign to undefined variable '{}'.",
                                self.chunk.strings.lookup(str_index)
                            );
                            return runtime_error!(msg, self.chunk.lines[self.ip - 1]);
                        }
                    } else {
                        unreachable!()
                    }
                }
                OpCode::GetLocal(index) => self.push(self.stack[index]),
                OpCode::SetLocal(index) => self.stack[index] = *self.peek(0),
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
                                OpCode::Add => {
                                    let l = self.chunk.strings.lookup(*l);
                                    let r = self.chunk.strings.lookup(*r);
                                    let concat =
                                        self.chunk.strings.intern(format!("{}{}", l, r).as_ref());
                                    Value::String(concat)
                                }
                                _ => {
                                    return runtime_error!(
                                        "Unsupported operation for Strings.",
                                        self.chunk.lines[self.ip - 1]
                                    )
                                }
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
        dbg!();
        unreachable!()
    }
}
