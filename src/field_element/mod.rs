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

        Ok(
            FieldElement {
                number: (self.number - rhs.number) % self.prime,
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