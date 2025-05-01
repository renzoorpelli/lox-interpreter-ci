use std::fmt::format;

use crate::error::{Error, ErrorKind, Result};
use crate::parser::value::Value;
use crate::token::{Token, TokenKind};

/*
   expression = literal | unary | binary | grouping;
   literal = NUMBER | STRING | "true" | "false" | "nill";
   grouping = "(" expression ")";
   unary = ( "-" | "|" ) expression;
   binary = expression operator expression;
   operator = "==" | "!=" | "<" | "<=" | ">" | ">=" | "+" | "-" | "*" | "/";
*/

// Box<Expr> provide known size at compile time
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}
#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}
impl Expr {
    pub fn evaluate(&self) -> Result<Value> {
        match self {
            Expr::Literal(lit) => Self::evaluate_literal(lit),
            Expr::Grouping { expr } => expr.evaluate(),
            Expr::Unary { operator, right } => Self::evaluate_unary(operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => Self::evaluate_binary(left, operator, right),
        }
    }

    fn evaluate_literal(lit: &Literal) -> Result<Value> {
        Ok(match lit {
            Literal::Number(n) => Value::Number(f64::from(*n)),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Bool(b) => Value::Bool(*b),
            Literal::Nil => Value::Nil,
        })
    }

    fn evaluate_unary(operator: &Token, right: &Expr) -> Result<Value> {
        let right_val = right.evaluate()?;
        match operator.kind {
            TokenKind::Minus => match right_val {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(Error::new(
                    ErrorKind::Runtime,
                    "Operand must be a number.",
                    0,
                    0,
                )),
            },
            TokenKind::Bang => Ok(Value::Bool(!Value::is_truthy(&right_val))),
            _ => Err(Error::new(
                ErrorKind::Runtime,
                "Invalid unary operator.",
                0,
                0,
            )),
        }
    }

    fn evaluate_binary(left: &Expr, operator: &Token, right: &Expr) -> Result<Value> {
        let left_val = left.evaluate()?;
        let right_val = right.evaluate()?;
        match operator.kind {
            TokenKind::Plus => match (&left_val, &right_val) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(a.clone() + b)),
                _ => Err(Error::new(
                    ErrorKind::Runtime,
                    "Operands must be two numbers or two strings",
                    0,
                    0,
                )),
            },
            TokenKind::Minus => Value::binary_number_operation(&left_val, &right_val, |a, b| a - b),
            TokenKind::Star => Value::binary_number_operation(&left_val, &right_val, |a, b| a + b),
            TokenKind::Slash => Value::binary_number_operation(&left_val, &right_val, |a, b| a / b),
            _ => Err(Error::new(
                ErrorKind::Runtime,
                "Invalid binary operator",
                0,
                0,
            )),
        }
    }

    pub fn print(&self) -> String {
        match self {
            Expr::Literal(lit) => match lit {
                Literal::Number(n) => n.to_string(),
                Literal::String(s) => format!("\"{}\"", s),
                Literal::Bool(b) => b.to_string(),
                Literal::Nil => "nil".into(),
            },
            Expr::Grouping { expr } => format!("(group {})", expr.print()),
            Expr::Unary { operator, right } => format!("({} {})", operator.value, right.print()),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                format!("({} {} {})", operator.value, left.print(), right.print())
            }
        }
    }
}
