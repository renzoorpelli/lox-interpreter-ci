use crate::error::{Error, ErrorKind, Result};

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

    /// function to make arithmetic operations only if values are numbers
    pub fn binary_number_operation<F>(l: &Value, r: &Value, op: F) -> Result<Value>
    where
        F: FnOnce(f64, f64) -> f64,
    {
        if let (Value::Number(l), Value::Number(r)) = (l, r) {
            Ok(Value::Number(op(*l, *r)))
        } else {
            Err(Error::new(
                ErrorKind::Runtime,
                "Operands must be numbers.",
                0,
                0,
            ))
        }
    }
}
