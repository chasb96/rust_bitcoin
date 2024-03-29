pub mod field_element;
pub mod elliptic_curve;
mod signature;
mod private_key;
mod random;
mod bitcoin_private_key;

pub const BITCOIN_SECP256K1_CONFIG: Secp256k1Config = Secp256k1Config {
    p: [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFE, 0xFFFFFC2F],
    a: [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
    b: [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000007],
    gx: [0x79BE667E, 0xF9DCBBAC, 0x55A06295, 0xCE870B07, 0x029BFCDB, 0x2DCE28D9, 0x59F2815B, 0x16F81798],
    gy: [0x483ADA77, 0x26A3C465, 0x5DA4FBFC, 0x0E1108A8, 0xFD17B448, 0xA6855419, 0x9C47D08F, 0xFB10D4B8],
    n: [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFE, 0xBAAEDCE6, 0xAF48A03B, 0xBFD25E8C, 0xD0364141],
};

pub struct Secp256k1Config {
    pub p: [u32; 8],
    pub a: [u32; 8],
    pub b: [u32; 8],
    pub gx: [u32; 8],
    pub gy: [u32; 8],
    pub n: [u32; 8],
}