#![deny(rust_2018_idioms)]
#[macro_use]
mod error;
mod chunk;
mod compiler;
mod lexer;
mod token;

// code to compile project on buffer save
// (add-hook 'after-save-hook 'rust-compile)
// (setq compilation-scroll-output 'first-error)

use std::fs;
fn main() {
    let contents = fs::read_to_string("test/scan.lox").unwrap();
    // let contents = String::from("1 + ((-1 + -1) * 90)");
    // let mut scanner = lexer::Lexer::new(&contents);
    // for _i in 0..15 {
    //     println!("Token: {:?}", scanner.next_token());
    // }
    let mut compiler = compiler::Compiler::new(&contents);
    match compiler.compile() {
        Ok(c) => c.print(),
        Err(e) => eprintln!("{}", e),
    }
}
