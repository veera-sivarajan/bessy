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

pub fn evaluate(input: String, output: &mut impl std::io::Write) {
    let mut compiler = compiler::Compiler::new(&input);
    match compiler.compile() {
        Ok(c) => {
            c.print();
            let mut vm = vm::VM::new(c);
            if let Err(e) = vm.run(output) {
                let _ = write!(output, "{}", e)
                    .expect("Unable to write to stdout.");
            }
        }
        Err(e) => {
            let _ = write!(output, "{}", e)
                .expect("Unable to write to stdout.");
        }
    }
}
