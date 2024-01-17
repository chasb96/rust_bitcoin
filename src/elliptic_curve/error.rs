use std::{error::Error, fmt::Display};
use crate::field_element::{FieldElement, error::FieldError};
use super::Curve;

#[derive(Debug)]
pub enum PointError {
    NotOnCurve(FieldElement, FieldElement, Curve),
    MismatchCurves(Curve, Curve),
    FieldError(FieldError),
}

impl Error for PointError { }

impl Display for PointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PointError::NotOnCurve(x, y, c) => write!(f, "PointError::NotOnCurve(({}, {}) not on curve {})", x, y, c),
            PointError::MismatchCurves(l, r) => write!(f, "PointError::MismatchCurve({} != {})", l, r),
            PointError::FieldError(e) => write!(f, "PointError::FieldError({})", e),
        }
    }
}

impl From<FieldError> for PointError {
    fn from(value: FieldError) -> Self {
        Self::FieldError(value)
    }
}