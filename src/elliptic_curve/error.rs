use std::{error::Error, fmt::Display};
use super::Curve;

#[derive(Debug)]
pub enum PointError {
    NotOnCurve(f64, f64, Curve),
    MismatchCurves(Curve, Curve),
    DivideByZero,
}

impl Error for PointError { }

impl Display for PointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PointError::NotOnCurve(x, y, c) => write!(f, "PointError::NotOnCurve(({}, {}) not on curve {})", x, y, c),
            PointError::MismatchCurves(l, r) => write!(f, "PointError::MismatchCurve({} != {})", l, r),
            PointError::DivideByZero => write!(f, "PointError::DivideByZero(Attempted to divide by zero)"),
        }
    }
}