use crate::{
    crypto::{self, SecretKey},
    primitives::{BlsPublicKey, BlsSignature, Domain, Root},
    ssz::prelude::*,
    Error,
};
use std::fmt::Debug;
use tracing::info;

#[derive(Default, Debug, SimpleSerialize)]
pub struct SigningData {
    pub object_root: Root,
    pub domain: Domain,
}

pub fn compute_signing_root<T: HashTreeRoot>(
    ssz_object: &T,
    domain: Domain,
) -> Result<Root, Error> {
    let object_root = ssz_object.hash_tree_root()?;

    let s = SigningData { object_root, domain };
    s.hash_tree_root().map_err(Error::Merkleization)
}

pub fn sign_with_domain<T: HashTreeRoot>(
    data: &T,
    signing_key: &SecretKey,
    domain: Domain,
) -> Result<BlsSignature, Error> {
    let signing_root = compute_signing_root(data, domain)?;
    Ok(signing_key.sign(signing_root.as_ref()))
}

pub fn verify_signed_data<T: HashTreeRoot + Debug>(
    data: &T,
    signature: &BlsSignature,
    public_key: &BlsPublicKey,
    domain: Domain,
) -> Result<(), Error> {
    let signing_root = compute_signing_root(data, domain)?;
    info!(
        "signing_root: {:?}, data: {:?}, pubkey: {:?}, domain: {:?}",
        signing_root, data, public_key, domain
    );
    crypto::verify_signature(public_key, signing_root.as_ref(), signature).map_err(Into::into)
}
