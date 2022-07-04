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


fn main() {
    use std::fs;
    let contents = fs::read_to_string("/home/veera/Projects/bessy/test/scan.lox").unwrap();
    // let contents = String::from("\"hello\"");
    let mut compiler = compiler::Compiler::new(&contents);
    match compiler.compile() {
        Ok(c) => {
            c.print();
            let mut vm = vm::VM::new(c);
            match vm.run() {
                Err(e) => eprintln!("{}", e),
                Ok(()) => {},
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::chunk::Value;

//     fn test(input: &str, expected: Value) -> bool {
//         let mut compiler = compiler::Compiler::new(input);
//         if let Ok(code) = compiler.compile() {
//             let mut vm = vm::VM::new(code);
//             let output = vm.run();
//             output.map_or(false, |v| v == expected)
//         } else {
//             false
//         }
//     }

//     #[test]
//     fn empty_input() {
//         // errors
//         assert!(!test("", Value::Number(1.0)));
//     }

//     #[test]
//     fn numbers() {
//         assert!(test("1", Value::Number(1.0)))
//     }

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
