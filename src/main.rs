#![deny(rust_2018_idioms)]
#[macro_use]
mod error;
mod chunk;
mod compiler;
mod debug;
mod lexer;
mod token;
mod vm;

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
            // println!("Output buffer: {:?}", output);
        }
        Err(e) => eprintln!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::Value;

    fn test(input: &str, expected: Value) {
        let mut compiler = compiler::Compiler::new(input);
        if let Ok(code) = compiler.compile() {
            let mut vm = vm::VM::new(code);
            let mut output_buf: Vec<u8> = Vec::new();
            let result = vm.run(&mut output_buf);
            let expected_str = format!("{}\n", expected);
            println!("Expected: {}", expected_str);
            assert_eq!(&output_buf, expected_str.as_bytes());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn empty_input() {
        // errors
        // assert!(!test("", Value::Number(1.0)));
        test("", Value::String("".to_owned()))
    }

    #[test]
    fn numbers() {
        test("print 1;", Value::Number(1.0))
    }
}

//     #[test]
//     fn literals() {
//         assert!(test("true", Value::Bool(true)));
//         assert!(test("false", Value::Bool(false)));
//         assert!(test("nil", Value::Nil));
//     }

//     #[test]
//     fn expressions() {
//         assert!(test("1 + 2", Value::Number(3.0)));
//         assert!(test("!(5 - 4 > 3 * 2 == !nil)", Value::Bool(true)));
//         assert!(test("!true", Value::Bool(false)));

//         // errors
//         assert!(!test("1 + true", Value::Number(1.0)));
//         assert!(!test("true > true", Value::Number(1.0)));
//     }

//     #[test]
//     fn unknown_chars() {
//         assert!(!test("`", Value::Number(1.0)))
//     }

//     #[test]
//     fn strings() {
//         assert!(test("\"Hello, world!\"", Value::String(String::from("Hello, world!"))));
//         assert!(test("\"Hello, \" + \"world!\"", Value::String(String::from("Hello, world!"))));
//     }
// }
