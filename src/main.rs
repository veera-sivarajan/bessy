#![feature(if_let_guard)]
#![feature(let_chains)]
#[macro_use]
mod error;
mod token;
mod scanner;

use std::fs;

fn main() {
    let contents = fs::read_to_string("test/scan.lox").unwrap();
    // let contents = String::from("\"hello\"()");
    let mut scanner = scanner::Scanner::new(&contents);
    println!("{:?}", scanner.scan_token());
    println!("{:?}", scanner.scan_token());
    println!("{:?}", scanner.scan_token());
    println!("{:?}", scanner.scan_token());
}
