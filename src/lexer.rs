use std::collections::HashMap;

use crate::token::{Token, TokenValue, KEYWORDS};

pub fn scan(source: String) -> (Vec<Token>, i32) {
    let keywords = HashMap::from(KEYWORDS);
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut code = 0;
    let chars = source.chars().collect::<Vec<_>>();
    let mut i = 0;
    while let Some(char) = chars.get(i) {
        match char {
            '(' => tokens.push(Token::new(TokenValue::LeftParen, char.to_string(), line)),
            ')' => tokens.push(Token::new(TokenValue::RightParen, char.to_string(), line)),
            '{' => tokens.push(Token::new(TokenValue::LeftBrace, char.to_string(), line)),
            '}' => tokens.push(Token::new(TokenValue::RightBrace, char.to_string(), line)),
            ',' => tokens.push(Token::new(TokenValue::Comma, char.to_string(), line)),
            '.' => tokens.push(Token::new(TokenValue::Dot, char.to_string(), line)),
            '-' => tokens.push(Token::new(TokenValue::Minus, char.to_string(), line)),
            '+' => tokens.push(Token::new(TokenValue::Plus, char.to_string(), line)),
            ';' => tokens.push(Token::new(TokenValue::Semicolon, char.to_string(), line)),
            '*' => tokens.push(Token::new(TokenValue::Star, char.to_string(), line)),
            '/' => {
                if let Some('/') = chars.get(i + 1) {
                    while chars.get(i + 1).is_some_and(|c| *c != '\n') {
                        i += 1;
                    }
                } else {
                    tokens.push(Token::new(TokenValue::Slash, char.to_string(), line))
                }
            }
            '=' => {
                if let Some('=') = chars.get(i + 1) {
                    tokens.push(Token::new(TokenValue::EqualEqual, "==".to_string(), line));
                    i += 1;
                } else {
                    tokens.push(Token::new(TokenValue::Equal, char.to_string(), line))
                }
            }
            '!' => {
                if let Some('=') = chars.get(i + 1) {
                    tokens.push(Token::new(TokenValue::BangEqual, "!=".to_string(), line));
                    i += 1;
                } else {
                    tokens.push(Token::new(TokenValue::Bang, char.to_string(), line))
                }
            }
            '<' => {
                if let Some('=') = chars.get(i + 1) {
                    tokens.push(Token::new(TokenValue::LessEqual, "<=".to_string(), line));
                    i += 1;
                } else {
                    tokens.push(Token::new(TokenValue::Less, char.to_string(), line))
                }
            }
            '>' => {
                if let Some('=') = chars.get(i + 1) {
                    tokens.push(Token::new(TokenValue::GreaterEqual, ">=".to_string(), line));
                    i += 1;
                } else {
                    tokens.push(Token::new(TokenValue::Greater, char.to_string(), line))
                }
            }

            '"' => {
                let mut lexeme = String::new();
                lexeme.push(*char);
                i += 1;
                loop {
                    let char = chars.get(i);
                    match char {
                        Some('"') => {
                            lexeme.push('"');
                            let literal: String =
                                lexeme.clone().drain(1..lexeme.len() - 1).collect();
                            tokens.push(Token::new(TokenValue::String(literal), lexeme, line));
                            break;
                        }
                        Some('\n') => line += 1,
                        Some(char) => {
                            lexeme.push(*char);
                            i += 1;
                        }
                        None => {
                            eprintln!("[line {line}] Error: Unterminated string.");
                            code = 65;
                            break;
                        }
                    }
                }
            }

            '0'..='9' => {
                let mut lexeme = String::new();
                let mut has_digit = false;
                while let Some(char) = chars.get(i) {
                    match char {
                        '0'..='9' => {
                            lexeme.push(*char);
                            i += 1;
                        }
                        '.' if !has_digit => {
                            has_digit = true;
                            lexeme.push(*char);
                            i += 1;
                        }
                        _ => {
                            i -= 1;
                            break;
                        }
                    }
                }
                if lexeme.ends_with(".") {
                    i -= 2;
                    lexeme.pop();
                }
                if lexeme.parse::<f64>().is_ok() {
                    tokens.push(Token::new(
                        TokenValue::Number(lexeme.parse().unwrap()),
                        lexeme.to_string(),
                        line,
                    ));
                } else {
                    eprintln!("[line {line}] Error: Unexpected character: {lexeme}");
                    code = 65;
                }
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut lexeme = String::new();
                while let Some(char) = chars.get(i) {
                    match char {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            lexeme.push(*char);
                            i += 1;
                        }
                        _ => {
                            i -= 1;
                            break;
                        }
                    }
                }
                if keywords.contains_key(lexeme.as_str()) {
                    tokens.push(Token::new(
                        keywords.get(lexeme.as_str()).unwrap().clone(),
                        lexeme,
                        line,
                    ));
                } else {
                    tokens.push(Token::new(TokenValue::Identifier, lexeme, line));
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
    tokens.push(Token::new(TokenValue::Eof, "".to_string(), line));
    (tokens, code)
}
