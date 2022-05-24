use crate::crypto::SecretKey;
use crate::phase0::compute_signing_root;
use crate::primitives::{BlsPublicKey, BlsSignature, Domain, Root};
use crate::state_transition::Error;
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct SigningData {
    pub object_root: Root,
    pub domain: Domain,
}

pub fn sign_with_domain<T: SimpleSerialize>(
    data: &mut T,
    signing_key: &SecretKey,
    domain: Domain,
) -> Result<BlsSignature, Error> {
    let signing_root = compute_signing_root(data, domain)?;
    Ok(signing_key.sign(signing_root.as_ref()))
}

pub fn verify_signed_data<T: SimpleSerialize>(
    data: &mut T,
    signature: &BlsSignature,
    public_key: &BlsPublicKey,
    domain: Domain,
) -> Result<(), Error> {
    let signing_root = compute_signing_root(data, domain)?;
    if signature.verify(public_key, signing_root.as_bytes()) {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }
}
