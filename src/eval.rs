use std::{cell::RefCell, collections::HashMap, fmt::Display};

use anyhow::{Error, Result};

use crate::{
    expr::{Assign, Binary, Expr, ExprVisitor, Grouping, Literal, Unary},
    stmt::{Expression, Print, Stmt, StmtVisitor, Var},
    token::{Number, TokenValue},
    Walkable,
};

#[derive(Clone)]
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

pub struct Interpreter {
    env: RefCell<HashMap<String, Value>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: RefCell::new(HashMap::new()),
        }
    }

    pub fn interpret(&self, exprs: Vec<Expr>) -> Result<Vec<Value>, Vec<Error>> {
        let mut values = Vec::new();
        let mut errors = Vec::new();
        for expr in exprs {
            match expr.walk(self) {
                Ok(value) => values.push(value),
                Err(e) => errors.push(e),
            }
        }
        if !errors.is_empty() {
            return Err(std::mem::take(&mut errors));
        }
        Ok(values)
    }

    pub fn execute(&self, stmts: Vec<Stmt>) -> Result<(), Error> {
        for stmt in stmts {
            stmt.walk(self)?;
        }
        Ok(())
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
                    Err(Error::msg(format!(
                        "Operand must be a number.\n[line {}]",
                        expr.operator.line
                    )))
                }
            }
            TokenValue::Bang => match right {
                Value::Boolean(b) => Ok(Value::Boolean(!b)),
                Value::Nil => Ok(Value::Boolean(true)),
                _ => Ok(Value::Boolean(false)),
            },
            _ => unreachable!(),
        }
    }

    fn visit_binary(&self, expr: &Binary) -> Result<Value, Error> {
        let left = expr.left.walk(self)?;
        let right = expr.right.walk(self)?;
        match expr.operator.value {
            TokenValue::Plus => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
                _ => Err(Error::msg(format!(
                    "Operands must be two numbers or two strings.\n[line {}]",
                    expr.operator.line
                ))),
            },
            TokenValue::Minus => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Number(l - r))
                } else {
                    Err(Error::msg(format!(
                        "Operands must be numbers.\n[line {}]",
                        expr.operator.line
                    )))
                }
            }
            TokenValue::Star => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Number(l * r))
                } else {
                    Err(Error::msg(format!(
                        "Operands must be numbers.\n[line {}]",
                        expr.operator.line
                    )))
                }
            }
            TokenValue::Slash => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Number(l / r))
                } else {
                    Err(Error::msg(format!(
                        "Operands must be numbers.\n[line {}]",
                        expr.operator.line
                    )))
                }
            }
            TokenValue::Greater => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Boolean(l > r))
                } else {
                    Err(Error::msg(format!(
                        "Operands must be numbers.\n[line {}]",
                        expr.operator.line
                    )))
                }
            }
            TokenValue::GreaterEqual => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Boolean(l >= r))
                } else {
                    Err(Error::msg(format!(
                        "Operands must be numbers.\n[line {}]",
                        expr.operator.line
                    )))
                }
            }
            TokenValue::Less => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Boolean(l < r))
                } else {
                    Err(Error::msg(format!(
                        "Operands must be numbers.\n[line {}]",
                        expr.operator.line
                    )))
                }
            }
            TokenValue::LessEqual => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Ok(Value::Boolean(l <= r))
                } else {
                    Err(Error::msg(format!(
                        "Operands must be numbers.\n[line {}]",
                        expr.operator.line
                    )))
                }
            }
            TokenValue::EqualEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l == r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l == r)),
                (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
                (Value::Nil, Value::Nil) => Ok(Value::Boolean(true)),

                _ => Ok(Value::Boolean(false)),
            },
            TokenValue::BangEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l != r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l != r)),
                (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l != r)),
                (Value::Nil, Value::Nil) => Ok(Value::Boolean(false)),

                _ => Ok(Value::Boolean(true)),
            },
            _ => unreachable!(),
        }
    }

    fn visit_assign(&self, expr: &Assign) -> Result<Value, Error> {
        let new_value = expr.value.walk(self)?;
        self.env
            .borrow_mut()
            .entry(expr.name.lexeme.clone())
            .and_modify(|value| *value = new_value.clone())
            .or_insert(new_value.clone());
        Ok(new_value)
    }

    fn visit_variable(&self, expr: &crate::expr::Variable) -> Result<Value, Error> {
        self.env
            .borrow()
            .get(&expr.name.lexeme)
            .cloned()
            .ok_or(Error::msg(format!(
                "Undefined variable '{}'.\n[line {}]",
                expr.name.lexeme,
                expr.name.line
            )))
    }
}

impl StmtVisitor<Result<(), Error>> for Interpreter {
    fn visit_print(&self, stmt: &Print) -> Result<(), Error> {
        let value = stmt.expr.walk(self)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_expression(&self, stmt: &Expression) -> Result<(), Error> {
        stmt.expr.walk(self)?;
        Ok(())
    }

    fn visit_var(&self, stmt: &Var) -> Result<(), Error> {
        let value = stmt
            .initializer
            .as_ref();
        if let Some(value) = value {
            let value = value.walk(self)?;
            self.env
                .borrow_mut()
                .entry(stmt.name.lexeme.clone())
                .and_modify(|v| *v = value.clone())
                .or_insert(value.clone());
        } else {
            self.env
                .borrow_mut()
                .entry(stmt.name.lexeme.clone())
                .or_insert(Value::Nil);
        }
        Ok(())
    }
}
