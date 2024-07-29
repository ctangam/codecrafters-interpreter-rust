use std::fmt::Display;


pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
    pub literal: Option<String>,
    // pub line: u32,
}

#[derive(Debug)]
enum TokenType {
        VAR,
        IDENTIFIER,
        STRING,
        NUMBER,
        PLUS,
        MINUS,
        MULTIPLY,
        DIVIDE,
        PRINT,
        SEMICOLON,
        LEFT_PAREN,
        RIGHT_PAREN,
        LEFT_BRACE,
        RIGHT_BRACE,
        COMMA,
        EQUAL,
        EOF,

}

impl Token {
    pub fn new(token_type: TokenType, lexeme: Option<String>, literal: Option<String>) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            // line,
        }
    }


}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lexeme = self.lexeme.clone().unwrap_or("".to_string());
        let literal = self.literal.clone().unwrap_or("null".to_string());
        write!(f, "{:?} {} {}", self.token_type, lexeme, literal)
    }
}

pub fn scan(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    for char in source.chars() {
        match char {
            '(' => tokens.push(Token::new(TokenType::LEFT_PAREN, Some(char.to_string()), None)),
            ')' => tokens.push(Token::new(TokenType::RIGHT_PAREN, Some(char.to_string()), None)),
            '{' => tokens.push(Token::new(TokenType::LEFT_BRACE, Some(char.to_string()), None)),
            '}' => tokens.push(Token::new(TokenType::RIGHT_BRACE, Some(char.to_string()), None)),
            ',' => tokens.push(Token::new(TokenType::COMMA, Some(char.to_string()), None)),
            '-' => tokens.push(Token::new(TokenType::MINUS, Some(char.to_string()), None)),
            '+' => tokens.push(Token::new(TokenType::PLUS, Some(char.to_string()), None)),
            ';' => tokens.push(Token::new(TokenType::SEMICOLON, Some(char.to_string()), None)),
            '*' => tokens.push(Token::new(TokenType::MULTIPLY, Some(char.to_string()), None)),
            '/' => tokens.push(Token::new(TokenType::DIVIDE, Some(char.to_string()), None)),
            _ => (),
        }
    }
    tokens.push(Token::new(TokenType::EOF, None, None));
    tokens
}