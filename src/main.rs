#[macro_use]
mod error;
mod lexer;
mod token;


// code to compile project on buffer save 
// (add-hook 'after-save-hook 'rust-compile) 
// (setq compilation-scroll-output 'first-error) 

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
