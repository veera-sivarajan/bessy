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

    Number(f64),
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
pub struct Token {
    pub kind: TokenType,
    pub line: u16,
}

impl Token {
    pub fn new(kind: TokenType, line: u16) -> Self {
        Token { kind, line }
    }
}
