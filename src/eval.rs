use std::fmt::Display;

use anyhow::{Result, Error};

use crate::{expr::{Assign, Binary, Expr, ExprVisitor, Grouping, Literal, Unary}, token::{Number, TokenValue}, Walkable};

pub enum Value {
    Nil,
    Boolean(bool),
    Number(Number),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&self, exprs: Vec<Expr>) {
        for expr in exprs {
            let value = expr.walk(self);
            match value {
                Ok(v) => println!("{}", v),
                Err(e) => println!("{}", e)
            }
        }
    }
}

impl ExprVisitor<Result<Value, Error>> for Interpreter {
    fn visit_literal(&self, expr: &Literal) -> Result<Value, Error> {
        match expr {
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Number(n) => Ok(Value::Number(n.clone())),
            Literal::True => Ok(Value::Boolean(true)),
            Literal::False => Ok(Value::Boolean(false)),
            Literal::Nil => Ok(Value::Nil),
        }
    }

    fn visit_grouping(&self, expr: &Grouping) -> Result<Value, Error> {
        expr.expr.walk(self)
    }

    fn visit_unary(&self, expr: &Unary) -> Result<Value, Error> {
        let right = expr.right.walk(self)?;
        match expr.operator.value {
            TokenValue::Minus => {
                if let Value::Number(n) = right {
                    Ok(Value::Number(-n))
                } else {
                    Err(Error::msg(format!("Operand must be a number.\n[line {}]", expr.operator.line)))
                }
            }
            TokenValue::Bang => {
                match right {
                    Value::Boolean(b) => Ok(Value::Boolean(!b)),
                    Value::Nil => Ok(Value::Boolean(true)),
                    _ => Ok(Value::Boolean(false)),
                }
            }
            _ => unreachable!(),
        }
    }

    fn visit_binary(&self, expr: &Binary) -> Result<Value, Error> {
        let left = expr.left.walk(self)?;
        let right = expr.right.walk(self)?;
        match expr.operator.value {
            TokenValue::Plus => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
                    _ => Err(Error::msg(format!("[line {}] Error at '{}': Expect number or string.", expr.operator.line, expr.operator.lexeme)))
                }
            }
            TokenValue::Minus => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Number(l - r))
                } else {
                    Err(Error::msg(format!("[line {}] Error at '{}': Expect number.", expr.operator.line, expr.operator.lexeme)))
                }
            }
            TokenValue::Star => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Number(l * r))
                } else {
                    Err(Error::msg(format!("[line {}] Error at '{}': Expect number.", expr.operator.line, expr.operator.lexeme)))
                }
            }
            TokenValue::Slash => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Number(l / r))
                } else {
                    Err(Error::msg(format!("[line {}] Error at '{}': Expect number.", expr.operator.line, expr.operator.lexeme)))
                }
            }
            TokenValue::Greater => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Boolean(l > r))
                } else {
                    Err(Error::msg(format!("[line {}] Error at '{}': Expect number.", expr.operator.line, expr.operator.lexeme)))
                }
            }
            TokenValue::GreaterEqual => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Boolean(l >= r))
                } else {
                    Err(Error::msg(format!("[line {}] Error at '{}': Expect number.", expr.operator.line, expr.operator.lexeme)))
                }
            }
            TokenValue::Less => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Boolean(l < r))
                } else {
                    Err(Error::msg(format!("[line {}] Error at '{}': Expect number.", expr.operator.line, expr.operator.lexeme)))
                }
            }
            TokenValue::LessEqual => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Boolean(l <= r))
                } else {
                    Err(Error::msg(format!("[line {}] Error at '{}': Expect number.", expr.operator.line, expr.operator.lexeme)))
                }
            }
            TokenValue::EqualEqual => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l == r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l == r)),
                    (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
                    (Value::Nil, Value::Nil) => Ok(Value::Boolean(true)),

                    _ => Ok(Value::Boolean(false)),
                }
            }
            TokenValue::BangEqual => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l != r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l != r)),
                    (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l != r)),
                    (Value::Nil, Value::Nil) => Ok(Value::Boolean(false)),

                    _ => Ok(Value::Boolean(true)),
                }
            }
            _ => unreachable!(),
        }
    }

    fn visit_assign(&self, expr: &Assign) -> Result<Value, Error> {
        todo!()
    }
}