use std::ops::{Add, Mul};
use num_bigint::BigUint;
use crate::cryptography::{field_element::{bitcoin_field_element::BitcoinFieldElement, FieldElement}, BITCOIN_SECP256K1_CONFIG};
use super::{bitcoin_curve::BitcoinCurve, error::PointError, point::Point, Curve};

pub struct BitcoinPoint(Point);

impl BitcoinPoint {
    pub fn new(x: FieldElement, y: FieldElement) -> Result<Self, PointError> {
        let curve = BitcoinCurve::new().into();
        let point = Point::new(x, y, curve)?;

        Ok(Self(point))
    }

    pub fn g() -> Self {
        let gx = BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.gx);
        let gy = BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.gy);

        let x = BitcoinFieldElement::new(gx).unwrap().into();
        let y = BitcoinFieldElement::new(gy).unwrap().into();

        Self::new(x, y).unwrap()
    }

    pub fn identity(curve: Curve) -> Self {
        BitcoinPoint(Point::identity(curve))
    }

    pub fn infinity(curve: Curve) -> Self {
        BitcoinPoint(Point::infinity(curve))
    }

    pub fn is_identity(&self) -> bool {
        self.0.is_identity()
    }
}

impl Into<Point> for BitcoinPoint {
    fn into(self) -> Point {
        self.0
    }
}

impl Add for BitcoinPoint {
    type Output = Result<BitcoinPoint, PointError>;

    fn add(self, rhs: Self) -> Self::Output {
        Ok(BitcoinPoint((self.0 + rhs.0)?))
    }
}

impl Mul<u32> for BitcoinPoint {
    type Output = Result<BitcoinPoint, PointError>;

    fn mul(self, rhs: u32) -> Self::Output {
        Ok(BitcoinPoint((self.0 * rhs)?))
    }
}

impl Mul<BigUint> for BitcoinPoint {
    type Output = Result<BitcoinPoint, PointError>;

    fn mul(self, mut rhs: BigUint) -> Self::Output {
        rhs = rhs % BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.n);

        Ok(BitcoinPoint((self.0 * rhs)?))
    }
}

impl PartialEq for BitcoinPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}