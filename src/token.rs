use std::{collections::HashMap, fmt::{Display, Pointer}};

pub struct Token {
    pub value: TokenValue,
    pub lexeme: String,
    // pub line: u32,
}

pub type Number = f64;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Colon,
    Comma,
    Dot,
    Minus,
    Plus,
    Question,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String(String),
    Number(Number),

    // Keywords.
    And,
    Break,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

const KEYWORDS: [(&str, TokenValue); 17] = [
    ("and", TokenValue::And),
    ("break", TokenValue::Break),
    ("class", TokenValue::Class),
    ("else", TokenValue::Else),
    ("false", TokenValue::False),
    ("for", TokenValue::For),
    ("fun", TokenValue::Fun),
    ("if", TokenValue::If),
    ("nil", TokenValue::Nil),
    ("or", TokenValue::Or),
    ("print", TokenValue::Print),
    ("return", TokenValue::Return),
    ("super", TokenValue::Super),
    ("this", TokenValue::This),
    ("true", TokenValue::True),
    ("var", TokenValue::Var),
    ("while", TokenValue::While),
];

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::LeftParen => write!(f, "LEFT_PAREN"),
            TokenValue::RightParen => write!(f, "RIGHT_PAREN"),
            TokenValue::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenValue::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenValue::Colon => write!(f, "COLON"),
            TokenValue::Comma => write!(f, "COMMA"),
            TokenValue::Dot => write!(f, "DOT"),
            TokenValue::Minus => write!(f, "MINUS"),
            TokenValue::Plus => write!(f, "PLUS"),
            TokenValue::Question => write!(f, "QUESTION"),
            TokenValue::Semicolon => write!(f, "SEMICOLON"),
            TokenValue::Slash => write!(f, "SLASH"),
            TokenValue::Star => write!(f, "STAR"),

            TokenValue::Bang => write!(f, "BANG"),
            TokenValue::BangEqual => write!(f, "BANG_EQUAL"),
            TokenValue::Equal => write!(f, "EQUAL"),
            TokenValue::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenValue::Greater => write!(f, "GREATER"),
            TokenValue::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenValue::Less => write!(f, "LESS"),
            TokenValue::LessEqual => write!(f, "LESS_EQUAL"),

            TokenValue::Identifier => write!(f, "IDENTIFIER"),
            TokenValue::String(_) => write!(f, "STRING"),
            TokenValue::Number(_) => write!(f, "NUMBER"),

            TokenValue::And => write!(f, "AND"),
            TokenValue::Break => write!(f, "BREAK"),
            TokenValue::Class => write!(f, "CLASS"),
            TokenValue::Else => write!(f, "ELSE"),
            TokenValue::False => write!(f, "FALSE"),
            TokenValue::Fun => write!(f, "FUN"),
            TokenValue::For => write!(f, "FOR"),
            TokenValue::If => write!(f, "IF"),
            TokenValue::Nil => write!(f, "NIL"),
            TokenValue::Or => write!(f, "OR"),
            TokenValue::Print => write!(f, "PRINT"),
            TokenValue::Return => write!(f, "RETURN"),
            TokenValue::Super => write!(f, "SUPER"),
            TokenValue::This => write!(f, "THIS"),
            TokenValue::True => write!(f, "TRUE"),
            TokenValue::Var => write!(f, "VAR"),
            TokenValue::While => write!(f, "WHILE"),

            TokenValue::Eof => write!(f, "EOF"),
            
        }
    }
}

impl Token {
    pub fn new(value: TokenValue, lexeme: String) -> Token {
        Token {
            value,
            lexeme,
            // line,
        }
    }
}



impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            TokenValue::String(s) => write!(f, "{} {} {s}", self.value, self.lexeme),
            TokenValue::Number(n) => write!(f, "{} {} {n}", self.value, self.lexeme),
            _ => write!(f, "{} {} null", self.value, self.lexeme),
        }
    }
}

pub fn scan(source: String) -> (Vec<Token>, i32) {
    let keywords = HashMap::from(KEYWORDS);
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut code = 0;
    let chars = source.chars().collect::<Vec<_>>();
    let mut i = 0;
    loop {
        let char = chars.get(i);
        if char.is_none() {
            break;
        }
        let char = char.unwrap();
        match char {
            '(' => tokens.push(Token::new(TokenValue::LeftParen, char.to_string())),
            ')' => tokens.push(Token::new(TokenValue::RightParen, char.to_string())),
            '{' => tokens.push(Token::new(TokenValue::LeftBrace, char.to_string())),
            '}' => tokens.push(Token::new(TokenValue::RightBrace, char.to_string())),
            ',' => tokens.push(Token::new(TokenValue::Comma, char.to_string())),
            '.' => tokens.push(Token::new(TokenValue::Dot, char.to_string())),
            '-' => tokens.push(Token::new(TokenValue::Minus, char.to_string())),
            '+' => tokens.push(Token::new(TokenValue::Plus, char.to_string())),
            ';' => tokens.push(Token::new(TokenValue::Semicolon, char.to_string())),
            '*' => tokens.push(Token::new(TokenValue::Star, char.to_string())),
            '/' => {
                if let Some('/') = chars.get(i + 1) {
                    while chars.get(i + 1).is_some_and(|c| *c != '\n') {
                        i += 1;
                    }
                } else {
                    tokens.push(Token::new(TokenValue::Slash, char.to_string()))
                }
            }
            '=' => {
                if let Some('=') = chars.get(i + 1) {
                    tokens.push(Token::new(TokenValue::EqualEqual, "==".to_string()));
                    i += 1;
                } else {
                    tokens.push(Token::new(TokenValue::Equal, char.to_string()))
                }
            }
            '!' => {
                if let Some('=') = chars.get(i + 1) {
                    tokens.push(Token::new(TokenValue::BangEqual, "!=".to_string()));
                    i += 1;
                } else {
                    tokens.push(Token::new(TokenValue::Bang, char.to_string()))
                }
            }
            '<' => {
                if let Some('=') = chars.get(i + 1) {
                    tokens.push(Token::new(TokenValue::LessEqual, "<=".to_string()));
                    i += 1;
                } else {
                    tokens.push(Token::new(TokenValue::Less, char.to_string()))
                }
            }
            '>' => {
                if let Some('=') = chars.get(i + 1) {
                    tokens.push(Token::new(TokenValue::GreaterEqual, ">=".to_string()));
                    i += 1;
                } else {
                    tokens.push(Token::new(TokenValue::Greater, char.to_string()))
                }
            }

            '"' => {
                let mut lexeme = String::new();
                lexeme.push(*char);
                i += 1;
                loop {
                    let char = chars.get(i);
                    if char.is_none() {
                        eprintln!("[line {line}] Error: Unterminated string.");
                        code = 65;
                        break;
                    }
                    let char = char.unwrap();
                    if *char == '"' {
                        lexeme.push(*char);
                        let literal: String = lexeme.clone().drain(1..lexeme.len() - 1).collect();
                        tokens.push(Token::new(TokenValue::String(literal), lexeme));
                        break;
                    }
                    if *char == '\n' {
                        line += 1;
                    }
                    lexeme.push(*char);
                    i += 1;
                }
            }

            '0'..='9' => {
                let mut lexeme = String::new();
                lexeme.push(*char);
                i += 1;
                loop {
                    let char = chars.get(i);
                    if char.is_none() {
                        break;
                    }
                    let char = char.unwrap();
                    if !char.is_ascii_digit() && *char != '.' {
                        break;
                    }
                    lexeme.push(*char);
                    i += 1;
                }
                if lexeme.parse::<f64>().is_ok() {
                    tokens.push(Token::new(TokenValue::Number(lexeme.parse().unwrap()), lexeme));
                } else {
                    eprintln!("[line {line}] Error: Unexpected character: {lexeme}");
                    code = 65;
                }
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut lexeme = String::new();
                lexeme.push(*char);
                i += 1;
                loop {
                    let char = chars.get(i);
                    if char.is_none() {
                        break;
                    }
                    let char = char.unwrap();
                    if !char.is_ascii_alphanumeric() && *char != '_' {
                        break;
                    }
                    lexeme.push(*char);
                    i += 1;
                }
                if keywords.contains_key(lexeme.as_str()) {
                    tokens.push(Token::new(
                        keywords.get(lexeme.as_str()).unwrap().clone(),
                        lexeme,
                    ));
                } else {
                    tokens.push(Token::new(TokenValue::Identifier, lexeme));
                }
            }

            ' ' | '\r' | '\t' => {}
            '\n' => line += 1,
            c => {
                eprintln!("[line {line}] Error: Unexpected character: {c}");
                code = 65;
            }
        }
        i += 1;
    }
    tokens.push(Token::new(TokenValue::Eof, "".to_string()));
    (tokens, code)
}
