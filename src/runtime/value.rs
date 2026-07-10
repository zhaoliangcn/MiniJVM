use std::fmt;
use crate::error::RuntimeError;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Byte(i8),
    Short(i16),
    Char(u16),
    Boolean(bool),
    ObjectRef(usize),
    ArrayRef(usize),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v),
            Value::Long(v) => write!(f, "{}L", v),
            Value::Float(v) => write!(f, "{}f", v),
            Value::Double(v) => write!(f, "{}d", v),
            Value::Byte(v) => write!(f, "{}", v),
            Value::Short(v) => write!(f, "{}", v),
            Value::Char(v) => write!(f, "'{}'", char::from_u32(*v as u32).unwrap_or('?')),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::ObjectRef(idx) => write!(f, "Ref@{}", idx),
            Value::ArrayRef(idx) => write!(f, "ArrayRef@{}", idx),
            Value::Null => write!(f, "null"),
        }
    }
}

impl Value {
    pub fn as_int(&self) -> i32 {
        match self {
            Value::Int(v) => *v,
            Value::Long(v) => *v as i32,
            Value::Float(v) => *v as i32,
            Value::Double(v) => *v as i32,
            Value::Byte(v) => *v as i32,
            Value::Short(v) => *v as i32,
            Value::Char(v) => *v as i32,
            Value::Boolean(v) => if *v { 1 } else { 0 },
            _ => panic!("Type mismatch: expected numeric type"),
        }
    }

    pub fn as_long(&self) -> i64 {
        match self {
            Value::Int(v) => *v as i64,
            Value::Long(v) => *v,
            Value::Float(v) => *v as i64,
            Value::Double(v) => *v as i64,
            Value::Byte(v) => *v as i64,
            Value::Short(v) => *v as i64,
            Value::Char(v) => *v as i64,
            Value::Boolean(v) => if *v { 1 } else { 0 },
            _ => panic!("Type mismatch: expected numeric type"),
        }
    }

    pub fn as_float(&self) -> f32 {
        match self {
            Value::Int(v) => *v as f32,
            Value::Long(v) => *v as f32,
            Value::Float(v) => *v,
            Value::Double(v) => *v as f32,
            Value::Byte(v) => *v as f32,
            Value::Short(v) => *v as f32,
            Value::Char(v) => *v as f32,
            Value::Boolean(v) => if *v { 1.0 } else { 0.0 },
            _ => panic!("Type mismatch: expected numeric type"),
        }
    }

    pub fn as_double(&self) -> f64 {
        match self {
            Value::Int(v) => *v as f64,
            Value::Long(v) => *v as f64,
            Value::Float(v) => *v as f64,
            Value::Double(v) => *v,
            Value::Byte(v) => *v as f64,
            Value::Short(v) => *v as f64,
            Value::Char(v) => *v as f64,
            Value::Boolean(v) => if *v { 1.0 } else { 0.0 },
            _ => panic!("Type mismatch: expected numeric type"),
        }
    }

    pub fn as_ref(&self) -> usize {
        match self {
            Value::ObjectRef(idx) => *idx,
            Value::ArrayRef(idx) => *idx,
            _ => panic!("Type mismatch: expected reference type"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Boolean(v) => *v,
            Value::Int(v) => *v != 0,
            Value::Long(v) => *v != 0,
            Value::Float(v) => *v != 0.0,
            Value::Double(v) => *v != 0.0,
            Value::Byte(v) => *v != 0,
            Value::Short(v) => *v != 0,
            Value::Char(v) => *v != 0,
            _ => panic!("Type mismatch: expected boolean or numeric type"),
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, Value::ObjectRef(_) | Value::ArrayRef(_) | Value::Null)
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Value::Int(_) | Value::Long(_) | Value::Float(_) | Value::Double(_) 
            | Value::Byte(_) | Value::Short(_) | Value::Char(_) | Value::Boolean(_))
    }
}
