use std::fmt::Display;

use anyhow::{Result, Error};

use crate::{expr::{Assign, Binary, Expr, ExprVisitor, Grouping, Literal, Unary}, token::Number, Walkable};

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
    fn visitLiteral(&self, expr: &Literal) -> Result<Value, Error> {
        match expr {
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Number(n) => Ok(Value::Number(n.clone())),
            Literal::True => Ok(Value::Boolean(true)),
            Literal::False => Ok(Value::Boolean(false)),
            Literal::Nil => Ok(Value::Nil),
        }
    }

    fn visitGrouping(&self, expr: &Grouping) -> Result<Value, Error> {
        todo!()
    }

    fn visitUnary(&self, expr: &Unary) -> Result<Value, Error> {
        todo!()
    }

    fn visitBinary(&self, expr: &Binary) -> Result<Value, Error> {
        todo!()
    }

    fn visitAssign(&self, expr: &Assign) -> Result<Value, Error> {
        todo!()
    }
}