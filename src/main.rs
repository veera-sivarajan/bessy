#![deny(rust_2018_idioms)]
#[macro_use]
mod error;
mod chunk;
mod compiler;
mod debug;
mod lexer;
mod token;
mod vm;
mod strings;

// code to compile project on buffer save
// (add-hook 'after-save-hook 'rust-compile)
// (setq compilation-scroll-output 'first-error)

use std::io;

fn main() {
    use std::fs;
    let contents = fs::read_to_string("/home/veera/Projects/bessy/test/scan.lox").unwrap();
    // let contents = String::from("print \"\";");
    let mut compiler = compiler::Compiler::new(&contents);
    match compiler.compile() {
        Ok(c) => {
            c.print();
            let mut vm = vm::VM::new(c);
            let mut output = io::stdout();
            if let Err(e) = vm.run(&mut output) { 
                eprintln!("{}", e);
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    fn test(input: &str, expected: &str) {
        let mut compiler = compiler::Compiler::new(input);
        if let Ok(code) = compiler.compile() {
            let mut vm = vm::VM::new(code);
            let mut output_buf: Vec<u8> = Vec::new();
            if let Ok(_) = vm.run(&mut output_buf) {
                let expected_str = format!("{}\n", expected);
                assert_eq!(&output_buf, expected_str.as_bytes());
            } else {
                assert!(false)
            }
        } else {
            assert!(false)
        }
    }

    #[test]
    fn numbers() {
        test("print 1;", "1");
    }

    #[test]
    fn literals() {
        test("print true;", "true"); 
        test("print false;", "false");
        test("print nil;", "Nil"); 
    }

    #[test]
    fn expressions() {
        test("print 1 + 2;", "3");
        test("print !(5 - 4 > 3 * 2 == !nil);", "true");
        test("print !true;", "false"); 
    }

    #[test]
    fn strings() {
        test("print \"Hello, world!\";", "Hello, world!");
        test("print \"Hello, \" + \"world!\";", "Hello, world!");
    }
}
