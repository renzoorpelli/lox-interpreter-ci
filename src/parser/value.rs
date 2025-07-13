use crate::error::{Error, ErrorKind, Position};

/// separation of concerns
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}
impl Value {
    /// function to evaluate boolean values
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Bool(_) => "boolean",
            Value::Nil => "nil",
        }
    }

    /// function to make arithmetic operations only if values are numbers
    pub fn binary_number_operation<F>(left: &Value, right: &Value, op: F, position: Position) -> Result<Value, Error>
    where
        F: FnOnce(f64, f64) -> f64,
    {
        if let (Value::Number(l), Value::Number(r)) = (left, right) {
            Ok(Value::Number(op(*l, *r)))
        } else {
            Err(Error::invalid_operand_types(
                "Operands must be numbers.",
                left.type_name(),
                right.type_name(),
                position,
            ))
        }
    }
}
