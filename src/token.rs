#[derive(PartialEq, Debug, Eq, Hash, Copy, Clone)]
pub enum TokenType {
    // Single Character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    True,
    Var,
    While,

    Error,
    Eof,
}

#[derive(Copy, Clone)]
pub struct Token<'src> {
    pub kind: TokenType,
    pub lexeme: &'src str,
    pub line: usize,
}

impl<'src> Default for Token<'src> {
    fn default() -> Self {
        Token {
            kind: TokenType::Eof,
            lexeme: "",
            line: 0,
        }
    }
}


