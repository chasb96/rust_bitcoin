use num_bigint::BigUint;

pub fn random_biguint() -> BigUint {
    let mut bytes = [0; 8];

    for i in 0..8 {
        bytes[i] = rand::random::<u32>();
    }

    BigUint::from_slice(&bytes)
}