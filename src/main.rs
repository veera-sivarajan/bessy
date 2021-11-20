mod chunk;
use crate::chunk::Chunk;
use crate::chunk::Opcode;

fn disassemble_chunk(chunk: Chunk, name: &str) {
    println!("== {} ==", name);
    for (offset, instruction) in chunk.code.iter().enumerate() {
        print!("{} ", offset);
        print_instruction(instruction);
    }
    println!("== END ==");
}
    
fn print_instruction(instruction: &Opcode) {
    match instruction {
        Opcode::Return => println!("OP_RETURN"),
        Opcode::Add    => println!("OP_ADD"),
        Opcode::Sub    => println!("OP_SUB"),
    }
}

fn main() {
    // println!("Hello, world!");
    let mut program = Chunk::new();
    program.write(Opcode::Return);
    program.write(Opcode::Add);
    program.write(Opcode::Sub);
    disassemble_chunk(program, "test chunk");
}
