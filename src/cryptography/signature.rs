use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct Signature {
    r: BigUint,
    s: BigUint,
}

impl Signature {
    pub fn new(r: BigUint, s: BigUint) -> Self {
        Self { r, s }
    }

    pub fn s(&self) -> &BigUint {
        &self.s
    }

    pub fn r(&self) -> &BigUint {
        &self.r
    }
}