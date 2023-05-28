use crate::error::{BessyError, Index};
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
    pub index: Index,
    pub kind: TokenType,
    pub line: u16,
}

impl Token {
    pub fn new(index: Index, kind: TokenType, line: u16) -> Self {
        Self { index, kind, line }
    }

    pub fn is_identifier(&self) -> bool {
        match self.kind {
            TokenType::Identifier(_) => true,
            _ => false,
        }
    }
}

pub struct Lexer<'src> {
    cursor: Peekable<CharIndices<'src>>,
    tokens: Vec<Token>,
    line: u16,
    consumed: u16,
}

impl<'src> Lexer<'src> {
    pub fn new(text: &'src str) -> Self {
        Self {
            cursor: text.char_indices().peekable(),
            tokens: vec![],
            line: 1,
            consumed: 0,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, BessyError> {
        while let Some(&(start_pos, c)) = self.cursor.peek() {
            match c {
                '(' | ')' | '.' | '-' | '+' | '*' | ';' | '{' | '}' | ','
                | '/' => self.scan_single_token(),
                '~' => self.scan_comment(),
                '!' | '=' | '>' | '<' => self.scan_double_token(),
                ' ' | '\r' | '\t' => {
                    self.cursor.next();
                }
                '\n' => {
                    let (index, _) = self.cursor.next().unwrap();
                    self.line += 1;
                    self.consumed = index as u16;
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
                        self.tokens.push(Token::new(
                            self.make_index(start_pos),
                            TokenType::Unknown,
                            self.line,
                        ));
                    }
                }
            }
        }
        Ok(self.tokens.clone())
    }

    fn make_index(&self, start: usize) -> Index {
        if self.line > 1 {
            Index {
                row: self.line,
                column: (start as u16 - self.consumed) - 2,
            }
        } else {
            Index {
                row: self.line,
                column: (start as u16 - self.consumed),
            }
        }
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
            _ => unreachable!(),
        };
        self.tokens.push(Token::new(
            self.make_index(start_pos),
            kind,
            self.line,
        ));
    }

    fn scan_comment(&mut self) {
        for (index, ch) in self.cursor.by_ref() {
            if ch == '\n' {
                self.line += 1;
                self.consumed = index as u16;
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
        if let Some((end_pos, _)) = self.cursor.next_if(|x| x.1 == '=') {
            Token::new(self.make_index(start_pos), this, self.line)
        } else {
            Token::new(self.make_index(start_pos), that, self.line)
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
        if self.cursor.peek().map_or(false, |x| x.1 == '"') {
            let _ = self.cursor.next();
            Ok(Token::new(
                self.make_index(start),
                TokenType::StrLit(lexeme),
                self.line,
            ))
        } else {
            let column = start as u16 - self.consumed;
            Err(BessyError::UnterminatedString(self.make_index(start)))
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
        self.tokens.push(Token::new(
            self.make_index(start_pos),
            TokenType::Number(num),
            self.line,
        ));
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
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier(lexeme),
        };
        self.tokens
            .push(Token::new(self.make_index(start_pos), kind, self.line));
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
            &[TokenType::StrLit("hello".into())]
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
            &[TokenType::False, TokenType::And, TokenType::True,]
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
