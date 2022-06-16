#[derive(Debug)]
pub enum TokenType<'a> {
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

    Number(f64),
    True,
    False,
    Unknown,
    Identifier,
    StrLit(&'a str),
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
    pub kind: TokenType<'a>,
    pub line: u16,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType<'a>, line: u16) -> Self {
        Token { kind, line }
    }
}
