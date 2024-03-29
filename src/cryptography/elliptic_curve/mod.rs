use std::fmt::Display;
use super::field_element::FieldElement;

pub mod error;
pub mod point;
pub mod bitcoin_curve;
pub mod bitcoin_point;

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
    use crate::cryptography::field_element::FieldElement;
    use super::{Curve, point::Point};


    #[test]
    pub fn element_on_curve() {
        let prime: u32 = 223;
        
        let a = FieldElement::new(0u32, prime).unwrap();
        let b = FieldElement::new(7u32, prime).unwrap();
        let curve = Curve::new(a, b);

        let x = FieldElement::new(192u32, prime).unwrap();
        let y = FieldElement::new(105u32, prime).unwrap();
        Point::new(x, y, curve.clone()).unwrap();

        let x = FieldElement::new(17u32, prime).unwrap();
        let y = FieldElement::new(56u32, prime).unwrap();
        Point::new(x, y, curve.clone()).unwrap();

        let x = FieldElement::new(1u32, prime).unwrap();
        let y = FieldElement::new(193u32, prime).unwrap();
        Point::new(x, y, curve.clone()).unwrap();

        let x = FieldElement::new(200u32, prime).unwrap();
        let y = FieldElement::new(119u32, prime).unwrap();
        assert!(Point::new(x, y, curve.clone()).is_err());

        let x = FieldElement::new(42u32, prime).unwrap();
        let y = FieldElement::new(99u32, prime).unwrap();
        assert!(Point::new(x, y, curve).is_err());
    }
}