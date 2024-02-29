use std::{error::Error, fmt::Display, fmt::Debug};

#[derive(Debug)]
pub struct SerializeSECError;

impl Error for SerializeSECError { }

impl Display for SerializeSECError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SerializeSECError(Invalid Value)")
    }
}

pub trait SerializeSEC {
    fn serialize_sec(&self, compressed: bool) -> Result<String, SerializeSECError>;
}