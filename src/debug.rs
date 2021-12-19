use crate::chunk::{Chunk, Opcode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    for (offset, instruction) in chunk.code.iter().enumerate() {
        print!("{:04} ", offset);
        let curr_line = chunk.lines[offset];
        if offset > 0 && curr_line == chunk.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:>4} ", curr_line);
        }
        match instruction {
            Opcode::Return => println!("OP_RETURN"),
            Opcode::Constant(index) =>
                println!("OP_CONSTANT {}", chunk.get_constant(*index)),
        }
    }
    println!("== END ==");
}

pub fn print_instruction(chunk: &Chunk, instruction: &Opcode) {
    match instruction {
        Opcode::Return => println!("OP_RETURN"),
        Opcode::Constant(index) =>
            println!("OP_CONSTANT: {}", chunk.get_constant(*index)),
    }
}
