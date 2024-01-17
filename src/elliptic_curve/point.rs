use std::ops::Add;
use crate::field_element::FieldElement;
use super::{Curve, error::PointError};

#[derive(Debug)]
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
        if let (Some(xp), Some(yp)) = (x, y) {
            let lhs = yp.pow(2);
            let rhs = ((xp.pow(3) + (curve.a * xp)?)? + curve.b)?;

            if lhs != rhs {
                return Err(PointError::NotOnCurve(xp, yp, curve));
            }
        }

        Ok(
            Self {
                x,
                y,
                curve,
            }
        )
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
}

impl Add for Point {
    type Output = Result<Point, PointError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.curve != rhs.curve {
            return Err(PointError::MismatchCurves(self.curve, rhs.curve));
        }

        if self.is_identity() || rhs.is_identity() {
            return Ok(Self::identity(self.curve.clone()))
        }

        match self == rhs {
            true => add_eq(self),
            false => add_ne(self, rhs),
        }
    }
}

fn add_ne(p1: Point, p2: Point) -> Result<Point, PointError> {
    let (x1, x2) = (p1.x.unwrap(), p2.x.unwrap());
    let (y1, y2) = (p1.y.unwrap(), p2.y.unwrap());

    let x_diff = x2 - x1;
    let y_diff = y2 - y1;

    let slope = (y_diff? / x_diff?)?;

    let x3 = ((slope.pow(2) - x1)? - x2)?;
    let y3 = ((slope * (x1 - x3)?)? - y1)?;

    Point::new_point(Some(x3), Some(y3), p1.curve.clone())
}

fn add_eq(p: Point) -> Result<Point, PointError> {
    let (x1, y1, a) = (p.x.unwrap(), p.y.unwrap(), p.curve.a);

    if y1.number() == 0 {
        return Ok(Point::infinity(p.curve.clone()))
    }

    let three = FieldElement::new(3, x1.prime())?;
    let two = FieldElement::new(2, x1.prime())?;

    let slope = (((three * x1.pow(2))? + a)? / (two * y1)?)?;

    let x3 = (slope.pow(2) - (two * x1)?)?;
    let y3 = ((slope * (x1 - x3)?)? - y1)?;

    Point::new_point(Some(x3), Some(y3), p.curve.clone())
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        match (self.x, other.x, self.y, other.y) {
            (Some(x1), Some(x2), Some(y1), Some(y2)) => 
                x2 == x1 && y2 == y1 && self.curve == other.curve,
            (None, None, None, None) => self.curve == other.curve,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{elliptic_curve::Curve, field_element::FieldElement};
    use super::Point;

    #[test]
    pub fn test_new() {
        let prime = 13;

        let x = FieldElement::new(3, prime).unwrap();
        let y = FieldElement::new(6, prime).unwrap();

        let a = FieldElement::new(2, prime).unwrap();
        let b = FieldElement::new(3, prime).unwrap();

        assert!(Point::new(x, y, Curve::new(a, b)).is_ok());
    }

    #[test]
    pub fn test_new_invalid() {
        let prime = 13;

        let x = FieldElement::new(3, prime).unwrap();
        let y = FieldElement::new(8, prime).unwrap();

        let a = FieldElement::new(2, prime).unwrap();
        let b = FieldElement::new(3, prime).unwrap();

        assert!(Point::new(x, y, Curve::new(a, b)).is_err());
    }

    #[test]
    pub fn test_is_identity() {
        let prime = 13;

        let a = FieldElement::new(2, prime).unwrap();
        let b = FieldElement::new(3, prime).unwrap();

        assert!(Point::identity(Curve::new(a, b)).is_identity());
    }

    #[test]
    pub fn test_eq() {
        let prime = 13;

        let x = FieldElement::new(3, prime).unwrap();
        let y = FieldElement::new(6, prime).unwrap();

        let a = FieldElement::new(2, prime).unwrap();
        let b = FieldElement::new(3, prime).unwrap();
        let curve = Curve::new(a, b);

        let p1 = Point::new(x, y, curve).unwrap();
        let p2 = Point::new(x, y, curve).unwrap();

        assert_eq!(p1, p2)
    }

    #[test]
    pub fn test_add() {
        let prime = 13;

        let ax = FieldElement::new(3, prime).unwrap();
        let ay = FieldElement::new(6, prime).unwrap();
        let bx = FieldElement::new(6, prime).unwrap();
        let by = FieldElement::new(6, prime).unwrap();

        let a = FieldElement::new(2, prime).unwrap();
        let b = FieldElement::new(3, prime).unwrap();
        let curve = Curve::new(a, b);

        let p1 = Point::new(ax, ay, curve).unwrap();
        let p2 = Point::new(bx, by, curve).unwrap();

        (p1 + p2).unwrap();
    }

    #[test]
    pub fn test_add_invalid() {
        let prime = 13;

        let ax = FieldElement::new(3, prime).unwrap();
        let ay = FieldElement::new(6, prime).unwrap();
        let bx = FieldElement::new(2, prime).unwrap();
        let by = FieldElement::new(4, prime).unwrap();

        let a1 = FieldElement::new(2, prime).unwrap();
        let b1 = FieldElement::new(3, prime).unwrap();
        let a2 = FieldElement::new(2, prime).unwrap();
        let b2 = FieldElement::new(4, prime).unwrap();
        let curvea = Curve::new(a1, b1);
        let curveb = Curve::new(a2, b2);

        let p1 = Point::new(ax, ay, curvea).unwrap();
        let p2 = Point::new(bx, by, curveb).unwrap();

        let p1p2 = p1 + p2;

        assert!(p1p2.is_err());
    }

    #[test]
    pub fn test_add_eq() {
        let prime = 13;

        let ax = FieldElement::new(3, prime).unwrap();
        let ay = FieldElement::new(6, prime).unwrap();
        let bx = FieldElement::new(3, prime).unwrap();
        let by = FieldElement::new(6, prime).unwrap();

        let a = FieldElement::new(2, prime).unwrap();
        let b = FieldElement::new(3, prime).unwrap();
        let curve = Curve::new(a, b);

        let p1 = Point::new(ax, ay, curve).unwrap();
        let p2 = Point::new(bx, by, curve).unwrap();

        (p1 + p2).unwrap();
    }

    // #[test]
    // pub fn test_add_identity() {
    //     let identity = Point::identity(Curve::new(2., 3.));

    //     let p1 = Point::new(1., (6. as f64).sqrt(), Curve::new(2., 3.)).unwrap();
    //     let p2 = Point::identity(Curve::new(2., 3.));

    //     assert!((p1 + p2).unwrap() == identity);

    //     let p1 = Point::identity(Curve::new(2., 3.));
    //     let p2 = Point::new(1., (6. as f64).sqrt(), Curve::new(2., 3.)).unwrap();

    //     assert!((p1 + p2).unwrap() == identity)
    // }
}