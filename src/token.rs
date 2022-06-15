#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Semicolon,
    Eof,
    LeftBrace,
    RightBrace,
    Comma,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Number(i64),
    True,
    False,
    Unknown,
    Identifier,
    StrLit,
    Print,
    Var,
    Nil,
    If,
    Else,
    While,
    For,
    Fun,
    Return,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub line: u16,
    pub lexeme: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType, line: u16, lexeme: &'a str) -> Self {
        Token { kind, line, lexeme }
    }
}
