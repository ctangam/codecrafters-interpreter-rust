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
                    Err(Error::msg(format!("[line {}] Error: Expect number.", expr.operator.line)))
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
        todo!()
    }

    fn visit_assign(&self, expr: &Assign) -> Result<Value, Error> {
        todo!()
    }
}