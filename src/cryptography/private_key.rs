use num_bigint::BigUint;
use super::{elliptic_curve::{error::PointError, point::Point}, random::random_biguint, signature::Signature};

pub struct PrivateKey {
    secret: BigUint,
    g: Point,
    n: BigUint,
    point: Point,
}

impl PrivateKey {
    pub fn new(secret: impl Into<BigUint>, g: impl Into<Point>, n: impl Into<BigUint>) -> Result<Self, PointError>  {
        let secret = secret.into();
        let g = g.into();
        let point = g.clone() * secret.clone();

        Ok(Self {
            secret,
            g,
            n: n.into(),
            point: point?,
        })
    }

    pub fn sign(&self, z: BigUint) -> Result<Signature, PointError> {
        let two = BigUint::from_slice(&[0x00000002]);

        let g = &self.g;
        let n = &self.n;
        let k = random_biguint() % n.clone();

        let r = (g * k.clone())?;
        let r = r.x().as_ref().unwrap().number();
        let k_inv = k.modpow(&(n - &two), &n);

        let mut s = (z + r.clone() * &self.secret) * k_inv % n;

        if s > n / two {
            s = n - s;
        }

        Ok(Signature::new(r.clone(), s))
    }
}