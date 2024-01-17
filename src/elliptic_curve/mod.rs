use std::fmt::Display;
use crate::field_element::FieldElement;

mod error;
mod point;

#[derive(Clone, Debug)]
pub struct Curve {
    pub a: FieldElement,
    pub b: FieldElement,
}

impl Curve {
    pub fn new(a: FieldElement, b: FieldElement) -> Self {
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

#[cfg(test)]
mod test {
    use crate::field_element::FieldElement;
    use super::{Curve, point::Point};


    #[test]
    pub fn element_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let curve = Curve::new(a, b);

        let x = FieldElement::new(192, prime).unwrap();
        let y = FieldElement::new(105, prime).unwrap();
        Point::new(x, y, curve).unwrap();
    }
}