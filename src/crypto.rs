use ssz_rs::prelude::*;

const BLS_PUBLIC_KEY_BYTES: usize = 48;
const BLS_SIGNATURE_BYTES: usize = 96;

pub type BLSPubkey = Vector<u8, BLS_PUBLIC_KEY_BYTES>;
pub type BLSSignature = Vector<u8, BLS_SIGNATURE_BYTES>;
