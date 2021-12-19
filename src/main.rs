mod chunk;
mod debug;
mod value;
mod vm;

use crate::chunk::{Chunk, Opcode};
use crate::value::Value;

fn main() {
    // println!("Hello, world!");
    let mut program = Chunk::new();
    let constant = program.add_constant(Value::Number(1.2));
    program.write_opcode(Opcode::Constant(constant), 123);
    program.write_opcode(Opcode::Return, 123);
    program.write_opcode(Opcode::Return, 123);
    program.write_opcode(Opcode::Return, 123);

    let mut vm = vm::VM::new(program.clone(), 0);
    vm.interpret(program);
    // debug::disassemble_chunk(&program, "test chunk");
}
