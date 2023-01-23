use std::iter::Peekable;
use std::str::CharIndices;

trait OptionCheck {
    fn check(&self, expected: char) -> bool;
}

impl OptionCheck for Option<char> {
    fn check(&self, expected: char) -> bool {
        match self {
            Some(c) => *c == expected,
            None => false,
        }
    }
}

#[derive(Clone, Debug)]
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
    Comment,

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
    Identifier(String),
    StrLit(String),
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
}

#[derive(Clone, Debug)]
pub struct Token {
    span: (usize, usize),
    token_type: TokenType,
}

impl Token {
    pub fn new(span: (usize, usize), token_type: TokenType) -> Self {
        Self { span, token_type }
    }
}

#[derive(Debug)]
pub enum LexError {
    UnterminatedString,
}

pub fn scan(text: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = vec![];
    let mut cursor = text.char_indices().peekable();
    while let Some(&(start_pos, c)) = cursor.peek() {
        let token = match c {
            '(' | ')' | '.' | '-' | '+' | '*' | ';' | '{' | '}' | ','
            | '/' => scan_single_token(&mut cursor),
            '~' => {
                scan_comment(&mut cursor);
                continue;
            }
            '!' | '=' | '>' | '<' => scan_double_token(&mut cursor),
            ' ' | '\r' | '\t' | '\n' => {
                let _ = cursor.next();
                continue
            },
            '"' => scan_string(&mut cursor)?,
            _ => {
                if c.is_ascii_digit() {
                    scan_number(&mut cursor, start_pos)
                } else if c.is_ascii_alphanumeric() || c == '_' {
                    scan_identifier(&mut cursor, start_pos)
                } else {
                    Token::new((start_pos, start_pos + c.len_utf8()), TokenType::Unknown)
                }
            }
        };
        tokens.push(token);
    }
    Ok(tokens)
}

fn scan_single_token(cursor: &mut Peekable<CharIndices>) -> Token {
    let (start_pos, c) = cursor.next().unwrap();
    let kind = match c {
        '(' => TokenType::LeftParen,
        ')' => TokenType::RightParen,
        '.' => TokenType::Dot,
        '-' => TokenType::Minus,
        '+' => TokenType::Plus,
        '*' => TokenType::Star,
        ';' => TokenType::Semicolon,
        '{' => TokenType::LeftBrace,
        '}' => TokenType::RightBrace,
        ',' => TokenType::Comma,
        '/' => TokenType::Slash,
        _ => unreachable!(),
    };
    Token::new((start_pos, start_pos + c.len_utf8()), kind)
}

fn scan_comment(cursor: &mut Peekable<CharIndices>) {
    for (_, c) in cursor {
        if c == '\n' {
            break;
        }
    }
}

fn scan_double_token(cursor: &mut Peekable<CharIndices>) -> Token {
    let (start_pos, c) = cursor.next().unwrap();
    match c {
        '!' => {
            if let Some((end_pos, _)) = cursor.next_if(|x| x.1 == '=') {
                Token::new((start_pos, end_pos), TokenType::BangEqual)
            } else {
                Token::new(
                    (start_pos, start_pos + c.len_utf8()),
                    TokenType::Bang,
                )
            }
        }
        '=' => {
            if let Some((end_pos, _)) = cursor.next_if(|x| x.1 == '=') {
                Token::new((start_pos, end_pos), TokenType::EqualEqual)
            } else {
                Token::new(
                    (start_pos, start_pos + c.len_utf8()),
                    TokenType::Equal,
                )
            }
        }
        '>' => {
            if let Some((end_pos, _)) = cursor.next_if(|x| x.1 == '=') {
                Token::new((start_pos, end_pos), TokenType::GreaterEqual)
            } else {
                Token::new(
                    (start_pos, start_pos + c.len_utf8()),
                    TokenType::Greater,
                )
            }
        }
        '<' => {
            if let Some((end_pos, _)) = cursor.next_if(|x| x.1 == '=') {
                Token::new((start_pos, end_pos), TokenType::LessEqual)
            } else {
                Token::new(
                    (start_pos, start_pos + c.len_utf8()),
                    TokenType::Less,
                )
            }
        }
        _ => unreachable!(),
    }
}

fn scan_string(
    cursor: &mut Peekable<CharIndices>,
) -> Result<Token, LexError> {
    let mut lexeme = String::from("");
    let (start_pos, _) = cursor.next().unwrap(); // skip opening quotes
    let start = start_pos + 1;
    while let Some((_, ch)) = cursor.next_if(|x| x.1 != '"') {
        lexeme.push(ch);
    }
    if cursor.peek().map_or(false, |x| x.1 == '"') {
        let _ = cursor.next();
        Ok(Token::new(
            (start, start + lexeme.len()),
            TokenType::StrLit(lexeme),
        ))
    } else {
        Err(LexError::UnterminatedString)
    }
}

fn scan_number(cursor: &mut Peekable<CharIndices>, start_pos: usize) -> Token {
    let mut lexeme = String::from("");
    while let Some((_, num)) = cursor.next_if(|x| x.1.is_ascii_digit()) {
        lexeme.push(num);
    }
    if cursor.peek().map_or(false, |x| x.1 == '.') {
        lexeme.push('.');
        let _ = cursor.next();
        while let Some((_, num)) = cursor.next_if(|x| x.1.is_ascii_digit()) {
            lexeme.push(num);
        }
    }
    let num= &lexeme.parse::<f64>().expect("Unable to parse number.");
    Token::new((start_pos, start_pos + lexeme.len()), TokenType::Number(*num))
}

fn scan_identifier(
    cursor: &mut Peekable<CharIndices>,
    start_pos: usize,
) -> Token {
    let mut lexeme = String::from("");
    while let Some((_, ch)) = cursor.next_if(|x| x.1.is_ascii_alphanumeric()) {
        lexeme.push(ch);
    }
    let len = lexeme.len();
    let kind = match lexeme.as_str() {
        "and"    => TokenType::And,
        "else"   => TokenType::Else,
        "false"  => TokenType::False,
        "for"    => TokenType::For,
        "fun"    => TokenType::Fun,
        "if"     => TokenType::If,
        "nil"    => TokenType::Nil,
        "or"     => TokenType::Or,
        "print"  => TokenType::Print,
        "return" => TokenType::Return,
        "true"   => TokenType::True,
        "var"    => TokenType::Var,
        "while"  => TokenType::While,
        _ => TokenType::Identifier(lexeme),
    };
    Token::new((start_pos, start_pos + len), kind)
}
