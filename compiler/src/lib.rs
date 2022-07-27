#![deny(rust_2018_idioms)]
#[macro_use]
mod error;
mod chunk;
mod compiler;
mod debug;
mod lexer;
mod strings;
mod token;
mod vm;

use std::fmt::Write;

pub fn evaluate(input: String, output: &mut impl Write) {
    let mut compiler = compiler::Compiler::new(&input);
    match compiler.compile() {
        Ok(c) => {
            c.print();
            let mut vm = vm::VM::new(c);
            if let Err(e) = vm.run(output) {
                write!(output, "{}", e);
            }
        }
        Err(e) => {
            write!(output, "{}", e);
        }
    }
}
