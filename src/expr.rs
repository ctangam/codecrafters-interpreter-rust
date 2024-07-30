use crate::{token::Token, Walkable};

pub enum Expr {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Binary(Binary),
    Assign(Assign),
}

pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Nil
}

pub struct Grouping {
    pub expr: Box<Expr>,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

impl<V: ExprVisitor<T>, T> Walkable<V, T> for Expr {
    fn walk(&self, visitor: &V) -> T {
        match self {
            Expr::Literal(literal) => visitor.visitLiteral(literal),
            Expr::Grouping(grouping) => visitor.visitGrouping(grouping),
            Expr::Unary(unary) => visitor.visitUnary(unary),
            Expr::Binary(binary) => visitor.visitBinary(binary),
            Expr::Assign(assign) => visitor.visitAssign(assign),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visitLiteral(&self, expr: &Literal) -> T;

    fn visitGrouping(&self, expr: &Grouping) -> T;

    fn visitUnary(&self, expr: &Unary) -> T;

    fn visitBinary(&self, expr: &Binary) -> T;

    fn visitAssign(&self, expr: &Assign) -> T;
}


impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(Literal::Number(n)) => n.fmt(f),
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
            }) => f.write_fmt(format_args!("({} {} {})", left, operator, right)),
            
            Expr::Grouping(Grouping { expr }) => f.write_fmt(format_args!("({})", expr)),
            Expr::Assign(Assign { name, value }) => write!(f, "{} = {}", name, value),
        
        }
    }
}