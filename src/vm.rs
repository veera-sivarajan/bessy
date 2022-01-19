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
        self.stack[top - distance].clone()
    }

    fn evaluate_binary(&mut self, operation: Opcode) -> Result<Value, &'static str> {
        if let (Value::Number(b), Value::Number(a)) = (self.peek(0), self.peek(1)) {
            self.stack.pop();
            self.stack.pop();
            match operation {
                Opcode::Add => Ok(Value::Number(a + b)),
                Opcode::Subtract => Ok(Value::Number(a - b)),
                Opcode::Multiply => Ok(Value::Number(a * b)),
                Opcode::Divide => Ok(Value::Number(a / b)),
                Opcode::Greater => Ok(Value::Boolean(a > b)),
                Opcode::Less => Ok(Value::Boolean(a < b)),
                _ => Err("Unknown binary operation for numbers."), 
            }
        } else if let (Value::String(b), Value::String(a)) = (self.peek(0), self.peek(1)) {
            self.stack.pop();
            self.stack.pop();
            match operation {
                Opcode::Add => Ok(Value::String(format!("{}{}", a, b))),
                _ => Err("Unknown binary operation for strings."),
            }
        } else {
            Err("Operands should be of type numbers or strings.")
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
                }
                Opcode::Nil => self.stack.push(Value::Nil),
                Opcode::True => self.stack.push(Value::Boolean(true)),
                Opcode::False => self.stack.push(Value::Boolean(false)),
                Opcode::Not => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(value.is_falsey()));
                }
                Opcode::Negate => if let Value::Number(top) = self.peek(0) {
                    self.stack.pop();
                    self.stack.push(Value::Number(-top));
                } else {
                    let error_message = format!("{:?} type cannot be negated.", self.peek(0));
                    self.runtime_error(error_message.as_str());
                    return InterpretResult::RuntimeError;
                }
                Opcode::Equal => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(a == b));
                }
                Opcode::Add | Opcode::Subtract | Opcode::Greater |
                Opcode::Less| Opcode::Multiply | Opcode::Divide => match self.evaluate_binary(instruction) {
                    Ok(result) => self.stack.push(result),
                    Err(message) => {
                        self.runtime_error(message);
                        return InterpretResult::RuntimeError;
                    }
                }
            }
        }
    }
}
