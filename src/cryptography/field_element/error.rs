use std::{error::Error, fmt::Display};
use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub enum FieldError {
    InvalidNumber(BigUint, BigUint),
    MismatchPrimes(BigUint, BigUint),
}

impl Error for FieldError { }

impl Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldError::InvalidNumber(n, p) => write!(f, "FieldError::InvalidNumber(0 <= n < p, n = {}, p = {})", n, p),
            FieldError::MismatchPrimes(l, r) => write!(f, "FieldError::MismatchPrimes({} != {})", l, r),
        }
    }
}