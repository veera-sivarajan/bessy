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
    let contents = String::from("!(5 - 4 > 3 * 2 == !nil)");
    let mut compiler = compiler::Compiler::new(&contents);
    match compiler.compile() {
        Ok(c) => {
            c.print();
            let mut vm = vm::VM::new(c);
            match vm.run() {
                Err(e) => eprintln!("{}", e),
                Ok(o) => println!("{}", o),
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &str, expected: chunk::Value) -> bool {
        let mut compiler = compiler::Compiler::new(input);
        let code = compiler.compile();
        assert!(code.is_ok());
        let mut vm = vm::VM::new(code.unwrap());
        let output = vm.run();
        output.map_or(false, |v| v == expected)
    }
    
    #[test]
    fn numbers() {
        assert!(test("1", chunk::Value::Number(1.0)))
    }

    #[test]
    fn literals() {
        assert!(test("true", chunk::Value::Bool(true)));
        assert!(test("false", chunk::Value::Bool(false)));
        assert!(test("nil", chunk::Value::Nil));
    }

    #[test]
    fn expression() {
        assert!(test("!(5 - 4 > 3 * 2 == !nil)", chunk::Value::Bool(true)));
    }
}
