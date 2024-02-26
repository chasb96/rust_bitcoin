use std::ops::{Add, Mul};
use num_bigint::BigUint;
use crate::{field_element::FieldElement, BITCOIN_SECP256K1_CONFIG};
use super::{error::PointError, point::Point, Curve};

pub struct S256Point(Point);

impl S256Point {
    pub fn new(x: FieldElement, y: FieldElement) -> Result<Self, PointError> {
        let curve = Curve::new(
            FieldElement::new(
                BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.a), 
                BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.p)
            )?, 
            FieldElement::new(
                BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.b), 
                BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.p)
            )?
        );
        
        Ok(Self(
            Point::new(
                x, 
                y, 
                curve
            )?
        ))
    }

    pub fn identity(curve: Curve) -> Self {
        S256Point(Point::identity(curve))
    }

    pub fn infinity(curve: Curve) -> Self {
        S256Point(Point::infinity(curve))
    }

    pub fn is_identity(&self) -> bool {
        self.0.is_identity()
    }
}

impl Add for S256Point {
    type Output = Result<S256Point, PointError>;

    fn add(self, rhs: Self) -> Self::Output {
        Ok(S256Point((self.0 + rhs.0)?))
    }
}

impl Mul<u32> for S256Point {
    type Output = Result<S256Point, PointError>;

    fn mul(self, rhs: u32) -> Self::Output {
        Ok(S256Point((self.0 * rhs)?))
    }
}

impl Mul<BigUint> for S256Point {
    type Output = Result<S256Point, PointError>;

    fn mul(self, mut rhs: BigUint) -> Self::Output {
        rhs = rhs % BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.n);

        Ok(S256Point((self.0 * rhs)?))
    }
}

impl PartialEq for S256Point {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}