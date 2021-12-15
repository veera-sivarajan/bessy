use crate::chunk::{Chunk, Opcode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    for (offset, instruction) in chunk.code.iter().enumerate() {
        print!("{} ", offset);
        print_instruction(instruction);
    }
    println!("== END ==");
}

pub fn print_instruction(instruction: &Opcode) {
    match instruction {
        Opcode::Return => println!("OP_RETURN"),
        Opcode::Add => println!("OP_ADD"),
        Opcode::Sub => println!("OP_SUB"),
    }
}
