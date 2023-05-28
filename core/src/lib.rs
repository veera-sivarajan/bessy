mod error;
mod expr;
mod lexer;
mod parser;
mod stmt;

use crate::error::BessyError;

pub fn evaluate(text: &str) -> Result<(), BessyError> {
    let mut lex = lexer::Lexer::new(text);
    let tokens = lex.scan()?;
    println!("Tokens: {tokens:?}");
    let mut parser = parser::Parser::new(tokens.into_iter());
    let ast = parser.parse()?;
    println!("AST: {ast:?}");
    Ok(())
}
