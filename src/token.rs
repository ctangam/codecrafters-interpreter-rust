use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
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

pub const KEYWORDS: [(&str, TokenValue); 17] = [
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
            TokenValue::String(s) => write!(f, "{} {} {}", self.value, self.lexeme, s),
            TokenValue::Number(n) => {
                if n % 1.0 == 0.0 {
                    write!(f, "{} {} {:.1}", self.value, self.lexeme, n)
                } else {
                    write!(f, "{} {} {}", self.value, self.lexeme, n)
                }
            }
            _ => write!(f, "{} {} null", self.value, self.lexeme),
        }
    }
}
