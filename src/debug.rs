use crate::chunk::{Chunk, Opcode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    for (offset, instruction) in chunk.code.iter().enumerate() {
        disassemble_instruction(chunk, offset, *instruction);
    }
    println!("== END ==");
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize, instruction: Opcode) {
    print_offset(offset, chunk);
    match instruction {
        Opcode::Return => println!("OP_RETURN"),
        Opcode::Constant(index) => print_constant("OP_CONSTANT", index, chunk),
        Opcode::Negate => println!("OP_NEGATE"),
        Opcode::Add => println!("OP_ADD"),
        Opcode::Subtract => println!("OP_SUBTRACT"),
        Opcode::Multiply => println!("OP_MULTIPLY"),
        Opcode::Divide => println!("OP_DIVIDE"),
    }
}

fn print_constant(name: &str, index: usize, chunk: &Chunk) {
    print!("{} {:04} '", name, index);
    println!("{}'", chunk.get_constant(index));
}

fn print_offset(offset: usize, chunk: &Chunk) {
    print!("{:04} ", offset);
    let curr_line = chunk.lines[offset];
    if offset > 0 && curr_line == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:>4} ", curr_line);
    }
}
