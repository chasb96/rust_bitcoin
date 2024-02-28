use num_bigint::BigUint;
use super::{elliptic_curve::{bitcoin_point::BitcoinPoint, error::PointError}, private_key::PrivateKey, signature::Signature, BITCOIN_SECP256K1_CONFIG};

pub struct BitcoinPrivateKey(PrivateKey);

impl BitcoinPrivateKey {
    pub fn new(secret: impl Into<BigUint>) -> Result<Self, PointError>  {
        let n = BigUint::from_slice(&BITCOIN_SECP256K1_CONFIG.n);
        let g = BitcoinPoint::g();
        
        Ok(Self(PrivateKey::new(secret, g, n)?))
    }

    pub fn sign(&self, z: BigUint) -> Result<Signature,  PointError> {
        self.0.sign(z)
    }
}