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
    program.write_opcode(Opcode::Constant(constant), 121);
    program.write_opcode(Opcode::Return, 123);
    // FIXME: Cloning for now but I should pass a borrow with
    // lifetime parameter to make it more efficient
    let mut vm = vm::VM::new();
    vm.interpret(program.clone());
    // debug::disassemble_chunk(&program, "test chunk");
}
