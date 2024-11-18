use anyhow::{Error, Result};

use crate::{
    expr::{Assign, Binary, Call, Expr, Grouping, Literal, Unary, Variable},
    stmt::{Block, Expression, For, Func, If, Print, Return, Stmt, Var, While},
    token::{Token, TokenValue},
};

pub struct Parser {
    tokens: Vec<Token>,
    errors: Vec<Error>,
    current: usize,
}

impl Parser {
    pub fn parse2(&mut self) -> Result<Vec<Stmt>, Vec<Error>> {
        let mut stmts = Vec::new();
        while !self.at_the_end() {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => self.errors.push(e),
            }
        }
        if !self.errors.is_empty() {
            return Err(std::mem::take(&mut self.errors));
        }
        Ok(stmts)
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        match self.peek().value {
            TokenValue::Print => self.print_stmt(),
            TokenValue::Var => self.var_stmt(),
            TokenValue::LeftBrace => self.block(),
            TokenValue::If => self.if_stmt(),
            TokenValue::While => self.while_stmt(),
            TokenValue::For => self.for_stmt(),
            TokenValue::Fun => self.func_stmt(),
            TokenValue::Return => self.return_stmt(),
            _ => self.expr_stmt(),
        }
    }

    fn return_stmt(&mut self) -> Result<Stmt, Error> {
        self.advance();
        let value = if self.peek().value != TokenValue::Semicolon {
            Some(self.expression()?)
        } else {
            None
        };
        if self.peek().value != TokenValue::Semicolon {
            return Err(Error::msg(format!(
                "[line {}] Error at '{}': Expect ';' after value.",
                self.peek().line,
                self.peek().lexeme
            )));
        }
        self.advance();
        let stmt = Stmt::Return(Return { value });
        Ok(stmt)
    }

    fn func_stmt(&mut self) -> Result<Stmt, Error> {
        self.advance();
        let name = self.advance().clone();
        assert_eq!(self.peek().value, TokenValue::LeftParen);
        self.advance();
        let mut params = Vec::new();
        while self.peek().value != TokenValue::RightParen && !self.at_the_end() {
            params.push(self.advance().clone());
            match self.peek().value {
                TokenValue::Comma => {
                    self.advance();
                }
                TokenValue::RightParen => break,
                _ => {
                    return Err(Error::msg(format!(
                        "[line {}] Error at '{}': Expect ')' after paramters.",
                        self.peek().line,
                        self.peek().lexeme
                    )))
                }
            }
        }
        self.advance();

        if self.peek().value != TokenValue::LeftBrace {
            return Err(Error::msg(format!(
                "[line {}] Error at '{}': Expect '{{' before function body.",
                self.peek().line,
                self.peek().lexeme
            )));
        }
        self.advance();
        let mut body = Vec::new();
        while self.peek().value != TokenValue::RightBrace && !self.at_the_end() {
            body.push(self.declaration()?);
        }
        if self.peek().value != TokenValue::RightBrace {
            return Err(Error::msg(format!(
                "[line {}] Error at end: Expect '}}' .",
                self.peek().line
            )));
        }
        self.advance();

        let stmt = Stmt::Func(Func { name, params, body });
        Ok(stmt)
    }

    fn for_stmt(&mut self) -> Result<Stmt, Error> {
        self.advance();
        self.advance();
        let init = if self.matches(&[TokenValue::Semicolon]) {
            None
        } else {
            Some(Box::new(self.declaration()?))
        };

        let condition = Box::new(self.expression()?);
        self.advance();

        let update = if self.peek().value != TokenValue::RightParen {
            Some(Box::new(self.expression()?))
        } else {
            None
        };
        self.advance();

        let body = Box::new(self.declaration()?);
        let stmt = Stmt::For(For {
            init,
            condition,
            update,
            body,
        });
        Ok(stmt)
    }

    fn while_stmt(&mut self) -> Result<Stmt, Error> {
        self.advance();
        let condition = Box::new(self.expression()?);
        let body = Box::new(self.declaration()?);
        let stmt = Stmt::While(While { condition, body });
        Ok(stmt)
    }

    fn if_stmt(&mut self) -> Result<Stmt, Error> {
        self.advance();
        let condition = Box::new(self.expression()?);
        let then_branch = Box::new(self.declaration()?);
        let else_branch = if self.matches(&[TokenValue::Else]) {
            Some(Box::new(self.declaration()?))
        } else {
            None
        };
        let stmt = Stmt::If(If {
            condition,
            then_branch,
            else_branch,
        });
        Ok(stmt)
    }

    fn print_stmt(&mut self) -> Result<Stmt, Error> {
        self.advance();
        let expr = self.expression()?;
        if self.peek().value != TokenValue::Semicolon {
            return Err(Error::msg(format!(
                "[line {}] Error at '{}': Expect ';' after value.",
                self.peek().line,
                self.peek().lexeme
            )));
        }
        self.advance();
        let stmt = Stmt::Print(Print {
            expr: Box::new(expr),
        });
        Ok(stmt)
    }

    fn expr_stmt(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.advance();
        let stmt = Stmt::Expression(Expression {
            expr: Box::new(expr),
        });
        Ok(stmt)
    }

    fn var_stmt(&mut self) -> Result<Stmt, Error> {
        self.advance();
        let name = self.advance().clone();
        let initializer = if self.peek().value == TokenValue::Equal {
            self.advance();
            let expr = self.expression()?;
            Some(Box::new(expr))
        } else {
            None
        };
        self.advance();
        let stmt = Stmt::Var(Var { name, initializer });
        Ok(stmt)
    }

    fn block(&mut self) -> Result<Stmt, Error> {
        self.advance();
        let mut stmts = Vec::new();
        while self.peek().value != TokenValue::RightBrace && !self.at_the_end() {
            stmts.push(self.declaration()?);
        }
        if self.peek().value != TokenValue::RightBrace {
            return Err(Error::msg(format!(
                "[line {}] Error at end: Expect '}}' .",
                self.peek().line
            )));
        }
        self.advance();
        let stmt = Stmt::Block(Block { statements: stmts });
        Ok(stmt)
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            errors: vec![],
            current: 0,
        }
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
        self.assign()
    }

    fn assign(&mut self) -> Result<Expr, Error> {
        if self
            .peek_next()
            .is_some_and(|token| token.value == TokenValue::Equal)
        {
            let name = self.advance().clone();
            self.advance();
            let value = self.assign()?;
            return Ok(Expr::Assign(Assign {
                name,
                value: Box::new(value),
            }));
        } else {
            return self.logical_or();
        }
    }

    fn logical_or(&mut self) -> Result<Expr, Error> {
        let mut left = self.logical_and()?;
        while self.matches(&[TokenValue::Or]) {
            let operator = self.previous().clone();
            let right = self.logical_and()?;
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn logical_and(&mut self) -> Result<Expr, Error> {
        let mut left = self.equality()?;
        while self.matches(&[TokenValue::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut left = self.comparison()?;
        while self.matches(&[TokenValue::BangEqual, TokenValue::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut left = self.term()?;
        while self.matches(&[
            TokenValue::Greater,
            TokenValue::GreaterEqual,
            TokenValue::Less,
            TokenValue::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut left = self.factory()?;
        while self.matches(&[TokenValue::Plus, TokenValue::Minus]) {
            let operator = self.previous().clone();
            let right = self.factory()?;
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn factory(&mut self) -> Result<Expr, Error> {
        let mut left = self.unary()?;
        while self.matches(&[TokenValue::Star, TokenValue::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            left = Expr::Binary(Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.matches(&[TokenValue::Minus, TokenValue::Bang]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            }))
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
                if self.matches(&[TokenValue::RightParen]) {
                    Ok(Expr::Grouping(Grouping {
                        expr: Box::new(expr),
                    }))
                } else {
                    Err(Error::msg(format!(
                        "[line {}] Error: Expect ')' after expression.",
                        self.peek().line
                    )))
                }
            }

            TokenValue::Identifier => {
                let callee = self.previous().clone();
                if self.peek().value == TokenValue::LeftParen {
                    self.advance();
                    let mut arguments = Vec::new();
                    while self.peek().value != TokenValue::RightParen && !self.at_the_end() {
                        arguments.push(self.expression()?);
                        match self.peek().value {
                            TokenValue::Comma => {
                                self.advance();
                            }
                            TokenValue::RightParen => break,
                            _ => {
                                return Err(Error::msg(format!(
                                    "[line {}] Error at '{}': Expect ')' after parameters.",
                                    self.peek().line,
                                    self.peek().lexeme
                                )));
                            }
                        }
                    }
                    self.advance();
                    Ok(Expr::Call(Call { callee, arguments }))
                } else {
                    Ok(Expr::Variable(Variable { name: callee }))
                }
            }

            _ => Err(Error::msg(format!(
                "[line {}] Error at '{}': Expect expression.",
                self.previous().line,
                self.previous().lexeme
            ))),
        }
    }

    fn matches(&mut self, expected: &[TokenValue]) -> bool {
        if self.at_the_end() {
            return false;
        }

        if !expected.contains(&self.peek().value) {
            return false;
        }

        self.advance();
        true
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
