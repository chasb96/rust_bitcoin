use std::ops::{Add, Mul};
use num_bigint::BigUint;
use crate::cryptography::{field_element::{bitcoin_field_element::BitcoinFieldElement, FieldElement}, BITCOIN_SECP256K1_CONFIG};
use super::{bitcoin_curve::BitcoinCurve, error::PointError, point::Point};

pub struct BitcoinPoint(Point);

impl BitcoinPoint {
    pub fn new(x: BitcoinFieldElement, y: BitcoinFieldElement) -> Self {
        let curve = BitcoinCurve::new().into();
        let point = Point::new(x.into(), y.into(), curve).unwrap();

        Self(point)
    }

    pub fn g() -> Self {
        let gx = BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.gx);
        let gy = BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.gy);

        let x = BitcoinFieldElement::new(gx).unwrap().into();
        let y = BitcoinFieldElement::new(gy).unwrap().into();

        Self::new(x, y)
    }

    pub fn identity() -> Self {
        BitcoinPoint(Point::identity(BitcoinCurve::new().into()))
    }

    pub fn infinity() -> Self {
        BitcoinPoint(Point::infinity(BitcoinCurve::new().into()))
    }

    pub fn is_identity(&self) -> bool {
        self.0.is_identity()
    }

    pub fn x(&self) -> &Option<FieldElement> {
        self.0.x()
    }

    pub fn y(&self) -> &Option<FieldElement> {
        self.0.y()
    }
}

impl Into<Point> for BitcoinPoint {
    fn into(self) -> Point {
        self.0
    }
}

impl Add for &BitcoinPoint {
    type Output = Result<BitcoinPoint, PointError>;

    fn add(self, rhs: Self) -> Self::Output {
        Ok(BitcoinPoint((&self.0 + &rhs.0)?))
    }
}

impl<'a, T: Into<&'a BigUint>> Mul<T> for BitcoinPoint {
    type Output = Result<BitcoinPoint, PointError>;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into() % BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.n);

        Ok(BitcoinPoint((&self.0 * &rhs)?))
    }
}

impl PartialEq for BitcoinPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}