use std::ops::Add;
use super::{Curve, error::PointError, float_eq};

#[derive(Debug)]
pub struct Point {
    x: Option<f64>,
    y: Option<f64>,
    curve: Curve,
}

impl Point {
    pub fn new(x: f64, y: f64, curve: Curve) -> Result<Self, PointError> {
        Self::new_point(Some(x), Some(y), curve)
    }

    fn new_point(x: Option<f64>, y: Option<f64>, curve: Curve) -> Result<Self, PointError> {
        if let (Some(xp), Some(yp)) = (x, y) {
            let lhs = yp.powi(2);
            let rhs = xp.powi(3) + curve.a * xp + curve.b;

            if (rhs - lhs).abs() > f64::EPSILON.sqrt() {
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

    if x_diff == 0. {
        return Err(PointError::DivideByZero);
    }

    let slope = y_diff / x_diff;

    let x3 = slope.powi(2) - x1 - x2;
    let y3 = (slope * (x1 - x3)) - y1;

    Point::new_point(Some(x3), Some(y3), p1.curve.clone())
}

fn add_eq(p: Point) -> Result<Point, PointError> {
    let (x1, y1, a) = (p.x.unwrap(), p.y.unwrap(), p.curve.a);

    if float_eq(y1, 0.) {
        return Ok(Point::infinity(p.curve.clone()))
    }

    let slope = (3. * x1.powi(3) + a) / (2. * y1);

    let x3 = slope.powi(2) - (2. * x1);
    let y3 = (slope * (x1 - x3)) - y1;

    Point::new_point(Some(x3), Some(y3), p.curve.clone())
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        match (self.x, other.x, self.y, other.y) {
            (Some(x1), Some(x2), Some(y1), Some(y2)) => 
                float_eq(x2, x1) && 
                float_eq(y2, y1) && 
                self.curve == other.curve,
            (None, None, None, None) => self.curve == other.curve,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::elliptic_curve::Curve;
    use super::Point;

    #[test]
    pub fn test_new() {
        assert!(Point::new(1., (6. as f64).sqrt(), Curve::new(2., 3.)).is_ok());
    }

    #[test]
    pub fn test_new_invalid() {
        assert!(Point::new(1., 1., Curve::new(2., 3.)).is_err());
    }

    #[test]
    pub fn test_is_identity() {
        assert!(Point::identity(Curve::new(2., 3.)).is_identity());
    }

    #[test]
    pub fn test_eq() {
        let p1 = Point::new(1., (6. as f64).sqrt(), Curve::new(2., 3.)).unwrap();
        let p2 = Point::new(1., (6. as f64).sqrt(), Curve::new(2., 3.)).unwrap();

        assert_eq!(p1, p2)
    }

    #[test]
    pub fn test_add() {
        let p1 = Point::new(1., (6. as f64).sqrt(), Curve::new(2., 3.)).unwrap();
        let p2 = Point::new(2., (15. as f64).sqrt(), Curve::new(2., 3.)).unwrap();

        assert!((p1 + p2).is_ok())
    }

    #[test]
    pub fn test_add_invalid() {
        let p1 = Point::new(1., (6. as f64).sqrt(), Curve::new(2., 3.)).unwrap();
        let p2 = Point::new(2., (13. as f64).sqrt(), Curve::new(1., 3.)).unwrap();

        assert!((p1 + p2).is_err())
    }

    #[test]
    pub fn test_add_eq() {
        let p1 = Point::new(1., (6. as f64).sqrt(), Curve::new(2., 3.)).unwrap();
        let p2 = Point::new(1., (6. as f64).sqrt(), Curve::new(2., 3.)).unwrap();

        assert!((p1 + p2).is_ok())
    }

    #[test]
    pub fn test_add_identity() {
        let identity = Point::identity(Curve::new(2., 3.));

        let p1 = Point::new(1., (6. as f64).sqrt(), Curve::new(2., 3.)).unwrap();
        let p2 = Point::identity(Curve::new(2., 3.));

        assert!((p1 + p2).unwrap() == identity);

        let p1 = Point::identity(Curve::new(2., 3.));
        let p2 = Point::new(1., (6. as f64).sqrt(), Curve::new(2., 3.)).unwrap();

        assert!((p1 + p2).unwrap() == identity)
    }
}