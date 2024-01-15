use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum FieldError {
    InvalidNumber(u32, u32),
    MismatchPrimes(u32, u32),
}

impl Error for FieldError { }

impl Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldError::InvalidNumber(n, p) => write!(f, "FieldError::InvalidNumber({},{})", n, p),
            FieldError::MismatchPrimes(l, r) => write!(f, "FieldError::MismatchPrimes({},{})", l, r),
        }
    }
}