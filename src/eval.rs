use std::{cell::RefCell, collections::HashMap, fmt::Display, hash::Hash, rc::Rc};

use anyhow::{Error, Result};
use thiserror::Error;

use crate::{
    expr::{Assign, Binary, Expr, ExprVisitor, Grouping, Literal, Unary},
    stmt::{Block, Expression, For, If, Print, Stmt, StmtVisitor, Var, While},
    token::{Number, Token, TokenValue},
    Walkable,
};

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("{0}")]
    Return(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(Number),
    String(String),
    Function(LoxFunction),
    RustFunction(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoxFunction {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
    pub closure: Rc<RefCell<Vec<HashMap<String, Value>>>>,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Function(func) => write!(f, "<fn {}>", func.name.lexeme),
            Value::RustFunction(s) => write!(f, "fn {}>", s),
        }
    }
}

pub struct Interpreter {
    env: Vec<HashMap<String, Value>>,
    rets: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut env = HashMap::new();
        env.insert("clock".into(), Value::RustFunction("clock".into()));
        Interpreter {
            env: vec![env],
            rets: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, exprs: Vec<Expr>) -> Result<Vec<Value>, Vec<Error>> {
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

    pub fn execute(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
        for stmt in stmts {
            stmt.walk(self)?;
        }
        Ok(())
    }
}

impl ExprVisitor<Result<Value, Error>> for Interpreter {
    fn visit_literal(&mut self, expr: &Literal) -> Result<Value, Error> {
        match expr {
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Number(n) => Ok(Value::Number(n.clone())),
            Literal::True => Ok(Value::Boolean(true)),
            Literal::False => Ok(Value::Boolean(false)),
            Literal::Nil => Ok(Value::Nil),
        }
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Result<Value, Error> {
        expr.expr.walk(self)
    }

    fn visit_unary(&mut self, expr: &Unary) -> Result<Value, Error> {
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

    fn visit_binary(&mut self, expr: &Binary) -> Result<Value, Error> {
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
            },
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

    fn visit_assign(&mut self, expr: &Assign) -> Result<Value, Error> {
        let new_value = expr.value.walk(self)?;
        if let Some(v) = self
            .env
            .iter_mut()
            .rev()
            .filter_map(|env| env.get_mut(&expr.name.lexeme))
            .nth(0)
        {
            *v = new_value.clone();
        }
        Ok(new_value)
    }

    fn visit_variable(&mut self, expr: &crate::expr::Variable) -> Result<Value, Error> {
        self.env
            .iter()
            .rev()
            .find_map(|env| env.get(&expr.name.lexeme))
            .cloned()
            .ok_or(Error::msg(format!(
                "Undefined variable '{}'.\n[line {}]",
                expr.name.lexeme, expr.name.line
            )))
    }

    fn visit_call(&mut self, expr: &crate::expr::Call) -> Result<Value, Error> {
        match expr.callee.walk(self)? {
            Value::Function(value) => {
                let LoxFunction {
                    name,
                    params,
                    body,
                    closure,
                } = value;

                if params.len() != expr.args.len() {
                    return Err(Error::msg(format!(
                        "Expected {} arguments but got {}.\n[line {}]",
                        params.len(),
                        expr.args.len(),
                        expr.paren.line
                    )));
                }
                let mut new_env = HashMap::new();
                let mut args = Vec::new();
                expr.args
                    .iter()
                    .zip(params.iter())
                    .for_each(|(arg, param)| {
                        let arg = arg.walk(self).unwrap();
                        args.push(arg.clone());
                        new_env.insert(param.lexeme.clone(), arg);
                    });

                let args = args
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let func_key = format!("{}({})", &name.lexeme, args);
                if let Some(ret) = self.rets.get(&func_key) {
                    return Ok(ret.clone());
                }

                let old_env = self.env.clone();
                self.env = closure.borrow().clone();

                self.env.push(new_env);

                let mut ret = "return".to_string();
                for stmt in body.iter() {
                    match stmt.walk(self) {
                        Ok(_) => {}
                        Err(e) => ret = e.to_string(),
                    }
                }

                let ret = self
                    .env
                    .pop()
                    .unwrap()
                    .get(&ret)
                    .unwrap_or(&Value::Nil)
                    .clone();

                if let Value::Number(_) = &ret {
                    self.rets.insert(func_key, ret.clone());
                }

                closure.replace(self.env.clone());
                self.env = old_env;

                return Ok(ret);
            }
            Value::RustFunction(s) if &s == "clock" => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                return Ok(Value::Number(now as f64));
            }
            _ => Err(Error::msg(format!(
                "Can only call functions and classes.\n[line {}]",
                expr.paren.line
            ))),
        }
    }
}

impl StmtVisitor<Result<(), Error>> for Interpreter {
    fn visit_print(&mut self, stmt: &Print) -> Result<(), Error> {
        let value = stmt.expr.walk(self)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_expression(&mut self, stmt: &Expression) -> Result<(), Error> {
        stmt.expr.walk(self)?;
        Ok(())
    }

    fn visit_var(&mut self, stmt: &Var) -> Result<(), Error> {
        let value = stmt.initializer.as_ref();
        if let Some(value) = value {
            let value = value.walk(self)?;
            self.env
                .last_mut()
                .unwrap()
                .entry(stmt.name.lexeme.clone())
                .and_modify(|v| *v = value.clone())
                .or_insert(value.clone());
        } else {
            self.env
                .last_mut()
                .unwrap()
                .entry(stmt.name.lexeme.clone())
                .or_insert(Value::Nil);
        }
        Ok(())
    }

    fn visit_block(&mut self, stmt: &Block) -> Result<(), Error> {
        self.env.push(HashMap::new());
        let _ = self.execute(&stmt.statements);
        let env = self.env.pop().unwrap();
        if let Some(ret) = env.get("return") {
            self.env
                .last_mut()
                .unwrap()
                .insert("return".into(), ret.clone());
        }
        Ok(())
    }

    fn visit_if(&mut self, stmt: &If) -> Result<(), Error> {
        if Value::Boolean(false) != stmt.condition.walk(self)? {
            stmt.then_branch.walk(self)?;
        } else if let Some(else_branch) = &stmt.else_branch {
            else_branch.walk(self)?;
        }
        Ok(())
    }

    fn visit_while(&mut self, stmt: &While) -> Result<(), Error> {
        while Value::Boolean(false) != stmt.condition.walk(self)? {
            stmt.body.walk(self)?;
        }
        Ok(())
    }

    fn visit_for(&mut self, stmt: &For) -> Result<(), Error> {
        if let Some(init) = &stmt.init {
            init.walk(self)?;
        }
        while Value::Boolean(false) != stmt.condition.walk(self)? {
            stmt.body.walk(self)?;
            if let Some(update) = &stmt.update {
                update.walk(self)?;
            }
        }
        Ok(())
    }

    fn visit_func(&mut self, stmt: &crate::stmt::Func) -> Result<(), Error> {
        let closure = Rc::new(RefCell::new(self.env.clone()));
        closure
            .borrow_mut()
            .last_mut()
            .unwrap()
            .entry(stmt.name.lexeme.clone())
            .or_insert(Value::Function(LoxFunction {
                name: stmt.name.clone(),
                params: stmt.params.clone(),
                body: stmt.body.clone(),
                closure: closure.clone(),
            }));
        self.env
            .last_mut()
            .unwrap()
            .entry(stmt.name.lexeme.clone())
            .or_insert(Value::Function(LoxFunction {
                name: stmt.name.clone(),
                params: stmt.params.clone(),
                body: stmt.body.clone(),
                closure: closure.clone(),
            }));
        Ok(())
    }

    fn visit_return(&mut self, stmt: &crate::stmt::Return) -> Result<(), Error> {
        let value = stmt.value.as_ref();
        if let Some(value) = value {
            let value = value.walk(self)?;
            self.env
                .last_mut()
                .unwrap()
                .entry("return".to_string())
                .or_insert(value.clone());
        } else {
            self.env
                .last_mut()
                .unwrap()
                .entry("return".to_string())
                .or_insert(Value::Nil);
        }

        return Err(EvalError::Return("return".into()).into());
    }
}
