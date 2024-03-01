use std::{error::Error, fmt::Display};

use crate::cryptography::{elliptic_curve::error::PointError, field_element::error::FieldError};

#[derive(Debug)]
pub enum DeserializeSECError {
    InvalidFormat,
    InvalidValue,
}

impl Error for DeserializeSECError { }

impl Display for DeserializeSECError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializeSECError::InvalidFormat => write!(f, "DeserializeSECError(Invalid Format)"),
            DeserializeSECError::InvalidValue => write!(f, "DeserializeSECError(Invalid Value)"),
        }
    }
}

pub trait DeserializeSEC: Sized {
    fn deserialize_sec<'a>(s: impl Into<&'a [u8]>) -> Result<Self, DeserializeSECError>;
}

impl From<FieldError> for DeserializeSECError {
    fn from(_: FieldError) -> Self {
        DeserializeSECError::InvalidValue
    }
}

impl From<PointError> for DeserializeSECError {
    fn from(_: PointError) -> Self {
        DeserializeSECError::InvalidValue
    }
}