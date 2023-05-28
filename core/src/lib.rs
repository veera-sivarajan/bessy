mod expr;
mod lexer;
mod parser;
mod stmt;

pub fn evaluate(text: &str) {
    let mut lex = lexer::Lexer::new(text);
    let tokens = lex.scan().unwrap();
    let mut parser = parser::Parser::new(&tokens);
    println!("Tokens: {tokens:?}");
}
