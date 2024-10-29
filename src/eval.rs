use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    collections::HashMap,
    fmt::Display,
};

use anyhow::{Error, Result};

use crate::{
    expr::{Assign, Binary, Expr, ExprVisitor, Grouping, Literal, Unary},
    stmt::{Block, Expression, If, Print, Stmt, StmtVisitor, Var, While},
    token::{Number, TokenValue},
    Walkable,
};

#[derive(Clone, Debug, PartialEq)]
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
    env: RefCell<Vec<HashMap<String, Value>>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: RefCell::new(vec![HashMap::new()]),
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

    pub fn execute(&self, stmts: &Vec<Stmt>) -> Result<(), Error> {
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
        let right = expr.right.walk(self)?;
        let left = expr.left.walk(self)?;
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
            TokenValue::Or => match (left, right) {
                (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l || r)),
                (Value::Boolean(_), v) => Ok(v),
                (v, Value::Boolean(_)) => Ok(v),
                _ => Ok(Value::Boolean(true)),
            }
            TokenValue::And => match (left, right) {
                (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l && r)),
                (Value::Boolean(false), _) => Ok(Value::Boolean(false)),
                (Value::Boolean(true), v) => Ok(v),
                (_, Value::Boolean(r)) => Ok(Value::Boolean(r)),
                (Value::Nil, _) => Ok(Value::Boolean(false)),
                (_, Value::Nil) => Ok(Value::Boolean(false)),
                (_, r) => Ok(r),
            },
            _ => unreachable!(),
        }
    }

    fn visit_assign(&self, expr: &Assign) -> Result<Value, Error> {
        let new_value = expr.value.walk(self)?;
        if let Some(v) = self
            .env
            .borrow_mut()
            .iter_mut()
            .rev()
            .filter_map(|env| env.get_mut(&expr.name.lexeme))
            .nth(0)
        {
            *v = new_value.clone();
        }
        Ok(new_value)
    }

    fn visit_variable(&self, expr: &crate::expr::Variable) -> Result<Value, Error> {
        self.env
            .borrow()
            .iter()
            .rev()
            .filter_map(|env| env.get(&expr.name.lexeme))
            .nth(0)
            .cloned()
            .ok_or(Error::msg(format!(
                "Undefined variable '{}'.\n[line {}]",
                expr.name.lexeme, expr.name.line
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
        let value = stmt.initializer.as_ref();
        if let Some(value) = value {
            let value = value.walk(self)?;
            self.env
                .borrow_mut()
                .last_mut()
                .unwrap()
                .entry(stmt.name.lexeme.clone())
                .and_modify(|v| *v = value.clone())
                .or_insert(value.clone());
        } else {
            self.env
                .borrow_mut()
                .last_mut()
                .unwrap()
                .entry(stmt.name.lexeme.clone())
                .or_insert(Value::Nil);
        }
        Ok(())
    }

    fn visit_block(&self, stmt: &Block) -> Result<(), Error> {
        self.env.borrow_mut().push(HashMap::new());
        let _ = self.execute(&stmt.statements);
        self.env.borrow_mut().pop();
        Ok(())
    }

    fn visit_if(&self, stmt: &If) -> Result<(), Error> {
        if Value::Boolean(false) != stmt.condition.walk(self)? {
            stmt.then_branch.walk(self)?;
        } else if let Some(else_branch) = &stmt.else_branch {
            else_branch.walk(self)?;
        }
        Ok(())
    }

    fn visit_while(&self, stmt: &While) -> Result<(), Error> {
        while Value::Boolean(false) != stmt.condition.walk(self)? {
            stmt.body.walk(self)?;
        }
        Ok(())
    }
}
