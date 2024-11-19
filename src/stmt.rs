use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{eval::Value, expr::Expr, token::Token, Walkable};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Print(Print),
    Expression(Expression),
    Var(Var),
    Block(Block),
    If(If),
    While(While),
    For(For),
    Func(Func),
    Return(Return),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Print {
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Box<Expr>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct If {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct While {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct For {
    pub init: Option<Box<Stmt>>,
    pub condition: Box<Expr>,
    pub update: Option<Box<Expr>>,
    pub body: Box<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Func {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub value: Option<Expr>,
}

impl<V: StmtVisitor<T>, T> Walkable<V, T> for Stmt {
    fn walk(&self, visitor: &V) -> T {
        match self {
            Stmt::Print(print) => visitor.visit_print(print),
            Stmt::Expression(expression) => visitor.visit_expression(expression),
            Stmt::Var(var) => visitor.visit_var(var),
            Stmt::Block(block) => visitor.visit_block(block),
            Stmt::If(if_stmt) => visitor.visit_if(if_stmt),
            Stmt::While(while_stmt) => visitor.visit_while(while_stmt),
            Stmt::For(for_stmt) => visitor.visit_for(for_stmt),
            Stmt::Func(func) => visitor.visit_func(func),
            Stmt::Return(ret) => visitor.visit_return(ret),
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_print(&self, stmt: &Print) -> T;

    fn visit_expression(&self, stmt: &Expression) -> T;

    fn visit_var(&self, stmt: &Var) -> T;

    fn visit_block(&self, stmt: &Block) -> T;

    fn visit_if(&self, stmt: &If) -> T;

    fn visit_while(&self, stmt: &While) -> T;

    fn visit_for(&self, stmt: &For) -> T;

    fn visit_func(&self, stmt: &Func) -> T;

    fn visit_return(&self, stmt: &Return) -> T;
}
