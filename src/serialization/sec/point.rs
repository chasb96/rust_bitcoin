use num_bigint::BigUint;
use crate::cryptography::{elliptic_curve::bitcoin_point::BitcoinPoint, field_element::bitcoin_field_element::BitcoinFieldElement, BITCOIN_SECP256K1_CONFIG};
use super::{deserialize::{DeserializeSEC, DeserializeSECError}, serialize::{SerializeSEC, SerializeSECError}};

impl SerializeSEC for BitcoinPoint {
    fn serialize_sec(&self, compressed: bool) -> Result<String, SerializeSECError> {
        let x = self.x().as_ref().ok_or(SerializeSECError)?.number();
        let y = self.y().as_ref().ok_or(SerializeSECError)?.number();

        let y_odd = y % BigUint::from(2u32) == BigUint::from(0u32);

        let serialized = match (compressed, y_odd) {
            (true, true) => format!("\x02{0}", hex::encode(x.to_bytes_be())),
            (true, false) => format!("\x03{0}", hex::encode(x.to_bytes_be())),
            (false, _) => format!("\x04{0}{1}", hex::encode(x.to_bytes_be()), hex::encode(y.to_bytes_be())),
        };

        Ok(serialized)
    }
}

impl DeserializeSEC for BitcoinPoint {
    fn deserialize_sec(s: String) -> Result<Self, DeserializeSECError> {
        let s_bytes = s.as_bytes();

        if s_bytes.len() == 0 {
            return Err(DeserializeSECError::InvalidFormat);
        }

        match s_bytes[0] {
            4 => {
                let x = BitcoinFieldElement::new(BigUint::from_bytes_be(&s_bytes[1..33]))?;
                let y = BitcoinFieldElement::new(BigUint::from_bytes_be(&s_bytes[33..65]))?;

                Ok(BitcoinPoint::new(x.into(), y.into()))
            },
            2 => {
                let x = BitcoinFieldElement::new(BigUint::from_bytes_be(&s_bytes[1..]))?;
                let b = BitcoinFieldElement::new(BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.b)).unwrap();

                let alpha = (&x.pow(BigUint::from(3u32)) + &b)?;
                let beta = alpha.sqrt();

                if beta.number() % BigUint::from(2u32) == BigUint::from(0u32) {
                    Ok(BitcoinPoint::new(x, beta))
                } else {
                    let prime = BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.p);

                    let beta_inv = BitcoinFieldElement::new(beta.number() - prime)?;

                    Ok(BitcoinPoint::new(x, beta_inv))
                }
            },
            3 => {
                let x = BitcoinFieldElement::new(BigUint::from_bytes_be(&s_bytes[1..]))?;
                let b = BitcoinFieldElement::new(BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.b)).unwrap();

                let alpha = (&x.pow(BigUint::from(3u32)) + &b)?;
                let beta = alpha.sqrt();

                if beta.number() % BigUint::from(2u32) == BigUint::from(1u32) {
                    Ok(BitcoinPoint::new(x, beta))
                } else {
                    let prime = BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.p);

                    let beta_inv = BitcoinFieldElement::new(beta.number() - prime)?;

                    Ok(BitcoinPoint::new(x, beta_inv))
                }
            },
            _ => Err(DeserializeSECError::InvalidFormat),
        }
    }
}