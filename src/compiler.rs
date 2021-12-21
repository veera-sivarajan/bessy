use crate::scanner::Scanner;
use crate::token::{Token, TokenType};

pub fn compile(source: &str) {
    let lexer = Scanner::init_scanner(source);
    let mut curr_line: i32 = -1;
    loop {
        let token = lexer.scan_token();
        if token.line != curr_line {
            print!("{:04} ", token.line);
            curr_line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:02} {}", token.kind, token.start);

        if token.kind == TokenType::Eof {
            break;
        } else {
            continue;
        }
    }
}
