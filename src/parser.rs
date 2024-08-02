use anyhow::{Result, Error};

use crate::{expr::{Expr, Grouping, Literal, Unary}, token::{Token, TokenValue}};

pub struct Parser {
    tokens: Vec<Token>,
    errors: Vec<Error>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, errors: vec![], current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, Vec<Error>> {
        let mut exprs = Vec::new();
        while !self.at_the_end() {
            match self.expression() {
                Ok(expr) => exprs.push(expr),
                Err(e) => self.errors.push(e),
            }
        }
        if !self.errors.is_empty() {
            return Err(std::mem::take(&mut self.errors));
        }
        Ok(exprs)
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.unary()
    }

    // fn factory(&mut self) -> Result<Expr, Error> {
    // }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.matches(TokenValue::Minus) || self.matches(TokenValue::Bang) {
            let operator = self.advance().clone();
            let right = self.unary()?;
            Ok(Expr::Unary(Unary { operator, right: Box::new(right) }))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        self.advance();
        match &self.previous().value {
            TokenValue::Number(n) => Ok(Expr::Literal(Literal::Number(*n))),
            TokenValue::String(s) => Ok(Expr::Literal(Literal::String(s.clone()))),
            TokenValue::True => Ok(Expr::Literal(Literal::True)),
            TokenValue::False => Ok(Expr::Literal(Literal::False)),
            TokenValue::Nil => Ok(Expr::Literal(Literal::Nil)),

            TokenValue::LeftParen => {
                let expr = self.expression()?;
                self.advance();
                if self.matches(TokenValue::RightParen) {
                    Ok(Expr::Grouping(Grouping { expr: Box::new(expr) }))
                } else {
                    Err(Error::msg("Error: Unmatched parentheses."))
                }
            }

            _ => todo!(),
        }
    }

    fn matches(&mut self, expected: TokenValue) -> bool {
        if self.at_the_end() {
            return false;
        }
        self.peek().value == expected
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn at_the_end(&self) -> bool {
        self.peek().value == TokenValue::Eof
    }

    fn advance(&mut self) -> &Token {
        if !self.at_the_end() {
            self.current += 1;
        }
        self.previous()
    }
}