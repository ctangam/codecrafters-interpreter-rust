use crate::{expr::Expr, Walkable};

pub enum Stmt {
    Print(Print),
    Expression(Expression),
}

pub struct Print {
    pub expr: Box<Expr>,
}

pub struct Expression {
    pub expr: Box<Expr>,
}

impl<V: StmtVisitor<T>, T> Walkable<V, T> for Stmt {
    fn walk(&self, visitor: &V) -> T {
        match self {
            Stmt::Print(print) => visitor.visit_print(print),
            Stmt::Expression(expression) => visitor.visit_expression(expression),
        }
    }
}

pub trait StmtVisitor<T> {

    fn visit_print(&self, stmt: &Print) -> T;

    fn visit_expression(&self, stmt: &Expression) -> T;
    
}