use std::fmt::Display;

mod error;
mod point;

#[derive(Clone, Debug)]
pub struct Curve {
    a: f32,
    b: f32,
}

impl Curve {
    pub fn new(a: f32, b: f32) -> Self {
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
        self.a == other.a && self.b == other.b
    }
}