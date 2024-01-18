use num_bigint::BigUint;
use super::{FieldElement, error::FieldError};

const SECP256K1_PRIME: [u32; 8] = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFE, 0xFFFFFC2F];

pub struct SECP256K1(FieldElement);

impl SECP256K1 {
    pub fn new(number: impl Into<BigUint>) -> Result<Self, FieldError> {
        let field_element = FieldElement::new(number.into(), BigUint::from_slice(&SECP256K1_PRIME))?;

        Ok(SECP256K1(field_element))
    }
}

#[cfg(test)]
mod test {
    use super::SECP256K1;

    #[test]
    pub fn test_new() {
        SECP256K1::new(7u32).unwrap();
    }
}