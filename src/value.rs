use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Nil => true,
            Value::Boolean(false) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num),
            Value::Boolean(value) => write!(f, "{}", value),
            Value::Nil => write!(f, "nil"),
        }
    }
}
