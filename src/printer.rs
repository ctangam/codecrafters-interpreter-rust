use crate::expr::{Assign, Binary, ExprVisitor, Grouping, Literal, Unary, Variable};

pub struct AstPrinter;

impl ExprVisitor<()> for AstPrinter {
    fn visit_literal(&self, expr: &Literal) -> () {
        match expr {
            Literal::String(s) => println!("{}", s),
            Literal::Number(n) => println!("{}", n),
            Literal::True => println!("true"),
            Literal::False => println!("false"),
            Literal::Nil => println!("nil"),
        }
    }

    fn visit_grouping(&self, expr: &Grouping) -> () {
        todo!()
    }

    fn visit_unary(&self, expr: &Unary) -> () {
        todo!()
    }

    fn visit_binary(&self, expr: &Binary) -> () {
        todo!()
    }

    fn visit_assign(&self, expr: &Assign) -> () {
        todo!()
    }

    fn visit_variable(&self, expr: &Variable) -> () {
        todo!()
    }

    fn visit_call(&self, expr: &crate::expr::Call) -> () {
        todo!()
    }
}
