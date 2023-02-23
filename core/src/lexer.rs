use std::iter::Peekable;
use std::str::CharIndices;

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

pub struct Lexer<'src> {
    cursor: Peekable<CharIndices<'src>>,
    tokens: Vec<Token>,
}

impl<'src> Lexer<'src> {
    pub fn new(text: &'src str) -> Self {
        Self {
            cursor: text.char_indices().peekable(),
            tokens: vec![],
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, LexError> {
        while let Some(&(start_pos, c)) = self.cursor.peek() {
            match c {
                '(' | ')' | '.' | '-' | '+' | '*' | ';' | '{' | '}' | ','
                    | '/' => self.scan_single_token(),
                '~' => self.scan_comment(),
                '!' | '=' | '>' | '<' => self.scan_double_token(),
                ' ' | '\r' | '\t' | '\n' => {
                    self.cursor.next();
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
                            (start_pos, start_pos + c.len_utf8()),
                            TokenType::Unknown,
                        ));
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
            _ => unreachable!(),
        };
        self.tokens.push(Token::new((start_pos, start_pos + c.len_utf8()),
                                    kind));
    }

    fn scan_comment(&mut self) {
        for (_, ch) in self.cursor.by_ref() {
            if ch == '\n' {
                break;
            }
        }
    }

    fn check_next(&mut self, start_pos: usize, len: usize, this: TokenType, that: TokenType) -> Token {
        if let Some((end_pos, _)) = self.cursor.next_if(|x| x.1 == '=') {
            Token::new((start_pos, end_pos + 1), this)
        } else {
            Token::new(
                (start_pos, start_pos + len),
                that,
            )
        }
    }

    fn scan_double_token(&mut self) {
        let (start_pos, c) = self.cursor.next().unwrap();
        let token = match c {
            '!' => self.check_next(start_pos, c.len_utf8(), TokenType::BangEqual, TokenType::Bang),
            '=' => self.check_next(start_pos, c.len_utf8(), TokenType::EqualEqual, TokenType::Equal),
            '>' => self.check_next(start_pos, c.len_utf8(), TokenType::GreaterEqual, TokenType::Greater),
            '<' => self.check_next(start_pos, c.len_utf8(), TokenType::LessEqual, TokenType::Less),
            _ => unreachable!(),
        };
        self.tokens.push(token);
    }

    fn scan_string(&mut self) -> Result<Token, LexError> {
        let mut lexeme = String::from("");
        let (start_pos, _) = self.cursor.next().unwrap(); // skip opening quotes
        let start = start_pos + 1;
        while let Some((_, ch)) = self.cursor.next_if(|x| x.1 != '"') {
            lexeme.push(ch);
        }
        if self.cursor.peek().map_or(false, |x| x.1 == '"') {
            let _ = self.cursor.next();
            Ok(Token::new(
                (start, start + lexeme.len()),
                TokenType::StrLit(lexeme),
            ))
        } else {
            Err(LexError::UnterminatedString)
        }
    }

    fn scan_number(&mut self, start_pos: usize) {
        let mut lexeme = String::from("");
        while let Some((_, num)) = self.cursor.next_if(|x| x.1.is_ascii_digit()) {
            lexeme.push(num);
        }
        if self.cursor.peek().map_or(false, |x| x.1 == '.') {
            lexeme.push('.');
            let _ = self.cursor.next();
            while let Some((_, num)) = self.cursor.next_if(|x| x.1.is_ascii_digit())
            {
                lexeme.push(num);
            }
        }
        let num = lexeme.parse::<f64>().expect("Unable to parse number.");
        self.tokens.push(Token::new(
            (start_pos, start_pos + lexeme.len()),
            TokenType::Number(num),
        ));
    }

    
    fn scan_identifier(&mut self, start_pos: usize) {
        let mut lexeme = String::from("");
        while let Some((_, ch)) = self.cursor.next_if(|x| x.1.is_ascii_alphanumeric()) {
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
        self.tokens.push(Token::new((start_pos, start_pos + len), kind));
    }
}
        
