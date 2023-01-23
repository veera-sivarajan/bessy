mod lexer;

pub fn evaluate(text: &str) {
    let tokens = lexer::scan(text).unwrap();
    println!("Tokens: {tokens:?}");
}
