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
        STAR,
        SLASH,
        SEMICOLON,
        LEFT_PAREN,
        RIGHT_PAREN,
        LEFT_BRACE,
        RIGHT_BRACE,
        COMMA,
        DOT,
        EQUAL,
        EQUAL_EQUAL,
        BANG,
        BANG_EQUAL,
        GREATER,
        GREATER_EQUAL,
        LESS,
        LESS_EQUAL,
        AND,
        OR,
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

pub fn scan(source: String) -> (Vec<Token>, i32) {
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
            '(' => tokens.push(Token::new(TokenType::LEFT_PAREN, Some(char.to_string()), None)),
            ')' => tokens.push(Token::new(TokenType::RIGHT_PAREN, Some(char.to_string()), None)),
            '{' => tokens.push(Token::new(TokenType::LEFT_BRACE, Some(char.to_string()), None)),
            '}' => tokens.push(Token::new(TokenType::RIGHT_BRACE, Some(char.to_string()), None)),
            ',' => tokens.push(Token::new(TokenType::COMMA, Some(char.to_string()), None)),
            '.' => tokens.push(Token::new(TokenType::DOT, Some(char.to_string()), None)),
            '-' => tokens.push(Token::new(TokenType::MINUS, Some(char.to_string()), None)),
            '+' => tokens.push(Token::new(TokenType::PLUS, Some(char.to_string()), None)),
            ';' => tokens.push(Token::new(TokenType::SEMICOLON, Some(char.to_string()), None)),
            '*' => tokens.push(Token::new(TokenType::STAR, Some(char.to_string()), None)),
            '/' => if let Some('/') = chars.get(i + 1) {
                while chars.get(i + 1).is_some_and(|c| *c != '\n') {
                    i += 1;
                }
            } else {
                tokens.push(Token::new(TokenType::SLASH, Some(char.to_string()), None))
            },
            '=' => if let Some('=') = chars.get(i + 1)  {
                tokens.push(Token::new(TokenType::EQUAL_EQUAL, Some("==".to_string()), None));
                i += 1;
            } else {
                tokens.push(Token::new(TokenType::EQUAL, Some(char.to_string()), None))
            },
            '!' => if let Some('=') = chars.get(i + 1) {
                tokens.push(Token::new(TokenType::BANG_EQUAL, Some("!=".to_string()), None));
                i += 1;
            } else {
                tokens.push(Token::new(TokenType::BANG, Some(char.to_string()), None))
            },
            '<' =>  if let Some('=') = chars.get(i + 1)  {
                tokens.push(Token::new(TokenType::LESS_EQUAL, Some("<=".to_string()), None));
                i += 1;
            } else {
                tokens.push(Token::new(TokenType::LESS, Some(char.to_string()), None))
            },
            '>' =>  if let Some('=') = chars.get(i + 1)  {
                tokens.push(Token::new(TokenType::GREATER_EQUAL, Some(">=".to_string()), None));
                i += 1;
            } else {
                tokens.push(Token::new(TokenType::GREATER, Some(char.to_string()), None))
            },

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
                        let literal = lexeme.clone().drain(1..lexeme.len() - 1).collect(); 
                        tokens.push(Token::new(TokenType::STRING, Some(lexeme), Some(literal)));
                        break;
                    }
                    if *char == '\n' {
                        line += 1;
                    }
                    lexeme.push(*char);
                    i += 1;
                }
            },
            
            ' ' | '\r' | '\t' => {},
            '\n' => line += 1,
            c => {eprintln!("[line {line}] Error: Unexpected character: {c}"); code = 65;},
        }
        i += 1;
    }
    tokens.push(Token::new(TokenType::EOF, None, None));
    (tokens, code)
}