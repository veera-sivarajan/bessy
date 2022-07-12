#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum TokenType<'a> {
    LeftParen,
    RightParen,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Semicolon,
    #[default]
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
    Identifier(&'a str),
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
    And,
    Or,
    Class,
    Super,
    This,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Token<'a> {
    pub kind: TokenType<'a>,
    pub line: u16,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType<'a>, line: u16) -> Self {
        Token { kind, line }
    }
}
