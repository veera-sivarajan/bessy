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

pub fn evaluate(input: String) -> String {
    let mut compiler = compiler::Compiler::new(&input);
    match compiler.compile() {
        Ok(c) => {
            c.print();
            let mut vm = vm::VM::new(c);
            match vm.run() {
                Ok(output) => return output,
                Err(msg) => return msg.to_string(),
            }
        }
        Err(e) => return e.to_string(),
    }
}
