use std::{ops::{Add, Sub, Mul, Div}, cmp::Ordering, fmt::Display};
use self::error::FieldError;

pub mod error;

#[derive(Debug, Copy, Clone)]
pub struct FieldElement {
    number: u32,
    prime: u32,
}

impl FieldElement {
    pub fn new(number: u32, prime: u32) -> Result<Self, FieldError> {
        if number >= prime as u32 {
            return Err(FieldError::InvalidNumber(number, prime))
        }

        Ok(
            Self {
                number,
                prime: prime as u32,
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

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn prime(&self) -> u32 {
        self.prime
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
            Ordering::Less => self.prime - ((rhs.number - self.number) % self.prime),
            Ordering::Equal => 0,
            Ordering::Greater => (self.number - rhs.number) % self.prime,
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

        // More memory efficient than: `rhs.number.pow(self.prime - 2) % self.prime`
        //  allowing the value to be computed without worrying about multiply
        //  overflow.
        //
        // Since `a ** b` can be written as `a * a .. * a`, we can take advantage of
        //  `(a * b) % m == (a % m) * (b % m) % m` by doing effectively 
        //  `(a % m) * (a % m) .. * (a % m) % m`
        let rhs = (0..(self.prime - 2)).fold(1, |exp, _| (exp * rhs.number) % self.prime);

        Ok(
            FieldElement {
                // Since `rhs` is no longer its exponentiated value, we must use
                //  a different route than 
                //      `(self.number * rhs.number.pow(self.prime - 2)) % self.prime`
                // 
                // Because identity `(a * b) % m == (a % m) * (b % m) % m`, and
                //  since `rhs` is already its remainder, we can calculate the
                //  value.
                number: ((self.number % self.prime) * rhs) % self.prime,
                prime: self.prime,
            }
        )
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement(number = {}, prime = {})", self.number, self.prime)
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