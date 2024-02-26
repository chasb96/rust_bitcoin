use std::{fmt::Display, ops::{Add, Div, Mul, Sub}};
use num_bigint::BigUint;
use crate::BITCOIN_SECP256K1_CONFIG;
use super::{error::FieldError, FieldElement};

pub struct S256Field(FieldElement);

impl S256Field {
    pub fn new(number: impl Into<BigUint>) -> Result<Self, FieldError> {
        Ok(Self(
            FieldElement::new(number, BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.p))?
        ))
    }

    pub fn pow(&self, pow: impl Into<BigUint>) -> FieldElement {
        self.0.pow(pow)
    }

    pub fn number(&self) -> &BigUint {
        &self.0.number
    }

    pub fn prime(&self) -> &BigUint {
        &self.0.prime
    }
}

impl PartialEq for S256Field {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Add for S256Field {
    type Output = Result<S256Field, FieldError>;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add<&S256Field> for S256Field {
    type Output = Result<S256Field, FieldError>;

    fn add(self, rhs: &S256Field) -> Self::Output {
        &self + rhs
    }
}

impl Add<S256Field> for &S256Field {
    type Output = Result<S256Field, FieldError>;

    fn add(self, rhs: S256Field) -> Self::Output {
        self + &rhs
    }
}

impl Add<Result<S256Field, FieldError>> for S256Field {
    type Output = Result<S256Field, FieldError>;

    fn add(self, rhs: Result<S256Field, FieldError>) -> Self::Output {
        &self + &rhs?
    }
}

impl Add<Result<S256Field, FieldError>> for &S256Field {
    type Output = Result<S256Field, FieldError>;

    fn add(self, rhs: Result<S256Field, FieldError>) -> Self::Output {
        self + &rhs?
    }
}

impl Add<S256Field> for Result<S256Field, FieldError> {
    type Output = Result<S256Field, FieldError>;

    fn add(self, rhs: S256Field) -> Self::Output {
        &self? + &rhs
    }
}

impl Add<&S256Field> for Result<S256Field, FieldError> {
    type Output = Result<S256Field, FieldError>;

    fn add(self, rhs: &S256Field) -> Self::Output {
        &self? + rhs
    }
}

impl Add<&S256Field> for &S256Field {
    type Output = Result<S256Field, FieldError>;

    fn add(self, rhs: &S256Field) -> Self::Output {
        Ok(S256Field((&self.0 + &rhs.0)?))
    }
}

impl Sub for S256Field {
    type Output = Result<S256Field, FieldError>;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Sub<&S256Field> for S256Field {
    type Output = Result<S256Field, FieldError>;

    fn sub(self, rhs: &S256Field) -> Self::Output {
        &self - rhs
    }
}

impl Sub<S256Field> for &S256Field {
    type Output = Result<S256Field, FieldError>;

    fn sub(self, rhs: S256Field) -> Self::Output {
        self - &rhs
    }
}

impl Sub<Result<S256Field, FieldError>> for S256Field {
    type Output = Result<S256Field, FieldError>;

    fn sub(self, rhs: Result<S256Field, FieldError>) -> Self::Output {
        &self - &rhs?
    }
}

impl Sub<Result<S256Field, FieldError>> for &S256Field {
    type Output = Result<S256Field, FieldError>;

    fn sub(self, rhs: Result<S256Field, FieldError>) -> Self::Output {
        self - &rhs?
    }
}

impl Sub<S256Field> for Result<S256Field, FieldError> {
    type Output = Result<S256Field, FieldError>;

    fn sub(self, rhs: S256Field) -> Self::Output {
        &self? - &rhs
    }
}

impl Sub<&S256Field> for Result<S256Field, FieldError> {
    type Output = Result<S256Field, FieldError>;

    fn sub(self, rhs: &S256Field) -> Self::Output {
        &self? - rhs
    }
}

impl Sub<&S256Field> for &S256Field {
    type Output = Result<S256Field, FieldError>;

    fn sub(self, rhs: &S256Field) -> Self::Output {
        Ok(S256Field((&self.0 - &rhs.0)?))
    }
}

impl Mul for S256Field {
    type Output = Result<S256Field, FieldError>;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&S256Field> for S256Field {
    type Output = Result<S256Field, FieldError>;

    fn mul(self, rhs: &S256Field) -> Self::Output {
        &self * rhs
    }
}

impl Mul<S256Field> for &S256Field {
    type Output = Result<S256Field, FieldError>;

    fn mul(self, rhs: S256Field) -> Self::Output {
        self * &rhs
    }
}

impl Mul<Result<S256Field, FieldError>> for S256Field {
    type Output = Result<S256Field, FieldError>;

    fn mul(self, rhs: Result<S256Field, FieldError>) -> Self::Output {
        &self * &rhs?
    }
}

impl Mul<Result<S256Field, FieldError>> for &S256Field {
    type Output = Result<S256Field, FieldError>;

    fn mul(self, rhs: Result<S256Field, FieldError>) -> Self::Output {        
        self * &rhs?
    }
}

impl Mul<&S256Field> for &S256Field {
    type Output = Result<S256Field, FieldError>;

    fn mul(self, rhs: &S256Field) -> Self::Output {
        Ok(S256Field((&self.0 * &rhs.0)?))
    }
}

impl Div<S256Field> for S256Field {
    type Output = Result<S256Field, FieldError>;

    fn div(self, rhs: S256Field) -> Self::Output {
        &self / &rhs
    }
}

impl Div<&S256Field> for S256Field {
    type Output = Result<S256Field, FieldError>;

    fn div(self, rhs: &S256Field) -> Self::Output {
        &self / rhs
    }
}

impl Div<S256Field> for &S256Field {
    type Output = Result<S256Field, FieldError>;

    fn div(self, rhs: S256Field) -> Self::Output {
        self / &rhs
    }
}

impl Div<Result<S256Field, FieldError>> for S256Field {
    type Output = Result<S256Field, FieldError>;

    fn div(self, rhs: Result<S256Field, FieldError>) -> Self::Output {
        &self / &rhs?
    }
}

impl Div<Result<S256Field, FieldError>> for &S256Field {
    type Output = Result<S256Field, FieldError>;

    fn div(self, rhs: Result<S256Field, FieldError>) -> Self::Output {
        self / &rhs?
    }
}

impl Div<S256Field> for Result<S256Field, FieldError> {
    type Output = Result<S256Field, FieldError>;

    fn div(self, rhs: S256Field) -> Self::Output {
        &self? / &rhs
    }
}

impl Div<&S256Field> for Result<S256Field, FieldError> {
    type Output = Result<S256Field, FieldError>;

    fn div(self, rhs: &S256Field) -> Self::Output {
        &self? / rhs
    }
}

impl Div<&S256Field> for &S256Field {
    type Output = Result<S256Field, FieldError>;

    fn div(self, rhs: &S256Field) -> Self::Output {
        Ok(S256Field((&self.0 / &rhs.0)?))
    }
}

impl Display for S256Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S256Field({})", self.0)
    }
}
