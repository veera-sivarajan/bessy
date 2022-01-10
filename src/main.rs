mod chunk;
mod debug;
mod value;
mod vm;
mod compiler;
mod scanner;
mod token;

use crate::vm::{InterpretResult, VM};
use std::{fs, env, io::{stdout, Write}};

fn get_input() -> String {
    let mut input = String::new();
    input.clear();
    print!(">> ");
    let _flush = stdout().flush();
    let _bytes_read = std::io::stdin().read_line(&mut input).unwrap();
    let _last_char = input.pop();

    input
}

fn run_file(path: &str, mut vm: VM) { 
    let file_string = fs::read_to_string(path)
        .expect("Source file cannot be read.");
    let result = vm.interpret(file_string.as_str()); // deref coericion
    match result {
        InterpretResult::CompileError => std::process::exit(65), 
        // InterpretResult::RuntimeError => std::process::exit(70), 
        InterpretResult::Ok => std::process::exit(0),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vm = VM::new(); 
    if args.len() == 1 {
        loop {
            let input = get_input();
            if input == "exit" {
                std::process::exit(0);
            } else if !input.is_empty() {
                // TODO
                vm.interpret(input.as_str()); // deref coercion 
            } else {
                continue;
            }
        }
    } else if args.len() == 2 {
        // TODO
        run_file(&args[1], vm);
    } else {
        eprintln!("Usage: bessy [path]");
        std::process::exit(64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number() {
        let mut vm = VM::new();
        assert_eq!(vm::InterpretResult::Ok, vm.interpret("1"));
    }

    #[test]
    fn binary() {
        let mut vm = VM::new();
        assert_eq!(vm::InterpretResult::Ok, vm.interpret("1 + 1"));
    }

    #[test]
    fn grouping() {
        let mut vm = VM::new();
        assert_eq!(vm::InterpretResult::Ok, vm.interpret("(1 + 1) + 1"));
    }

    #[test]
    fn unary() {
        let mut vm = VM::new();
        assert_eq!(vm::InterpretResult::Ok, vm.interpret("-1"));
    }

    #[test]
    fn parse_string_error() {
        let mut vm = VM::new();
        assert_eq!(vm::InterpretResult::CompileError, vm.interpret("\"hello\""));
    }
}
