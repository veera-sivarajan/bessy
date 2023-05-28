mod expr;
mod lexer;
mod stmt;

pub fn evaluate(text: &str) {
    let mut lex = lexer::Lexer::new(text);
    let tokens = lex.scan().unwrap();
    println!("Tokens: {tokens:?}");
}
