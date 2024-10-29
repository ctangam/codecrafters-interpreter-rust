use crate::{expr::Expr, token::Token, Walkable};

pub enum Stmt {
    Print(Print),
    Expression(Expression),
    Var(Var),
    Block(Block),
    If(If),
    While(While),
}

pub struct Print {
    pub expr: Box<Expr>,
}

pub struct Expression {
    pub expr: Box<Expr>,
}

pub struct Var {
    pub name: Token,
    pub initializer: Option<Box<Expr>>,
}

pub struct Block {
    pub statements: Vec<Stmt>,
}

pub struct If {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct While {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
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
    
}