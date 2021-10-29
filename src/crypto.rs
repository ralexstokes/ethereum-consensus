use ssz_rs::prelude::*;

const BLS_PUBLIC_KEY_BYTES: usize = 48;

pub type BLSPubkey = Vector<u8, BLS_PUBLIC_KEY_BYTES>;
