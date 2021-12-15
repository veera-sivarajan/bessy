mod chunk;
mod debug;
mod value;

use crate::chunk::{Chunk, Opcode};

fn main() {
    // println!("Hello, world!");
    let mut program = Chunk::new();
    program.write_chunk(Opcode::Return);
    program.write_chunk(Opcode::Add);
    program.write_chunk(Opcode::Sub);
    debug::disassemble_chunk(&program, "test chunk");
}
