use crate::scanner::Scanner;
use crate::token::{Token, TokenType};

pub fn compile(source: &str) {
    let mut lexer = Scanner::init_scanner(source);
    let mut curr_line: u32 = 0;
    loop {
        let token = lexer.scan_token();
        if token.line != curr_line {
            print!("{:04} ", token.line);
            curr_line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:?} {}", token.kind, token.lexeme);

        if token.kind == TokenType::Eof {
            break;
        } else {
            continue;
        }
    }
}
