/*
    Unsigned Float 64
*/

use std::fmt;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct uf64(f64);

impl fmt::Display for uf64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// op
    // +
    impl std::ops::Add for uf64 {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            uf64(self.0 + other.0)
        }
    }
    // -
    impl std::ops::Sub for uf64 {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            uf64(self.0 - other.0)
        }
    }
    // *
    impl std::ops::Mul for uf64 {
        type Output = Self;
        fn mul(self, other: Self) -> Self {
            uf64(self.0 * other.0)
        }
    }
    // /
    impl std::ops::Div for uf64 {
        type Output = Self;
        fn div(self, other: Self) -> Self {
            if other.0 != 0.0 {
                uf64(self.0 / other.0)
            } else {
                uf64(self.0)
            }
        }
    }
// uf64 <- f64 u64 i64
    // f64 -> uf64
    impl From<f64> for uf64 {
        fn from(value: f64) -> Self {
            if value >= 0.0 {
                uf64(value)
            } else {
                uf64(0.0)
            }
        }
    }
    // u64 -> uf64
    impl From<u64> for uf64 {
        fn from(value: u64) -> Self {
            uf64(value as f64)
        }
    }
    // i64 -> uf64
    impl From<i64> for uf64 {
        fn from(value: i64) -> Self {
            if value >= 0 {
                uf64(value as f64)
            } else {
                uf64(0.0)
            }
        }
    }
// uf64 -> f64 u64 i64
    // uf64 -> f64
    impl From<uf64> for f64 {
        fn from(value: uf64) -> Self {
            value.0
        }
    }
    // uf64 -> u64
    impl From<uf64> for u64 {
        fn from(value: uf64) -> Self {
            value.0 as u64
        }
    }
    // uf64 -> i64
    impl From<uf64> for i64 {
        fn from(value: uf64) -> Self {
            value.0 as i64
        }
    }
// uf64
impl uf64 {
    pub fn new(value: f64) -> Self {
        if value >= 0.0 {
            uf64(value)
        } else {
            uf64(0.0)
        }
    }
}