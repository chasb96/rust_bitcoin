use std::{fmt::Display, ops::{Add, Div, Mul, Sub}};
use num_bigint::BigUint;
use crate::cryptography::BITCOIN_SECP256K1_CONFIG;
use super::{error::FieldError, FieldElement};

pub struct BitcoinFieldElement(FieldElement);

impl BitcoinFieldElement {
    pub fn new(number: impl Into<BigUint>) -> Result<Self, FieldError> {
        let prime = BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.p);
        let field_element = FieldElement::new(number, prime)?;

        Ok(Self(field_element))
    }

    pub fn pow(&self, pow: impl Into<BigUint>) -> BitcoinFieldElement {
        Self(self.0.pow(pow))
    }

    pub fn sqrt(&self) -> BitcoinFieldElement {
        Self(self.0.sqrt())
    }

    pub fn number(&self) -> &BigUint {
        &self.0.number
    }

    pub fn prime(&self) -> &BigUint {
        &self.0.prime
    }
}

impl Into<FieldElement> for BitcoinFieldElement {
    fn into(self) -> FieldElement {
        self.0
    }
}

impl PartialEq for BitcoinFieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Add for &BitcoinFieldElement {
    type Output = Result<BitcoinFieldElement, FieldError>;

    fn add(self, rhs: Self) -> Self::Output {
        Ok(BitcoinFieldElement((&self.0 + &rhs.0)?))
    }
}

impl Sub for &BitcoinFieldElement {
    type Output = Result<BitcoinFieldElement, FieldError>;

    fn sub(self, rhs: Self) -> Self::Output {
        Ok(BitcoinFieldElement((&self.0 - &rhs.0)?))
    }
}

impl Mul for &BitcoinFieldElement {
    type Output = Result<BitcoinFieldElement, FieldError>;

    fn mul(self, rhs: Self) -> Self::Output {
        Ok(BitcoinFieldElement((&self.0 * &rhs.0)?))
    }
}

impl Div for &BitcoinFieldElement {
    type Output = Result<BitcoinFieldElement, FieldError>;

    fn div(self, rhs: Self) -> Self::Output {
        Ok(BitcoinFieldElement((&self.0 / &rhs.0)?))
    }
}

impl Display for BitcoinFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BitcoinFieldElement({})", self.0)
    }
}
