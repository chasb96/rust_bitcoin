use num_bigint::BigUint;
use crate::{field_element::BitcoinFieldElement, BITCOIN_SECP256K1_CONFIG};
use super::Curve;

pub struct BitcoinCurve(Curve);

impl BitcoinCurve {
    pub fn new() -> Self {
        let a = BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.a);
        let b = BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.b);

        let curve = Curve::new(
            BitcoinFieldElement::new(a).unwrap().into(), 
            BitcoinFieldElement::new(b).unwrap().into()
        );

        Self(curve)
    }
}

impl Into<Curve> for BitcoinCurve {
    fn into(self) -> Curve {
        self.0
    }
}