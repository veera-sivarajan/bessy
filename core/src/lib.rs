mod lexer;
mod expr;
mod stmt;

pub fn evaluate(text: &str) {
    let mut lex = lexer::Lexer::new(text);
    let tokens = lex.scan().unwrap();
    println!("Tokens: {tokens:?}");
}
