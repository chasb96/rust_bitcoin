use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct Signature {
    r: BigUint,
    s: BigUint,
}

impl Signature {
    pub fn new(r: impl Into<BigUint>, s: impl Into<BigUint>) -> Self {
        Self {
            r: r.into(),
            s: s.into()
        }
    }

    pub fn s(&self) -> &BigUint {
        &self.s
    }

    pub fn r(&self) -> &BigUint {
        &self.r
    }
}