use std::{ops::{Add, Sub, Mul, Div}, cmp::Ordering, fmt::Display};
use num_bigint::BigUint;
use self::error::FieldError;

pub mod error;
mod bitcoin_field_element;

pub use bitcoin_field_element::BitcoinFieldElement;

#[derive(Debug, Clone)]
pub struct FieldElement {
    number: BigUint,
    prime: BigUint,
}

impl FieldElement {
    pub fn new(number: impl Into<BigUint>, prime: impl Into<BigUint>) -> Result<Self, FieldError> {
        let number = number.into();
        let prime = prime.into();

        if &number >= &prime {
            return Err(FieldError::InvalidNumber(number, prime.clone()))
        }

        Ok(
            Self {
                number,
                prime: prime as BigUint,
            }
        )
    }

    pub fn pow(&self, pow: impl Into<BigUint>) -> FieldElement {
        FieldElement {
            number: self.number.modpow(&pow.into(), &self.prime),
            prime: self.prime.clone()
        }
    }

    pub fn number(&self) -> &BigUint {
        &self.number
    }

    pub fn prime(&self) -> &BigUint {
        &self.prime
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
        &self + &rhs
    }
}

impl Add<&FieldElement> for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn add(self, rhs: &FieldElement) -> Self::Output {
        &self + rhs
    }
}

impl Add<FieldElement> for &FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn add(self, rhs: FieldElement) -> Self::Output {
        self + &rhs
    }
}

impl Add<Result<FieldElement, FieldError>> for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn add(self, rhs: Result<FieldElement, FieldError>) -> Self::Output {
        &self + &rhs?
    }
}

impl Add<Result<FieldElement, FieldError>> for &FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn add(self, rhs: Result<FieldElement, FieldError>) -> Self::Output {
        self + &rhs?
    }
}

impl Add<FieldElement> for Result<FieldElement, FieldError> {
    type Output = Result<FieldElement, FieldError>;

    fn add(self, rhs: FieldElement) -> Self::Output {
        &self? + &rhs
    }
}

impl Add<&FieldElement> for Result<FieldElement, FieldError> {
    type Output = Result<FieldElement, FieldError>;

    fn add(self, rhs: &FieldElement) -> Self::Output {
        &self? + rhs
    }
}

impl Add<&FieldElement> for &FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn add(self, rhs: &FieldElement) -> Self::Output {
        if &self.prime != &rhs.prime {
            return Err(FieldError::MismatchPrimes(self.prime.clone(), rhs.prime.clone()));
        }

        Ok(
            FieldElement {
                number: (&self.number + &rhs.number) % &self.prime,
                prime: self.prime.clone(),
            }
        )
    }
}

impl Sub for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Sub<&FieldElement> for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn sub(self, rhs: &FieldElement) -> Self::Output {
        &self - rhs
    }
}

impl Sub<FieldElement> for &FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn sub(self, rhs: FieldElement) -> Self::Output {
        self - &rhs
    }
}

impl Sub<Result<FieldElement, FieldError>> for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn sub(self, rhs: Result<FieldElement, FieldError>) -> Self::Output {
        &self - &rhs?
    }
}

impl Sub<Result<FieldElement, FieldError>> for &FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn sub(self, rhs: Result<FieldElement, FieldError>) -> Self::Output {
        self - &rhs?
    }
}

impl Sub<FieldElement> for Result<FieldElement, FieldError> {
    type Output = Result<FieldElement, FieldError>;

    fn sub(self, rhs: FieldElement) -> Self::Output {
        &self? - &rhs
    }
}

impl Sub<&FieldElement> for Result<FieldElement, FieldError> {
    type Output = Result<FieldElement, FieldError>;

    fn sub(self, rhs: &FieldElement) -> Self::Output {
        &self? - rhs
    }
}

impl Sub<&FieldElement> for &FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn sub(self, rhs: &FieldElement) -> Self::Output {
        if &self.prime != &rhs.prime {
            return Err(FieldError::MismatchPrimes(self.prime.clone(), rhs.prime.clone()));
        }

        let res = match self.number.cmp(&rhs.number) {
            Ordering::Less => &self.prime - ((&rhs.number - &self.number) % &self.prime),
            Ordering::Equal => 0u32.into(),
            Ordering::Greater => (&self.number - &rhs.number) % &self.prime,
        };

        Ok(
            FieldElement {
                number: res,
                prime: self.prime.clone(),
            }
        )
    }
}

impl Mul for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&FieldElement> for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn mul(self, rhs: &FieldElement) -> Self::Output {
        &self * rhs
    }
}

impl Mul<FieldElement> for &FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn mul(self, rhs: FieldElement) -> Self::Output {
        self * &rhs
    }
}

impl Mul<Result<FieldElement, FieldError>> for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn mul(self, rhs: Result<FieldElement, FieldError>) -> Self::Output {
        &self * &rhs?
    }
}

impl Mul<Result<FieldElement, FieldError>> for &FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn mul(self, rhs: Result<FieldElement, FieldError>) -> Self::Output {        
        self * &rhs?
    }
}

impl Mul<&FieldElement> for &FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn mul(self, rhs: &FieldElement) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldError::MismatchPrimes(self.prime.clone(), rhs.prime.clone()));
        }

        Ok(
            FieldElement {
                number: (&self.number * &rhs.number) % &self.prime,
                prime: self.prime.clone(),
            }
        )
    }
}

impl Div<FieldElement> for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn div(self, rhs: FieldElement) -> Self::Output {
        &self / &rhs
    }
}

impl Div<&FieldElement> for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn div(self, rhs: &FieldElement) -> Self::Output {
        &self / rhs
    }
}

impl Div<FieldElement> for &FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn div(self, rhs: FieldElement) -> Self::Output {
        self / &rhs
    }
}

impl Div<Result<FieldElement, FieldError>> for FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn div(self, rhs: Result<FieldElement, FieldError>) -> Self::Output {
        &self / &rhs?
    }
}

impl Div<Result<FieldElement, FieldError>> for &FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn div(self, rhs: Result<FieldElement, FieldError>) -> Self::Output {
        self / &rhs?
    }
}

impl Div<FieldElement> for Result<FieldElement, FieldError> {
    type Output = Result<FieldElement, FieldError>;

    fn div(self, rhs: FieldElement) -> Self::Output {
        &self? / &rhs
    }
}

impl Div<&FieldElement> for Result<FieldElement, FieldError> {
    type Output = Result<FieldElement, FieldError>;

    fn div(self, rhs: &FieldElement) -> Self::Output {
        &self? / rhs
    }
}

impl Div<&FieldElement> for &FieldElement {
    type Output = Result<FieldElement, FieldError>;

    fn div(self, rhs: &FieldElement) -> Self::Output {
        if &self.prime != &rhs.prime {
            return Err(FieldError::MismatchPrimes(self.prime.clone(), rhs.prime.clone()));
        }

        let prime_m_2: BigUint = &self.prime - BigUint::from(2u32);
        let rhs = rhs.number.modpow(&prime_m_2, &self.prime);

        Ok(
            FieldElement {
                // Since `rhs` is no longer its exponentiated value, we must use
                //  a different route than 
                //      `(self.number * rhs.number.pow(self.prime - 2)) % self.prime`
                // 
                // Because identity `(a * b) % m == (a % m) * (b % m) % m`, and
                //  since `rhs` is already its remainder, we can calculate the
                //  value.
                number: ((&self.number % &self.prime) * rhs) % &self.prime,
                prime: self.prime.clone(),
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
    use num_bigint::BigUint;
    use super::FieldElement;

    #[test]
    pub fn test_new() {
        assert!(FieldElement::new(2u32, 13u32).is_ok())
    }

    #[test]
    pub fn test_new_invalid() {
        assert!(FieldElement::new(13u32, 3u32).is_err())
    }

    #[test]
    pub fn test_pow() {
        let fe = FieldElement::new(5u32, 13u32).unwrap();

        let powd = fe.pow(2u32);

        assert_eq!(powd.number, BigUint::from(12u32))
    }

    #[test]
    pub fn test_add() {
        let f1 = FieldElement::new(12u32, 13u32).unwrap();
        let f2 = FieldElement::new(11u32, 13u32).unwrap();

        let f3 = (f1 + f2).unwrap();

        assert_eq!(f3.number, BigUint::from(10u32));
    }

    #[test]
    pub fn test_add_invalid() {
        let f1 = FieldElement::new(12u32, 13u32).unwrap();
        let f2 = FieldElement::new(11u32, 12u32).unwrap();

        assert!((f1 + f2).is_err());
    }

    #[test]
    pub fn test_sub() {
        let f1 = FieldElement::new(5u32, 13u32).unwrap();
        let f2 = FieldElement::new(8u32, 13u32).unwrap();

        let f3 = (f1 - f2).unwrap();

        assert_eq!(f3.number, BigUint::from(10u32));
    }

    #[test]
    pub fn test_sub_invalid() {
        let f1 = FieldElement::new(12u32, 13u32).unwrap();
        let f2 = FieldElement::new(11u32, 12u32).unwrap();

        assert!((f1 - f2).is_err());
    }

    #[test]
    pub fn test_mul() {
        let f1 = FieldElement::new(5u32, 13u32).unwrap();
        let f2 = FieldElement::new(10u32, 13u32).unwrap();

        let f3 = (f1 * f2).unwrap();

        assert_eq!(f3.number, BigUint::from(11u32));
    }

    #[test]
    pub fn test_mul_invalid() {
        let f1 = FieldElement::new(12u32, 13u32).unwrap();
        let f2 = FieldElement::new(11u32, 12u32).unwrap();

        assert!((f1 * f2).is_err());
    }

    #[test]
    pub fn test_div() {
        let f1 = FieldElement::new(10u32, 13u32).unwrap();
        let f2 = FieldElement::new(5u32, 13u32).unwrap();

        let f3 = (f1 / f2).unwrap();

        assert_eq!(f3.number, BigUint::from(2u32));
    }

    #[test]
    pub fn test_div_invalid() {
        let f1 = FieldElement::new(12u32, 13u32).unwrap();
        let f2 = FieldElement::new(11u32, 12u32).unwrap();

        assert!((f1 / f2).is_err());
    }
}