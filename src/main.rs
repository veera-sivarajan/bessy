mod chunk;
mod debug;
mod value;
mod vm;
mod compiler;

use crate::chunk::{Chunk, Opcode};
use crate::value::Value;
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
    let result = vm.interpret(file_string);
    match result {
        InterpretResult::CompileError => std::process::exit(65), 
        InterpretResult::RuntimeError => std::process::exit(70), 
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
            } else if input.len() > 0 {
                // TODO
                vm.interpret(input);
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
