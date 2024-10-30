use crate::{token::Token, Walkable};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Binary(Binary),
    Assign(Assign),
    Variable(Variable),
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Nil
}

#[derive(Debug)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug)]
pub struct Variable {
    pub name: Token,
}

impl<V: ExprVisitor<T>, T> Walkable<V, T> for Expr {
    fn walk(&self, visitor: &V) -> T {
        match self {
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Assign(assign) => visitor.visit_assign(assign),
            Expr::Variable(variable) => visitor.visit_variable(variable),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_literal(&self, expr: &Literal) -> T;

    fn visit_grouping(&self, expr: &Grouping) -> T;

    fn visit_unary(&self, expr: &Unary) -> T;

    fn visit_binary(&self, expr: &Binary) -> T;

    fn visit_assign(&self, expr: &Assign) -> T;

    fn visit_variable(&self, expr: &Variable) -> T;
}


impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(Literal::Number(n)) => {
                if n % 1.0 == 0.0 {
                    write!(f, "{:.1}", n)
                } else {
                    write!(f, "{}", n)
                }
            },
            Expr::Literal(Literal::String(s)) => s.fmt(f),
            Expr::Literal(Literal::False) => false.fmt(f),
            Expr::Literal(Literal::True) => true.fmt(f),
            Expr::Literal(Literal::Nil) => f.write_str("nil"),
            Expr::Unary(Unary { operator, right }) => {
                f.write_fmt(format_args!("({} {})", operator.lexeme, right))
            }
            Expr::Binary(Binary {
                left,
                operator,
                right,
            }) => f.write_fmt(format_args!("({} {} {})", operator.lexeme, left, right)),
            
            Expr::Grouping(Grouping { expr }) => f.write_fmt(format_args!("(group {})", expr)),
            Expr::Assign(Assign { name, value }) => write!(f, "(= {} {})", name.lexeme, value),
            Expr::Variable(Variable { name }) => name.lexeme.fmt(f),
        }
    }
}