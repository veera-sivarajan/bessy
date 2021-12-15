mod chunk;
mod debug;

use crate::chunk::{Chunk, Opcode};

fn main() {
    // println!("Hello, world!");
    let mut program = Chunk::new();
    program.write(Opcode::Return);
    program.write(Opcode::Add);
    program.write(Opcode::Sub);
    debug::disassemble_chunk(&program, "test chunk");
}
