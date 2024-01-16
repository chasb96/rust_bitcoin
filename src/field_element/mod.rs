use std::ops::{Add, Sub, Mul, Div};
use self::error::FieldError;

mod error;

pub struct FieldElement {
    number: u32,
    prime: u32,
}

impl FieldElement {
    pub fn new(number: u32, prime: u32) -> Result<Self, FieldError> {
        if number >= prime {
            return Err(FieldError::InvalidNumber(number, prime))
        }

        Ok(
            Self {
                number,
                prime,
            }
        )
    }

    pub fn pow(self, mut pow: i32) -> FieldElement {
        pow %= (self.prime - 1) as i32;

        FieldElement {
            number: self.number.pow(pow as u32) % self.prime,
            prime: self.prime
        }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.prime == other.prime
    }
}

impl Add for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldError::MismatchPrimes(self.prime, rhs.prime));
        }

        Ok(
            FieldElement {
                number: (self.number + rhs.number) % self.prime,
                prime: self.prime,
            }
        )
    }
}

impl Sub for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldError::MismatchPrimes(self.prime, rhs.prime));
        }

        let res = match self.number.cmp(&rhs.number) {
            std::cmp::Ordering::Less => self.prime - ((rhs.number - self.number) % self.prime),
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => (self.number - rhs.number) % self.prime,
        };

        Ok(
            FieldElement {
                number: res,
                prime: self.prime,
            }
        )
    }
}

impl Mul for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldError::MismatchPrimes(self.prime, rhs.prime));
        }

        Ok(
            FieldElement {
                number: (self.number * rhs.number) % self.prime,
                prime: self.prime,
            }
        )
    }
}

impl Div for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldError::MismatchPrimes(self.prime, rhs.prime));
        }

        Ok(
            FieldElement {
                number: (self.number * rhs.number.pow(self.prime - 2)) % self.prime,
                prime: self.prime,
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::FieldElement;

    #[test]
    pub fn test_new() {
        assert!(FieldElement::new(2, 13).is_ok())
    }

    #[test]
    pub fn test_new_invalid() {
        assert!(FieldElement::new(13, 3).is_err())
    }

    #[test]
    pub fn test_pow() {
        let fe = FieldElement::new(5, 13).unwrap();

        let powd = fe.pow(2);

        assert_eq!(powd.number, 12)
    }

    #[test]
    pub fn test_add() {
        let f1 = FieldElement::new(12, 13).unwrap();
        let f2 = FieldElement::new(11, 13).unwrap();

        let f3 = (f1 + f2).unwrap();

        assert_eq!(f3.number, 10);
    }

    #[test]
    pub fn test_add_invalid() {
        let f1 = FieldElement::new(12, 13).unwrap();
        let f2 = FieldElement::new(11, 12).unwrap();

        assert!((f1 + f2).is_err());
    }

    #[test]
    pub fn test_sub() {
        let f1 = FieldElement::new(5, 13).unwrap();
        let f2 = FieldElement::new(8, 13).unwrap();

        let f3 = (f1 - f2).unwrap();

        assert_eq!(f3.number, 10);
    }

    #[test]
    pub fn test_sub_invalid() {
        let f1 = FieldElement::new(12, 13).unwrap();
        let f2 = FieldElement::new(11, 12).unwrap();

        assert!((f1 - f2).is_err());
    }

    #[test]
    pub fn test_mul() {
        let f1 = FieldElement::new(5, 13).unwrap();
        let f2 = FieldElement::new(10, 13).unwrap();

        let f3 = (f1 * f2).unwrap();

        assert_eq!(f3.number, 11);
    }

    #[test]
    pub fn test_mul_invalid() {
        let f1 = FieldElement::new(12, 13).unwrap();
        let f2 = FieldElement::new(11, 12).unwrap();

        assert!((f1 * f2).is_err());
    }

    #[test]
    pub fn test_div() {
        let f1 = FieldElement::new(10, 13).unwrap();
        let f2 = FieldElement::new(5, 13).unwrap();

        let f3 = (f1 / f2).unwrap();

        assert_eq!(f3.number, 2);
    }

    #[test]
    pub fn test_div_invalid() {
        let f1 = FieldElement::new(12, 13).unwrap();
        let f2 = FieldElement::new(11, 12).unwrap();

        assert!((f1 / f2).is_err());
    }
}