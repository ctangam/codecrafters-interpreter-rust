use crate::{token::Token, Walkable};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Binary(Binary),
    Assign(Assign),
    Variable(Variable),
    Call(Call),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: Token,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
    pub paren: Token,
}

impl<V: ExprVisitor<T>, T> Walkable<V, T> for Expr {
    fn walk(&self, visitor: &mut V) -> T {
        match self {
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Assign(assign) => visitor.visit_assign(assign),
            Expr::Variable(variable) => visitor.visit_variable(variable),
            Expr::Call(function) => visitor.visit_call(function),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_literal(&mut self, expr: &Literal) -> T;

    fn visit_grouping(&mut self, expr: &Grouping) -> T;

    fn visit_unary(&mut self, expr: &Unary) -> T;

    fn visit_binary(&mut self, expr: &Binary) -> T;

    fn visit_assign(&mut self, expr: &Assign) -> T;

    fn visit_variable(&mut self, expr: &Variable) -> T;

    fn visit_call(&mut self, expr: &Call) -> T;
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
            }
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
            Expr::Call(Call { callee, args , ..}) => {
                write!(
                    f,
                    "(fn {} {})",
                    callee.to_string(),
                    args.iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
        }
    }
}
