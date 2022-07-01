#![deny(rust_2018_idioms)]
#[macro_use]
mod error;
mod chunk;
mod compiler;
mod lexer;
mod token;
mod vm;

// code to compile project on buffer save
// (add-hook 'after-save-hook 'rust-compile)
// (setq compilation-scroll-output 'first-error)

use std::fs;
fn main() {
    // let contents = fs::read_to_string("test/scan.lox").unwrap();
    let contents = String::from("1 + true");
    let mut compiler = compiler::Compiler::new(&contents);
    match compiler.compile() {
        Ok(c) => {
            c.print();
            let mut vm = vm::VM::new(c);
            if let Err(e) = vm.run() {
                eprintln!("{}", e);
            } else {
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
