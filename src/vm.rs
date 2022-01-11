use crate::chunk::{Chunk, Opcode};
use crate::value::Value;
use crate::compiler::{Parser};

// macro_rules! debug_instruction {
//     ( $chunk:ident, $instruction:expr ) => {{
//         use crate::debug;
//         for ele in &$chunk.stack {
//             print!("[ {} ]", ele);
//         }
//         println!();
//         debug::disassemble(&$chunk.chunk, &$chunk.ip - 1, $instruction);
//     }};
// }

#[derive(PartialEq, Debug)]
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

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut parser = Parser::new(source);
        if !parser.compile() {
            InterpretResult::CompileError
        } else {
            self.chunk = parser.chunk;
            self.ip = 0;
            self.run()
        }
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

    fn runtime_error(&self, message: &str) {
        eprintln!("[line {}] runtime error: {}",
                  self.chunk.lines[self.ip - 1], message)
    }

    fn peek(&self, distance: usize) -> Value {
        let top = self.stack.len() - 1;
        *self.stack.get(top - distance).expect("Tried to peek into empty stack.")
    }

    fn evaluate_binary(&mut self, operation: Opcode) -> Result<Value, ()> {
        if let (Value::Number(b), Value::Number(a)) = (self.peek(0), self.peek(1)) {
            let result = match operation {
                Opcode::Add => Value::Number(a + b),
                Opcode::Subtract => Value::Number(a - b),
                Opcode::Multiply => Value::Number(a * b),
                Opcode::Divide => Value::Number(a / b),
                _ => unreachable!(),
            };
            Ok(result)
        } else {
            Err(())
        }
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.next_instruction();
            // debug_instruction!(self, instruction);
            match instruction {
                Opcode::Return => {
                    println!("{}", self.stack.pop().unwrap());
                    return InterpretResult::Ok;
                }
                Opcode::Constant(index) => {
                    let constant_value = self.chunk.get_constant(index);
                    self.stack.push(constant_value);
                    continue;
                }
                Opcode::Nil => self.stack.push(Value::Nil),
                Opcode::True => self.stack.push(Value::Boolean(true)),
                Opcode::False => self.stack.push(Value::Boolean(false)),
                Opcode::Negate => {
                    match self.peek(0) {
                        Value::Number(top) => {
                            self.stack.push(Value::Number(-top));
                            continue;
                        }
                        _ => return InterpretResult::RuntimeError,
                    }
                }
                Opcode::Add | Opcode::Subtract |
                Opcode::Multiply | Opcode::Divide => {
                    match self.evaluate_binary(instruction) {
                        Ok(result) => {
                            self.stack.push(result);
                            continue;
                        }
                        Err(()) => {
                            self.runtime_error("Operands should be number.");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
            }
        }
    }
}
