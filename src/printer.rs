use crate::expr::{Assign, Binary, ExprVisitor, Grouping, Literal, Unary};

pub struct AstPrinter;

impl ExprVisitor<()> for AstPrinter {
    fn visitLiteral(&self, expr: &Literal) -> () {
        match expr {
            Literal::String(s) => println!("{}", s),
            Literal::Number(n) => println!("{}", n),
            Literal::True => println!("true"),
            Literal::False => println!("false"),
            Literal::Nil => println!("nil"),
        }
    }
    
    fn visitGrouping(&self, expr: &Grouping) -> () {
        todo!()
    }

    fn visitUnary(&self, expr: &Unary) -> () {
        todo!()
    }

    fn visitBinary(&self, expr: &Binary) -> () {
        todo!()
    }

    fn visitAssign(&self, expr: &Assign) -> () {
        todo!()
    }
}