use crate::error::BessyError;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Semicolon,
    LeftBrace,
    RightBrace,
    Comma,
    Percent,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Number(f64),
    Boolean(bool),
    Unknown,
    Identifier(String),
    StringLiteral(String),
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

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use TokenType::*;
        match self {
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            Dot => write!(f, "."),
            Minus => write!(f, "-"),
            Plus => write!(f, "+"),
            Star => write!(f, "*"),
            Semicolon => write!(f, ";"),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            Comma => write!(f, ","),
            Percent => write!(f, "%"),
            Slash => write!(f, "/"),
            Bang => write!(f, "!"),
            BangEqual => write!(f, "!="),
            Equal => write!(f, "="),
            EqualEqual => write!(f, "=="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            Number(num) => write!(f, "number: {num}"),
            Boolean(value) => write!(f, "boolean: {value}"),
            Unknown => write!(f, "unknown token"),
            Identifier(name) => write!(f, "identifier: {name}"),
            StringLiteral(lexeme) => write!(f, "string: {lexeme}"),
            Print => write!(f, "keyword print"),
            Var => write!(f, "keyword var"),
            Nil => write!(f, "keyword nil"),
            If => write!(f, "keyword if"),
            Else => write!(f, "keyword else"),
            While => write!(f, "keyword while"),
            For => write!(f, "keyword for"),
            Fun => write!(f, "keyword fun"),
            Return => write!(f, "keyword return"),
            And => write!(f, "keyword and"),
            Or => write!(f, "keyword or"),
        }
    }
}
    

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenType,
    pub span: Span,
}

#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub line_number: u16,
    pub column_number: u16,
}

impl Token {
    pub const fn new(kind: TokenType, span: Span) -> Self {
        Self { kind, span }
    }

    pub const fn is_identifier(&self) -> bool {
        matches!(self.kind, TokenType::Identifier(_))
    }
}

pub struct Lexer<'src> {
    cursor: Peekable<CharIndices<'src>>,
    tokens: Vec<Token>,
    line: u16,
    column: u16,
    start_of_line: u16,
}

impl<'src> Lexer<'src> {
    pub fn new(text: &'src str) -> Self {
        Self {
            cursor: text.char_indices().peekable(),
            line: 1,
            column: 0,
            start_of_line: 0,
            tokens: Vec::with_capacity(text.len()),
        }
    }

    fn make_span(&mut self, start: usize, end: usize) -> Span {
        let end = start + end;
        let start_column = self.column(start);
        let end_column = self.column(end);

        let start = Position {
            line_number: self.line,
            column_number: start_column,
        };

        let end = Position {
            line_number: self.line,
            column_number: end_column,
        };

        Span { start, end }
    }

    fn next_line(&mut self, index: usize) {
        self.line += 1;
        self.start_of_line = (index + 1) as u16;
    }

    fn column(&mut self, start_pos: usize) -> u16 {
        self.column = start_pos as u16 - self.start_of_line;
        self.column
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, BessyError> {
        while let Some(&(start_pos, c)) = self.cursor.peek() {
            match c {
                '(' | ')' | '.' | '-' | '+' | '*' | ';' | '{' | '}' | ','
                | '/' | '%' => self.scan_single_token(),
                '~' => self.scan_comment(),
                '!' | '=' | '>' | '<' => self.scan_double_token(),
                ' ' | '\r' | '\t' => {
                    self.cursor.next();
                }
                '\n' => {
                    let (index, _) = self.cursor.next().unwrap();
                    self.next_line(index);
                }
                '"' => {
                    let token = self.scan_string()?;
                    self.tokens.push(token);
                }
                _ => {
                    if c.is_ascii_digit() {
                        self.scan_number(start_pos)
                    } else if c.is_ascii_alphanumeric() || c == '_' {
                        self.scan_identifier(start_pos);
                    } else {
                        self.cursor.next();
                        let span = self.make_span(start_pos, c.len_utf8());
                        self.tokens.push(Token::new(TokenType::Unknown, span));
                    }
                }
            }
        }
        Ok(self.tokens.clone())
    }

    fn scan_single_token(&mut self) {
        let (start_pos, c) = self.cursor.next().unwrap();
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
            '%' => TokenType::Percent,
            _ => unreachable!(),
        };
        let span = self.make_span(start_pos, c.len_utf8());
        self.tokens.push(Token::new(kind, span));
    }

    fn scan_comment(&mut self) {
        for (index, ch) in self.cursor.by_ref() {
            if ch == '\n' {
                self.next_line(index);
                break;
            }
        }
    }

    fn check_next(
        &mut self,
        start_pos: usize,
        len: usize,
        this: TokenType,
        that: TokenType,
    ) -> Token {
        if let Some((_, ch)) = self.cursor.next_if(|x| x.1 == '=') {
            Token::new(this, self.make_span(start_pos, ch.len_utf8()))
        } else {
            Token::new(that, self.make_span(start_pos, len))
        }
    }

    fn scan_double_token(&mut self) {
        let (start_pos, c) = self.cursor.next().unwrap();
        let token = match c {
            '!' => self.check_next(
                start_pos,
                c.len_utf8(),
                TokenType::BangEqual,
                TokenType::Bang,
            ),
            '=' => self.check_next(
                start_pos,
                c.len_utf8(),
                TokenType::EqualEqual,
                TokenType::Equal,
            ),
            '>' => self.check_next(
                start_pos,
                c.len_utf8(),
                TokenType::GreaterEqual,
                TokenType::Greater,
            ),
            '<' => self.check_next(
                start_pos,
                c.len_utf8(),
                TokenType::LessEqual,
                TokenType::Less,
            ),
            _ => unreachable!(),
        };
        self.tokens.push(token);
    }

    fn scan_string(&mut self) -> Result<Token, BessyError> {
        let mut lexeme = String::from("");
        let (start_pos, _) = self.cursor.next().unwrap(); // skip opening quotes
        let start = start_pos + 1;
        while let Some((_, ch)) = self.cursor.next_if(|x| x.1 != '"') {
            lexeme.push(ch);
        }
        let length = lexeme.len();
        if self.cursor.peek().map_or(false, |x| x.1 == '"') {
            let _ = self.cursor.next();
            Ok(Token::new(
                TokenType::StringLiteral(lexeme),
                self.make_span(start, length),
            ))
        } else {
            Err(BessyError::UnterminatedString(
                self.make_span(start, length),
            ))
        }
    }

    fn scan_number(&mut self, start_pos: usize) {
        let mut lexeme = String::from("");
        while let Some((_, num)) = self.cursor.next_if(|x| x.1.is_ascii_digit())
        {
            lexeme.push(num);
        }
        if self.cursor.peek().map_or(false, |x| x.1 == '.') {
            lexeme.push('.');
            let _ = self.cursor.next();
            while let Some((_, num)) =
                self.cursor.next_if(|x| x.1.is_ascii_digit())
            {
                lexeme.push(num);
            }
        }
        let num = lexeme.parse::<f64>().expect("Unable to parse number.");
        let span = self.make_span(start_pos, lexeme.len());
        self.tokens.push(Token::new(TokenType::Number(num), span));
    }

    fn scan_identifier(&mut self, start_pos: usize) {
        let mut lexeme = String::from("");
        while let Some((_, ch)) =
            self.cursor.next_if(|x| x.1.is_ascii_alphanumeric())
        {
            lexeme.push(ch);
        }
        let len = lexeme.len();
        let kind = match lexeme.as_str() {
            "and" => TokenType::And,
            "else" => TokenType::Else,
            "false" => TokenType::Boolean(false),
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "true" => TokenType::Boolean(true),
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier(lexeme),
        };
        let span = self.make_span(start_pos, len);
        self.tokens.push(Token::new(kind, span));
    }
}

#[cfg(test)]
mod test_lexer {
    use super::*;

    fn test_runner(src: &str, expected_tokens: &[TokenType]) -> bool {
        match Lexer::new(src).scan() {
            Ok(tokens) => {
                let output = tokens
                    .iter()
                    .map(|t| t.kind.clone())
                    .collect::<Vec<TokenType>>();
                output.as_slice() == expected_tokens
            }
            Err(error) => {
                eprintln!("{error:?}");
                false
            }
        }
    }

    #[test]
    fn test_single_char() {
        assert!(test_runner(".", &[TokenType::Dot]))
    }

    #[test]
    fn test_double_char() {
        assert!(test_runner("==", &[TokenType::EqualEqual]))
    }

    #[test]
    fn test_keyword() {
        assert!(test_runner(
            "while () {}",
            &[
                TokenType::While,
                TokenType::LeftParen,
                TokenType::RightParen,
                TokenType::LeftBrace,
                TokenType::RightBrace
            ]
        ));
    }

    #[test]
    fn test_string() {
        assert!(test_runner(
            "\"hello\"",
            &[TokenType::StringLiteral("hello".into())]
        ))
    }

    #[test]
    fn test_numbers() {
        assert!(test_runner("1", &[TokenType::Number(1.0)]));
        assert!(test_runner("123", &[TokenType::Number(123.0)]));
        assert!(test_runner("1.00", &[TokenType::Number(1.00)]));
    }

    #[test]
    fn test_bools() {
        assert!(test_runner(
            "false and true",
            &[
                TokenType::Boolean(false),
                TokenType::And,
                TokenType::Boolean(true),
            ]
        ));
    }

    #[test]
    fn test_identifier() {
        assert!(test_runner(
            "human",
            &[TokenType::Identifier("human".into())]
        ));
    }
}
