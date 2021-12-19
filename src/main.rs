mod chunk;
mod debug;
mod value;

use crate::chunk::{Chunk, Opcode};
use crate::value::Value;

fn main() {
    // println!("Hello, world!");
    let mut program = Chunk::new();
    let constant = program.add_constant(Value::Number(1.2));
    program.write_opcode(Opcode::Constant(constant as u8), 123);
    program.write_opcode(Opcode::Return, 123);
    program.write_opcode(Opcode::Return, 123);
    program.write_opcode(Opcode::Return, 123);
    debug::disassemble_chunk(&program, "test chunk");
}
