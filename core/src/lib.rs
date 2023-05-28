mod error;
mod expr;
mod lexer;
mod parser;
mod stmt;

pub fn evaluate(text: &str) {
    let mut lex = lexer::Lexer::new(text);
    let tokens = lex.scan().unwrap();
    println!("Tokens: {tokens:?}");
    let mut parser = parser::Parser::new(tokens.into_iter());
}
