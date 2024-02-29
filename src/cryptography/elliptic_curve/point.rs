use std::ops::{Add, Mul};
use num_bigint::BigUint;
use crate::cryptography::{field_element::FieldElement, signature::Signature, BITCOIN_SECP256K1_CONFIG};
use super::{bitcoin_point::BitcoinPoint, error::PointError, Curve};

#[derive(Clone, Debug)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    curve: Curve,
}

impl Point {
    pub fn new(x: FieldElement, y: FieldElement, curve: Curve) -> Result<Self, PointError> {
        Self::new_point(Some(x), Some(y), curve)
    }

    fn new_point(x: Option<FieldElement>, y: Option<FieldElement>, curve: Curve) -> Result<Self, PointError> {
        if let (Some(xp), Some(yp)) = (&x, &y) {
            let lhs = yp.pow(2u32);
            let rhs = &curve.b + &(&xp.pow(3u32) + &(&curve.a * xp)?)?;

            if lhs != rhs? {
                return Err(PointError::NotOnCurve(xp.clone(), yp.clone(), curve.clone()));
            }
        }

        Ok(Self {
            x,
            y,
            curve,
        })
    }

    pub fn identity(curve: Curve) -> Self {
        Self::new_point(None, None, curve).unwrap()
    }

    pub fn infinity(curve: Curve) -> Self {
        Self::identity(curve)
    }

    pub fn is_identity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    pub fn x(&self) -> &Option<FieldElement> {
        &self.x
    }

    pub fn y(&self) -> &Option<FieldElement> {
        &self.y
    }

    pub fn verify_signature<'a>(&self, z: impl Into<&'a BigUint>, signature: Signature) -> Result<bool, PointError> {
        let z = z.into();

        let n = BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.n);
        let g = BitcoinPoint::g();
        let two = BigUint::from(2u32);

        let s_inverse = signature.s().modpow(&(&n - two), &n);
        let u = z * &s_inverse % &n;
        let v = signature.r() * s_inverse % n;

        let ug: Point = (g * &u)?.into();
        let vp = (self * &v)?;
        let total: Point = (&ug + &vp)?; 

        match total.x {
            Some(x) => Ok(x.number() == signature.r()),
            None => Ok(false),
        }
    }
}

impl Add for &Point {
    type Output = Result<Point, PointError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.curve != rhs.curve {
            return Err(PointError::MismatchCurves(self.curve.clone(), rhs.curve.clone()));
        }

        if self.is_identity() || rhs.is_identity() {
            return Ok(Point::identity(self.curve.clone()))
        }

        if self == rhs {
            let (x1, y1, a) = (self.x.as_ref().unwrap(), self.y.as_ref().unwrap(), &self.curve.a);

            if y1.number() == &BigUint::from(0u32) {
                return Ok(Point::infinity(self.curve.clone()))
            }
        
            let three = FieldElement::new(3u32, x1.prime().to_owned())?;
            let two = FieldElement::new(2u32, x1.prime().to_owned())?;
        
            let slope = &(a + &(&three * &x1.pow(2u32))?)? / &(&two * y1)?;
            let slope = slope?;
        
            let x3 = &slope.pow(2u32) - &(&two * x1)?;
            let x3 = x3?;
        
            let y3 = &(&slope * &(x1 - &x3)?)? - y1;
        
            Point::new_point(Some(x3), Some(y3?), self.curve.clone())
        } else {
            let (x1, x2) = (self.x.as_ref().unwrap(), rhs.x.as_ref().unwrap());
            let (y1, y2) = (self.y.as_ref().unwrap(), rhs.y.as_ref().unwrap());

            let x_diff = x2 - x1;
            let y_diff = y2 - y1;

            let slope = (&y_diff? / &x_diff?)?;

            let x3 = &(&slope.pow(2u32) - x1)? - x2;
            let x3 = x3?;

            let y3 = &(&slope * &(x1 - &x3)?)? - y1;

            Point::new_point(Some(x3), Some(y3?), self.curve.clone())
        }
    }
}

impl<'a, T: Into<&'a BigUint>> Mul<T> for &Point {
    type Output = Result<Point, PointError>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut lhs = self.clone();
        let mut rhs = rhs.into().clone();

        let zero = BigUint::from_slice(&[0x00000000]);
        let one = BigUint::from_slice(&[0x00000001]);

        let mut result = Point::identity(lhs.curve.clone());

        while rhs > zero {
            if &rhs & &one == one {
                result = (&result + &lhs)?;
            }

            lhs = (&lhs + &lhs)?;

            rhs = rhs >> 1;
        }

        Ok(result)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.curve == other.curve
    }
}

#[cfg(test)]
mod test {
    use crate::cryptography::{elliptic_curve::Curve, field_element::FieldElement};
    use super::Point;

    #[test]
    pub fn test_new() {
        let prime: u32 = 13;

        let x = FieldElement::new(3u32, prime).unwrap();
        let y = FieldElement::new(6u32, prime).unwrap();

        let a = FieldElement::new(2u32, prime).unwrap();
        let b = FieldElement::new(3u32, prime).unwrap();

        assert!(Point::new(x, y, Curve::new(a, b)).is_ok());
    }

    #[test]
    pub fn test_new_invalid() {
        let prime: u32 = 13;

        let x = FieldElement::new(3u32, prime).unwrap();
        let y = FieldElement::new(8u32, prime).unwrap();

        let a = FieldElement::new(2u32, prime).unwrap();
        let b = FieldElement::new(3u32, prime).unwrap();

        assert!(Point::new(x, y, Curve::new(a, b)).is_err());
    }

    #[test]
    pub fn test_is_identity() {
        let prime: u32 = 13;

        let a = FieldElement::new(2u32, prime).unwrap();
        let b = FieldElement::new(3u32, prime).unwrap();

        assert!(Point::identity(Curve::new(a, b)).is_identity());
    }

    #[test]
    pub fn test_eq() {
        let prime: u32 = 13;

        let x = FieldElement::new(3u32, prime).unwrap();
        let y = FieldElement::new(6u32, prime).unwrap();

        let a = FieldElement::new(2u32, prime).unwrap();
        let b = FieldElement::new(3u32, prime).unwrap();
        let curve = Curve::new(a, b);

        let p1 = Point::new(x.clone(), y.clone(), curve.clone()).unwrap();
        let p2 = Point::new(x, y, curve).unwrap();

        assert_eq!(p1, p2)
    }

    #[test]
    pub fn test_add() {
        let prime: u32 = 13;

        let ax = FieldElement::new(3u32, prime).unwrap();
        let ay = FieldElement::new(6u32, prime).unwrap();
        let bx = FieldElement::new(6u32, prime).unwrap();
        let by = FieldElement::new(6u32, prime).unwrap();

        let a = FieldElement::new(2u32, prime).unwrap();
        let b = FieldElement::new(3u32, prime).unwrap();
        let curve = Curve::new(a, b);

        let p1 = Point::new(ax, ay, curve.clone()).unwrap();
        let p2 = Point::new(bx, by, curve).unwrap();

        (&p1 + &p2).unwrap();
    }

    #[test]
    pub fn test_add_result() {
        let prime: u32 = 223;

        let ax = FieldElement::new(192u32, prime).unwrap();
        let ay = FieldElement::new(105u32, prime).unwrap();
        let bx = FieldElement::new(17u32, prime).unwrap();
        let by = FieldElement::new(56u32, prime).unwrap();

        let a = FieldElement::new(0u32, prime).unwrap();
        let b = FieldElement::new(7u32, prime).unwrap();
        let curve = Curve::new(a, b);

        let p1 = Point::new(ax, ay, curve.clone()).unwrap();
        let p2 = Point::new(bx, by, curve.clone()).unwrap();

        let cx = FieldElement::new(170u32, prime).unwrap();
        let cy = FieldElement::new(142u32, prime).unwrap();

        let expected = Point::new(cx, cy, curve).unwrap();
        let actual = (&p1 + &p2).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn test_add_invalid() {
        let prime: u32 = 13;

        let ax = FieldElement::new(3u32, prime).unwrap();
        let ay = FieldElement::new(6u32, prime).unwrap();
        let bx = FieldElement::new(2u32, prime).unwrap();
        let by = FieldElement::new(4u32, prime).unwrap();

        let a1 = FieldElement::new(2u32, prime).unwrap();
        let b1 = FieldElement::new(3u32, prime).unwrap();
        let a2 = FieldElement::new(2u32, prime).unwrap();
        let b2 = FieldElement::new(4u32, prime).unwrap();
        let curvea = Curve::new(a1, b1);
        let curveb = Curve::new(a2, b2);

        let p1 = Point::new(ax, ay, curvea).unwrap();
        let p2 = Point::new(bx, by, curveb).unwrap();

        let p1p2 = &p1 + &p2;

        assert!(p1p2.is_err());
    }

    #[test]
    pub fn test_add_eq() {
        let prime: u32 = 13;

        let ax = FieldElement::new(3u32, prime).unwrap();
        let ay = FieldElement::new(6u32, prime).unwrap();
        let bx = FieldElement::new(3u32, prime).unwrap();
        let by = FieldElement::new(6u32, prime).unwrap();

        let a = FieldElement::new(2u32, prime).unwrap();
        let b = FieldElement::new(3u32, prime).unwrap();
        let curve = Curve::new(a, b);

        let p1 = Point::new(ax, ay, curve.clone()).unwrap();
        let p2 = Point::new(bx, by, curve).unwrap();

        (&p1 + &p2).unwrap();
    }

    #[test]
    pub fn test_add_identity() {
        let prime: u32 = 13;

        let ax = FieldElement::new(3u32, prime).unwrap();
        let ay = FieldElement::new(6u32, prime).unwrap();

        let a = FieldElement::new(2u32, prime).unwrap();
        let b = FieldElement::new(3u32, prime).unwrap();
        let curve = Curve::new(a.clone(), b.clone());

        let p = Point::new(ax, ay, curve).unwrap();
        let identity = Point::identity(Curve::new(a, b));

        assert!((&p + &identity).unwrap().is_identity());
        assert!((&identity + &p).unwrap().is_identity());
    }
}