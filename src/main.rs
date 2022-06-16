#[macro_use]
mod error;
mod lexer;
mod token;

use std::fs;

fn main() {
    let contents = fs::read_to_string("test/scan.lox").unwrap();
    // let contents = String::from("\"hello\"()");
    let mut scanner = lexer::Lexer::new(&contents);
    println!("Token: {:?}", scanner.next_token());
    println!("Token: {:?}", scanner.next_token());
    println!("Token: {:?}", scanner.next_token());
    println!("Token: {:?}", scanner.next_token());
    println!("Token: {:?}", scanner.next_token());
    println!("Token: {:?}", scanner.next_token());
    println!("Token: {:?}", scanner.next_token());
    println!("Token: {:?}", scanner.next_token());
}
