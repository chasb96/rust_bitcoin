use std::ops::Add;
use super::{Curve, error::PointError};

pub struct Point {
    x: Option<f32>,
    y: Option<f32>,
    curve: Curve,
}

impl Point {
    pub fn new(x: f32, y: f32, curve: Curve) -> Result<Self, PointError> {
        if y.powi(2) != x.powi(3) + curve.a * x + curve.b {
            return Err(PointError::NotOnCurve(x, y, curve));
        }

        Ok(
            Self {
                x: Some(x),
                y: Some(y),
                curve,
            }
        )
    }

    pub fn identity(curve: Curve) -> Self {
        Self {
            x: None,
            y: None,
            curve,
        }
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

    Ok(
        Point {
            x: Some(x3),
            y: Some(y3),
            curve: p1.curve.clone()
        }
    )
}

fn add_eq(p: Point) -> Result<Point, PointError> {
    let (x1, y1, a) = (p.x.unwrap(), p.y.unwrap(), p.curve.a);

    if y1 == 0. {
        return Ok(Point::infinity(p.curve.clone()))
    }

    let slope = (3. * x1.powi(3) + a) / (2. * y1);

    let x3 = slope.powi(2) - (2. * x1);
    let y3 = (slope * (x1 - x3)) - y1;

    Ok(
        Point {
            x: Some(x3),
            y: Some(y3),
            curve: p.curve.clone(),
        }
    )
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.curve == other.curve
    }
}