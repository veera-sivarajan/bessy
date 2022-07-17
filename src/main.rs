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

// code to compile project on buffer save
// (add-hook 'after-save-hook 'rust-compile)
// (setq compilation-scroll-output 'first-error)

use std::io;

fn main() {
    use std::fs;
    let contents = fs::read_to_string("/home/veera/Projects/bessy/test/main.lox").unwrap();
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

    fn test(input: &str, expected: &str) -> bool {
        let mut compiler = compiler::Compiler::new(input);
        if let Ok(code) = compiler.compile() {
            let mut vm = vm::VM::new(code);
            let mut output_buf: Vec<u8> = Vec::new();
            if let Ok(_) = vm.run(&mut output_buf) {
                // assert_eq!(&output_buf, expected.as_bytes());
                &output_buf == expected.as_bytes()
            } else {
                false
            }
        } else {
            false
        }
    }

    #[test]
    fn numbers() {
        assert!(test("print 1;", "1\n"));
        assert!(test("print 10000000;", "10000000\n"));
        assert!(test("100;", ""));
    }

    #[test]
    fn literals() {
        assert!(test("true;", ""));
        assert!(test("false;", ""));
        assert!(test("print true;", "true\n"));
        assert!(test("print false;", "false\n"));
        assert!(test("print nil;", "Nil\n"));
    }

    #[test]
    fn expressions() {
        assert!(test("print 1 + 2;", "3\n"));
        assert!(test("print !(5 - 4 > 3 * 2 == !nil);", "true\n"));
        assert!(test("print !true;", "false\n"));
        assert!(test("1 + 1;", ""));
    }

    #[test]
    fn strings() {
        assert!(test("print \"Hello, world!\";", "Hello, world!\n"));
        assert!(test("print \"Hello, \" + \"world!\";", "Hello, world!\n"));
        assert!(test("\"billa\";", ""));
    }

    #[test]
    fn local_variables() {
        use std::fs;
        let paths = [
            "/home/veera/Projects/bessy/test/scope.lox",
            "/home/veera/Projects/bessy/test/scope-1.lox",
        ];
        let outputs = ["3\n2\n1\n", "global\n2\n3\n4\nglobal\n"];
        for (file, result) in paths.iter().zip(outputs.iter()) {
            let input = fs::read_to_string(file).expect("File not found.");
            assert!(test(input.as_str(), result));
        }
    }
}
