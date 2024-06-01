/*
    Value
*/

use std::fmt;
use crate::parser::uf64::*;

// Value
#[derive(PartialEq, PartialOrd)]
pub enum Value {
    Int(i64),
    UInt(u64),
    Float(f64),
    UFloat(uf64),
    String(String),
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Int(val)        => write!(f, "{}", val),
            Value::UInt(val)       => write!(f, "{}", val),
            Value::Float(val)      => write!(f, "{}", val),
            Value::UFloat(val)     => write!(f, "{}", val),
            Value::String(ref val) => write!(f, "{}", val),
        }
    }
}
use std::ops::{Add, Sub, Div, Mul};
impl Add for Value {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            // Int
            (Value::Int(x),  Value::Int(y))  => Value::Int (x+y),
            // UInt
            (Value::UInt(x), Value::UInt(y)) => Value::UInt(x+y),
            // Int UInt
            (Value::UInt(x), Value::Int(y))  => Value::Int(x as i64 +y),
            (Value::Int(x),  Value::UInt(y)) => Value::Int(x+ y as i64),

            // Float
            (Value::Float(x),  Value::Float(y))  => Value::Float (x+y),
            // UFloat
            (Value::UFloat(x), Value::UFloat(y)) => Value::UFloat(x+y),
            // Float UFloat
            (Value::UFloat(x), Value::Float(y))  => Value::Float(f64::from(x) +y),
            (Value::Float(x),  Value::UFloat(y)) => Value::Float(x+ f64::from(y)),

            // Int Float
            (Value::Int(x),   Value::Float(y)) => Value::Float(x as f64 +y),
            (Value::Float(x), Value::Int(y))   => Value::Float(x+ y as f64),
            // Int UFloat
            (Value::Int(x),    Value::UFloat(y)) => Value::Float(x as f64 +f64::from(y)),
            (Value::UFloat(x), Value::Int(y))    => Value::Float(f64::from(x)+ y as f64),

            // UInt UFloat
            (Value::UInt(x),  Value::Float(y)) => Value::Float(x as f64 +y),
            (Value::Float(x), Value::UInt(y))  => Value::Float(x+ y as f64),
            // UInt UFloat
            (Value::UInt(x),   Value::UFloat(y)) => Value::UFloat(uf64::from(x) +y),
            (Value::UFloat(x), Value::UInt(y))   => Value::UFloat(x+ uf64::from(y)),

            // String
            (Value::String(x), Value::String(y)) => Value::String(x+&y),

            //
            _ => panic!("Unsupported operation: addition with mixed types"),
        }
    }
}
impl Sub for Value {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            // Int
            (Value::Int(x),  Value::Int(y))  => Value::Int (x-y),
            // UInt
            (Value::UInt(x), Value::UInt(y)) => Value::UInt(x-y),
            // Int UInt
            (Value::UInt(x), Value::Int(y))  => Value::Int(x as i64 -y),
            (Value::Int(x),  Value::UInt(y)) => Value::Int(x- y as i64),

            // Float
            (Value::Float(x),  Value::Float(y))  => Value::Float (x-y),
            // UFloat
            (Value::UFloat(x), Value::UFloat(y)) => Value::UFloat(x-y),
            // Float UFloat
            (Value::UFloat(x), Value::Float(y))  => Value::Float(f64::from(x) -y),
            (Value::Float(x),  Value::UFloat(y)) => Value::Float(x- f64::from(y)),

            // Int Float
            (Value::Int(x),   Value::Float(y)) => Value::Float(x as f64 -y),
            (Value::Float(x), Value::Int(y))   => Value::Float(x- y as f64),
            // Int UFloat
            (Value::Int(x),    Value::UFloat(y)) => Value::Float(x as f64 -f64::from(y)),
            (Value::UFloat(x), Value::Int(y))    => Value::Float(f64::from(x)- y as f64),

            // UInt UFloat
            (Value::UInt(x),  Value::Float(y)) => Value::Float(x as f64 -y),
            (Value::Float(x), Value::UInt(y))  => Value::Float(x- y as f64),
            // UInt UFloat
            (Value::UInt(x),   Value::UFloat(y)) => Value::UFloat(uf64::from(x) -y),
            (Value::UFloat(x), Value::UInt(y))   => Value::UFloat(x- uf64::from(y)),

            //
            _ => panic!("Unsupported operation: subtraction with mixed types"),
        }
    }
}
impl Div for Value {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match (self, other) {
            // Int
            (Value::Int(x),  Value::Int(y))  => Value::Int (x/y),
            // UInt
            (Value::UInt(x), Value::UInt(y)) => Value::UInt(x/y),
            // Int UInt
            (Value::UInt(x), Value::Int(y))  => Value::Int(x as i64 /y),
            (Value::Int(x),  Value::UInt(y)) => Value::Int(x/ y as i64),

            // Float
            (Value::Float(x),  Value::Float(y))  => Value::Float (x/y),
            // UFloat
            (Value::UFloat(x), Value::UFloat(y)) => Value::UFloat(x/y),
            // Float UFloat
            (Value::UFloat(x), Value::Float(y))  => Value::Float(f64::from(x) /y),
            (Value::Float(x),  Value::UFloat(y)) => Value::Float(x/ f64::from(y)),

            // Int Float
            (Value::Int(x),   Value::Float(y)) => Value::Float(x as f64 /y),
            (Value::Float(x), Value::Int(y))   => Value::Float(x/ y as f64),
            // Int UFloat
            (Value::Int(x),    Value::UFloat(y)) => Value::Float(x as f64 /f64::from(y)),
            (Value::UFloat(x), Value::Int(y))    => Value::Float(f64::from(x)/ y as f64),

            // UInt UFloat
            (Value::UInt(x),  Value::Float(y)) => Value::Float(x as f64 /y),
            (Value::Float(x), Value::UInt(y))  => Value::Float(x/ y as f64),
            // UInt UFloat
            (Value::UInt(x),   Value::UFloat(y)) => Value::UFloat(uf64::from(x) /y),
            (Value::UFloat(x), Value::UInt(y))   => Value::UFloat(x/ uf64::from(y)),

            //
            _ => panic!("Unsupported operation: division with mixed types"),
        }
    }
}
impl Mul for Value {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            // Int
            (Value::Int(x),  Value::Int(y))  => Value::Int (x*y),
            // UInt
            (Value::UInt(x), Value::UInt(y)) => Value::UInt(x*y),
            // Int UInt
            (Value::UInt(x), Value::Int(y))  => Value::Int(x as i64 *y),
            (Value::Int(x),  Value::UInt(y)) => Value::Int(x* y as i64),

            // Float
            (Value::Float(x),  Value::Float(y))  => Value::Float (x*y),
            // UFloat
            (Value::UFloat(x), Value::UFloat(y)) => Value::UFloat(x*y),
            // Float UFloat
            (Value::UFloat(x), Value::Float(y))  => Value::Float(f64::from(x) *y),
            (Value::Float(x),  Value::UFloat(y)) => Value::Float(x* f64::from(y)),

            // Int Float
            (Value::Int(x),   Value::Float(y)) => Value::Float(x as f64 *y),
            (Value::Float(x), Value::Int(y))   => Value::Float(x* y as f64),
            // Int UFloat
            (Value::Int(x),    Value::UFloat(y)) => Value::Float(x as f64 /f64::from(y)),
            (Value::UFloat(x), Value::Int(y))    => Value::Float(f64::from(x)* y as f64),

            // UInt UFloat
            (Value::UInt(x),  Value::Float(y)) => Value::Float(x as f64 *y),
            (Value::Float(x), Value::UInt(y))  => Value::Float(x* y as f64),
            // UInt UFloat
            (Value::UInt(x),   Value::UFloat(y)) => Value::UFloat(uf64::from(x) *y),
            (Value::UFloat(x), Value::UInt(y))   => Value::UFloat(x* uf64::from(y)),

            //
            _ => panic!("Unsupported operation: multiplication with mixed types"),
        }
    }
}