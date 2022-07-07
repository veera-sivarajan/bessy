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
                assert_eq!(&output_buf, expected.as_bytes());
            } else {
                assert!(false)
            }
        } else {
            assert!(false)
        }
    }

    #[test]
    fn numbers() {
        test("print 1;", "1\n");
        test("print 10000000;", "10000000\n");
        test("100;", "");
    }

    #[test]
    fn literals() {
        test("true;", "");
        test("false;", "");
        test("print true;", "true\n"); 
        test("print false;", "false\n");
        test("print nil;", "Nil\n"); 
    }

    #[test]
    fn expressions() {
        test("print 1 + 2;", "3\n");
        test("print !(5 - 4 > 3 * 2 == !nil);", "true\n");
        test("print !true;", "false\n"); 
        test("1 + 1;", "");
    }

    #[test]
    fn strings() {
        test("print \"Hello, world!\";", "Hello, world!\n");
        test("print \"Hello, \" + \"world!\";", "Hello, world!\n");
        test("\"billa\";", "");
    }
}
