use std::{fmt::Display, cmp::Ordering};

mod error;
mod point;

fn float_eq(l: f64, r: f64) -> bool {
    match l.partial_cmp(&r) {
        Some(Ordering::Less) => r - l <= f64::EPSILON,
        Some(Ordering::Equal) => true,
        Some(Ordering::Greater) => l - r <= f64::EPSILON,
        None => false,
    }
}

#[derive(Clone, Debug)]
pub struct Curve {
    a: f64,
    b: f64,
}

impl Curve {
    pub fn new(a: f64, b: f64) -> Self {
        Self {
            a,
            b,
        }
    }
}

impl Display for Curve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y^3 = x^3 + {}x + {}", self.a, self.b)
    }
}

impl PartialEq for Curve {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.a, other.a) && float_eq(self.b, other.b)
    }
}