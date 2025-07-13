use crate::error::{Error, ErrorKind, Position};
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
    Ternary {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
}
#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Clone, Copy)]
pub enum Notation {
    Lisp,
    Rpn,
    Polish,
}

impl Expr {
    pub fn evaluate(&self) -> Result<Value, Error> {
        match self {
            Expr::Literal(lit) => Self::evaluate_literal(lit),
            Expr::Grouping { expr } => expr.evaluate(),
            Expr::Unary { operator, right } => Self::evaluate_unary(operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => Self::evaluate_binary(left, operator, right),
            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
            } => Self::evaluate_ternary(condition, then_expr, else_expr),
        }
    }

    fn evaluate_literal(lit: &Literal) -> Result<Value, Error> {
        Ok(match lit {
            Literal::Number(n) => Value::Number(f64::from(*n)),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Bool(b) => Value::Bool(*b),
            Literal::Nil => Value::Nil,
        })
    }

    fn evaluate_unary(operator: &Token, right: &Expr) -> Result<Value, Error> {
        let right_val = right.evaluate()?;
        match operator.kind {
            TokenKind::Minus => match right_val {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(Error::runtime(
                    "Operand must be a number.",
                    Position::new(operator.line, operator.column, operator.offset),
                )),
            },
            TokenKind::Bang => Ok(Value::Bool(!Value::is_truthy(&right_val))),
            _ => Err(Error::runtime(
                "Invalid unary operator.",
                Position::new(operator.line, operator.column, operator.offset),
            )),
        }
    }

    fn evaluate_binary(left: &Expr, operator: &Token, right: &Expr) -> Result<Value, Error> {
        let left_val = left.evaluate()?;
        let right_val = right.evaluate()?;
        match operator.kind {
            TokenKind::Plus => match (&left_val, &right_val) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(a.clone() + b)),
                _ => Err(Error::runtime(
                    "Operands must be two numbers or two strings",
                    Position::new(operator.line, operator.column, operator.offset),
                )),
            },
            TokenKind::Minus => Value::binary_number_operation(
                &left_val,
                &right_val,
                |a, b| a - b,
                Position::new(operator.line, operator.column, operator.offset),
            ),
            TokenKind::Star => Value::binary_number_operation(
                &left_val,
                &right_val,
                |a, b| a + b,
                Position::new(operator.line, operator.column, operator.offset),
            ),
            TokenKind::Slash => Value::binary_number_operation(
                &left_val,
                &right_val,
                |a, b| a / b,
                Position::new(operator.line, operator.column, operator.offset),
            ),
            _ => Err(Error::runtime(
                "Invalid binary operator",
                Position::new(operator.line, operator.column, operator.offset),
            )),
        }
    }

    fn evaluate_ternary(
        condition: &Expr,
        then_expr: &Expr,
        else_expr: &Expr,
    ) -> Result<Value, Error> {
        let condition_val = condition.evaluate()?;

        if Value::is_truthy(&condition_val) {
            Ok(then_expr.evaluate()?)
        } else {
            Ok(else_expr.evaluate()?)
        }
    }

    pub fn print(&self, notation: Notation) -> String {
        match self {
            Expr::Literal(lit) => match lit {
                Literal::Number(n) => n.to_string(),
                Literal::String(s) => format!("\"{}\"", s),
                Literal::Bool(b) => b.to_string(),
                Literal::Nil => "nil".into(),
            },

            Expr::Grouping { expr } => match notation {
                Notation::Lisp => format!("(group {})", expr.print(notation)),
                _ => expr.print(notation),
            },

            Expr::Unary { operator, right } => match notation {
                Notation::Rpn => format!("{} {}", right.print(notation), operator.lexeme),
                _ => format!("({} {})", operator.lexeme, right.print(notation)),
            },

            Expr::Binary {
                left,
                operator,
                right,
            } => match notation {
                Notation::Lisp => format!(
                    "({} {} {})",
                    operator.lexeme,
                    left.print(notation),
                    right.print(notation)
                ),
                Notation::Polish => format!(
                    "{} {} {}",
                    operator.lexeme,
                    left.print(notation),
                    right.print(notation)
                ),
                Notation::Rpn => format!(
                    "{} {} {}",
                    left.print(notation),
                    right.print(notation),
                    operator.lexeme
                ),
            },
            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
            } => match notation {
                Notation::Lisp => format!(
                    "(?: {} {} {})",
                    condition.print(notation),
                    then_expr.print(notation),
                    else_expr.print(notation)
                ),
                Notation::Polish => format!(
                    "?: {} {} {}",
                    condition.print(notation),
                    then_expr.print(notation),
                    else_expr.print(notation)
                ),
                Notation::Rpn => format!(
                    "{} {} {} ?:",
                    condition.print(notation),
                    then_expr.print(notation),
                    else_expr.print(notation)
                ),
            },
        }
    }
}
