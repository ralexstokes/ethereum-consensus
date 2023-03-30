use crate::lib::*;
use crate::primitives::Bytes32;
#[cfg(feature = "serde")]
use crate::serde::{try_bytes_from_hex_str, HexError};
use crate::ssz::ByteVector;
use milagro_bls::{AggregatePublicKey, AmclError};
#[cfg(feature = "serde")]
use serde;
use sha2::{digest::FixedOutput, Digest, Sha256};
use ssz_rs::prelude::*;

pub fn hash<D: AsRef<[u8]>>(data: D) -> Bytes32 {
    let mut result = vec![0u8; 32];
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize_into(result.as_mut_slice().into());
    Bytes32::try_from(result.as_ref()).expect("correct input")
}

const BLS_PUBLIC_KEY_BYTES_LEN: usize = 48;
const BLS_SECRET_KEY_BYTES_LEN: usize = 32;
const BLS_SIGNATURE_BYTES_LEN: usize = 96;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "serde")]
    Hex,
    EmptyAggregate,
    EncodingError {
        provided: usize,
        expected: usize,
    },
    SimpleSerialize(SimpleSerializeError),
    Randomness,
    BLST,
    InvalidSignature,
    KeyValidationFailed,
}

#[cfg(feature = "serde")]
impl From<HexError> for Error {
    fn from(_: HexError) -> Self {
        Error::Hex
    }
}

impl From<rand::Error> for Error {
    fn from(_: rand::Error) -> Self {
        Error::Randomness
    }
}

impl From<SimpleSerializeError> for Error {
    fn from(err: SimpleSerializeError) -> Self {
        Error::SimpleSerialize(err)
    }
}

impl From<BLSTError> for Error {
    fn from(_: BLSTError) -> Self {
        Error::BLST
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            #[cfg(feature = "serde")]
            Error::Hex => write!(f, "error deserializing hex-encoded input",),
            Error::EmptyAggregate => {
                write!(f, "inputs required for aggregation but none were provided",)
            }
            Error::EncodingError { provided, expected } => write!(
                f,
                "invalid length of encoding: expected {} bytes but only provided {} bytes",
                expected, provided
            ),
            Error::SimpleSerialize(err) => {
                write!(f, "simple serialize error {}", err)
            }
            Error::Randomness => write!(f, "randomness failure",),
            Error::BLST => write!(f, "blst error",),
            Error::InvalidSignature => write!(f, "invalid signature",),
            Error::KeyValidationFailed => write!(f, "key validation failed",),
        }
    }
}

#[derive(Debug)]
pub struct BLSTError(String);

impl fmt::Display for BLSTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<milagro_bls::AmclError> for BLSTError {
    fn from(err: AmclError) -> Self {
        let inner = match err {
            AmclError::AggregateEmptyPoints => {
                unreachable!("do not create a BLSTError from a sucess")
            }
            AmclError::HashToFieldError => "hash to field  error",
            AmclError::InvalidG1Size => "invalid  g1 size",
            AmclError::InvalidG2Size => "invalid g2 size",
            AmclError::InvalidSecretKeyRange => "invalid secret key range",
            AmclError::InvalidSecretKeySize => "invalid secret key size",
            AmclError::InvalidYFlag => "invalid y flag",
            AmclError::InvalidPoint => "invalid point",
        };
        Self(inner.to_string())
    }
}

pub fn verify_signature(
    public_key: &PublicKey,
    msg: &[u8],
    signature: &Signature,
) -> Result<(), Error> {
    let public_key: milagro_bls::PublicKey = public_key.try_into()?;
    let signature: milagro_bls::Signature = signature.try_into()?;
    let res = signature.verify(msg, &public_key);
    if res {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }
}

pub fn aggregate(signatures: &[Signature]) -> Result<Signature, Error> {
    if signatures.is_empty() {
        return Err(Error::EmptyAggregate);
    }

    let signatures = signatures
        .iter()
        .map(milagro_bls::Signature::try_from)
        .collect::<Result<Vec<milagro_bls::Signature>, Error>>()?;
    let signatures: Vec<&milagro_bls::Signature> = signatures.iter().collect();

    let aggregate_signature = milagro_bls::AggregateSignature::aggregate(&signatures);

    Signature::try_from(aggregate_signature)
}

pub fn aggregate_verify(
    public_keys: &[PublicKey],
    msgs: &[&[u8]],
    signature: &Signature,
) -> Result<(), Error> {
    let public_keys = public_keys
        .iter()
        .map(milagro_bls::PublicKey::try_from)
        .collect::<Result<Vec<milagro_bls::PublicKey>, Error>>()?;
    let public_keys: Vec<&milagro_bls::PublicKey> = public_keys.iter().collect();
    let signature: milagro_bls::Signature = signature.try_into()?;
    let agg_sig = milagro_bls::AggregateSignature::from_signature(&signature);
    let res = agg_sig.aggregate_verify(msgs, &public_keys);
    if res {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }
}

pub fn fast_aggregate_verify(
    public_keys: &[&PublicKey],
    msg: &[u8],
    signature: &Signature,
) -> Result<(), Error> {
    let public_keys = public_keys
        .iter()
        .cloned()
        .map(milagro_bls::PublicKey::try_from)
        .collect::<Result<Vec<milagro_bls::PublicKey>, Error>>()?;
    let public_keys: Vec<&milagro_bls::PublicKey> = public_keys.iter().collect();
    let signature: milagro_bls::Signature = signature.try_into()?;
    let agg_sig = milagro_bls::AggregateSignature::from_signature(&signature);
    let res = agg_sig.fast_aggregate_verify(msg, &public_keys);
    if res {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }
}

// Return the aggregate public key for the public keys in `pks`
pub fn eth_aggregate_public_keys(public_keys: &[PublicKey]) -> Result<PublicKey, Error> {
    if public_keys.is_empty() {
        return Err(Error::EmptyAggregate);
    }
    let public_keys = public_keys
        .iter()
        .map(milagro_bls::PublicKey::try_from)
        .collect::<Result<Vec<milagro_bls::PublicKey>, Error>>()?;
    let public_keys: Vec<&milagro_bls::PublicKey> = public_keys.iter().collect();

    milagro_bls::AggregatePublicKey::aggregate(&public_keys)
        .map(|agg_pk| PublicKey::try_from(agg_pk).unwrap())
        .map_err(|e| BLSTError::from(e).into())
}

pub fn eth_fast_aggregate_verify(
    public_keys: &[&PublicKey],
    message: &[u8],
    signature: &Signature,
) -> Result<(), Error> {
    if public_keys.is_empty() && signature.is_infinity() {
        Ok(())
    } else {
        fast_aggregate_verify(public_keys, message, signature)
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "spec-tests", derive(Debug))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
pub struct SecretKey(milagro_bls::SecretKey);

#[cfg(feature = "serde")]
impl TryFrom<String> for SecretKey {
    type Error = Error;

    fn try_from(data: String) -> Result<Self, Self::Error> {
        let encoding = try_bytes_from_hex_str(&data)?;
        Self::try_from(encoding.as_ref())
    }
}

impl TryFrom<&[u8]> for SecretKey {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let inner = milagro_bls::SecretKey::from_bytes(data).map_err(BLSTError::from)?;
        Ok(Self(inner))
    }
}

impl SecretKey {
    pub fn random<R: rand::Rng>(rng: &mut R) -> Result<Self, Error> {
        let mut ikm = [0u8; BLS_SECRET_KEY_BYTES_LEN];
        rng.try_fill_bytes(&mut ikm)?;
        Self::key_gen(&ikm)
    }

    pub fn key_gen(ikm: &[u8]) -> Result<Self, Error> {
        let sk = milagro_bls::SecretKey::key_generate(ikm, &[]).map_err(BLSTError::from)?;
        Ok(Self(sk))
    }

    pub fn public_key(&self) -> PublicKey {
        let pk = milagro_bls::PublicKey::from_secret_key(&self.0);
        PublicKey::try_from(pk.as_bytes().as_ref()).unwrap()
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        let inner = milagro_bls::Signature::new(msg, &self.0);
        Signature::try_from(inner.as_bytes().as_ref()).unwrap()
    }
}

#[derive(Clone, Default, Hash, PartialEq, Eq, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PublicKey(ByteVector<BLS_PUBLIC_KEY_BYTES_LEN>);

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = &self.0;
        write!(f, "PublicKey({inner})")
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = &self.0;
        write!(f, "{inner}")
    }
}

impl Deref for PublicKey {
    type Target = ByteVector<BLS_PUBLIC_KEY_BYTES_LEN>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PublicKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Error> {
        let inner =
            ByteVector::try_from(data).map_err(|err| -> SimpleSerializeError { err.into() })?;
        Ok(Self(inner))
    }
}

impl TryFrom<&PublicKey> for milagro_bls::PublicKey {
    type Error = Error;

    fn try_from(public_key: &PublicKey) -> Result<Self, Error> {
        let milagro_bls_public_key = milagro_bls::PublicKey::from_bytes(public_key.0.as_ref());
        let key_validate = Self::key_validate(&milagro_bls_public_key.clone().unwrap());

        if key_validate {
            Ok(milagro_bls_public_key.unwrap())
        } else {
            Err(Error::KeyValidationFailed)
        }
    }
}

impl TryFrom<&PublicKey> for milagro_bls::AggregatePublicKey {
    type Error = Error;

    fn try_from(public_key: &PublicKey) -> Result<Self, Error> {
        let milagro_bls_public_key = milagro_bls::PublicKey::from_bytes(public_key.0.as_ref());
        let key_validate =
            milagro_bls::PublicKey::key_validate(&milagro_bls_public_key.clone().unwrap());

        if !key_validate {
            return Err(Error::KeyValidationFailed);
        }

        let milagro_bls_aggregate_public_key =
            milagro_bls::AggregatePublicKey::from_public_key(&milagro_bls_public_key.unwrap());

        Ok(milagro_bls_aggregate_public_key)
    }
}

impl TryFrom<milagro_bls::AggregatePublicKey> for PublicKey {
    type Error = Error;

    fn try_from(aggregate_public_key: AggregatePublicKey) -> Result<Self, Self::Error> {
        let public_key = milagro_bls::PublicKey {
            point: aggregate_public_key.point,
        };

        PublicKey::try_from(public_key.as_bytes().as_slice())
    }
}

#[derive(Clone, Default, Hash, PartialEq, Eq, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature(ByteVector<BLS_SIGNATURE_BYTES_LEN>);

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = &self.0;
        write!(f, "Signature({inner})")
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = &self.0;
        write!(f, "{inner}")
    }
}

impl Deref for Signature {
    type Target = ByteVector<BLS_SIGNATURE_BYTES_LEN>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Signature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Error> {
        let inner =
            ByteVector::try_from(data).map_err(|err| -> SimpleSerializeError { err.into() })?;
        Ok(Self(inner))
    }
}

impl TryFrom<&Signature> for milagro_bls::Signature {
    type Error = Error;

    fn try_from(signature: &Signature) -> Result<Self, Error> {
        Self::from_bytes(signature.0.as_ref()).map_err(|err| BLSTError::from(err).into())
    }
}

impl TryFrom<milagro_bls::AggregateSignature> for Signature {
    type Error = Error;

    fn try_from(aggregate_signature: milagro_bls::AggregateSignature) -> Result<Self, Self::Error> {
        let signature = milagro_bls::Signature {
            point: aggregate_signature.point,
        };

        Signature::try_from(signature.as_bytes().as_slice())
    }
}

const INFINITY_COMPRESSED_SIGNATURE: [u8; BLS_SIGNATURE_BYTES_LEN] = [
    192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0,
];

impl Signature {
    pub fn is_infinity(&self) -> bool {
        self.as_ref() == INFINITY_COMPRESSED_SIGNATURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{aggregate, aggregate_verify, fast_aggregate_verify};
    use rand::prelude::*;

    const INFINITY_COMPRESSED_PUBLIC_KEY: [u8; BLS_PUBLIC_KEY_BYTES_LEN] = [
        0xc0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn signature() {
        let mut rng = thread_rng();
        let sk = SecretKey::random(&mut rng).unwrap();
        let pk = sk.public_key();
        let msg = "message".as_bytes();
        let sig = sk.sign(msg);

        assert!(verify_signature(&pk, msg, &sig).is_ok());
    }

    #[test]
    #[should_panic(expected = "expected")]
    fn test_signature_from_null_bytes() {
        let b = [0u8; 0];
        Signature::try_from(b.as_ref()).expect("can make a signature");
    }

    #[test]
    fn test_signature_from_good_bytes() {
        let b = [
            0xab, 0xb0, 0x12, 0x4c, 0x75, 0x74, 0xf2, 0x81, 0xa2, 0x93, 0xf4, 0x18, 0x5c, 0xad,
            0x3c, 0xb2, 0x26, 0x81, 0xd5, 0x20, 0x91, 0x7c, 0xe4, 0x66, 0x65, 0x24, 0x3e, 0xac,
            0xb0, 0x51, 0x00, 0x0d, 0x8b, 0xac, 0xf7, 0x5e, 0x14, 0x51, 0x87, 0x0c, 0xa6, 0xb3,
            0xb9, 0xe6, 0xc9, 0xd4, 0x1a, 0x7b, 0x02, 0xea, 0xd2, 0x68, 0x5a, 0x84, 0x18, 0x8a,
            0x4f, 0xaf, 0xd3, 0x82, 0x5d, 0xaf, 0x6a, 0x98, 0x96, 0x25, 0xd7, 0x19, 0xcc, 0xd2,
            0xd8, 0x3a, 0x40, 0x10, 0x1f, 0x4a, 0x45, 0x3f, 0xca, 0x62, 0x87, 0x8c, 0x89, 0x0e,
            0xca, 0x62, 0x23, 0x63, 0xf9, 0xdd, 0xb8, 0xf3, 0x67, 0xa9, 0x1e, 0x84,
        ];
        Signature::try_from(b.as_ref()).expect("can make a signature");
    }

    #[test]
    #[should_panic]
    fn test_long_signature() {
        let b = [
            0xab, 0xb0, 0x12, 0x4c, 0x75, 0x74, 0xf2, 0x81, 0xa2, 0x93, 0xf4, 0x18, 0x5c, 0xad,
            0x3c, 0xb2, 0x26, 0x81, 0xd5, 0x20, 0x91, 0x7c, 0xe4, 0x66, 0x65, 0x24, 0x3e, 0xac,
            0xb0, 0x51, 0x00, 0x0d, 0x8b, 0xac, 0xf7, 0x5e, 0x14, 0x51, 0x87, 0x0c, 0xa6, 0xb3,
            0xb9, 0xe6, 0xc9, 0xd4, 0x1a, 0x7b, 0x02, 0xea, 0xd2, 0x68, 0x5a, 0x84, 0x18, 0x8a,
            0x4f, 0xaf, 0xd3, 0x82, 0x5d, 0xaf, 0x6a, 0x98, 0x96, 0x25, 0xd7, 0x19, 0xcc, 0xd2,
            0xd8, 0x3a, 0x40, 0x10, 0x1f, 0x4a, 0x45, 0x3f, 0xca, 0x62, 0x87, 0x8c, 0x89, 0x0e,
            0xca, 0x62, 0x23, 0x63, 0xf9, 0xdd, 0xb8, 0xf3, 0x67, 0xa9, 0x1e, 0x84, 0xfc,
        ];
        Signature::try_from(b.as_ref()).expect("can make a signature");
    }

    #[test]
    #[should_panic(expected = "null secret key")]
    fn null_secret_key() {
        let ikm = [0u8; 0];
        SecretKey::try_from(ikm.as_ref()).expect("can make a null secret key");
    }

    #[test]
    #[should_panic(expected = "short secret key")]
    fn short_secret_key() {
        let bytes = [1u8; BLS_SECRET_KEY_BYTES_LEN - 1];
        SecretKey::try_from(bytes.as_ref()).expect("can make a short secret key");
    }

    #[test]
    #[should_panic(expected = "long secret key")]
    fn long_secret_key() {
        let bytes = [1u8; BLS_SECRET_KEY_BYTES_LEN + 1];
        SecretKey::try_from(bytes.as_ref()).expect("can make a long secret key");
    }

    #[test]
    #[should_panic(expected = "bad secret key")]
    fn bad_secret_key() {
        let bytes: [u8; BLS_SECRET_KEY_BYTES_LEN] = [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ];
        SecretKey::try_from(bytes.as_ref()).expect("can make a bad secret key");
    }

    #[test]
    fn random_secret_key() {
        let mut rng = thread_rng();
        let _ = SecretKey::random(&mut rng).unwrap();
    }

    #[test]
    fn random_public_key() {
        let mut rng = thread_rng();
        let _ = SecretKey::random(&mut rng).unwrap().public_key();
    }

    #[test]
    fn good_public_key() {
        let bytes: [u8; 48] = [
            0xa9, 0x9a, 0x76, 0xed, 0x77, 0x96, 0xf7, 0xbe, 0x22, 0xd5, 0xb7, 0xe8, 0x5d, 0xee,
            0xb7, 0xc5, 0x67, 0x7e, 0x88, 0xe5, 0x11, 0xe0, 0xb3, 0x37, 0x61, 0x8f, 0x8c, 0x4e,
            0xb6, 0x13, 0x49, 0xb4, 0xbf, 0x2d, 0x15, 0x3f, 0x64, 0x9f, 0x7b, 0x53, 0x35, 0x9f,
            0xe8, 0xb9, 0x4a, 0x38, 0xe4, 0x4c,
        ];
        let _ = PublicKey::try_from(bytes.as_ref()).expect("can't make a good public key");
    }

    #[test]
    fn zero_public_key() {
        let z = [0u8; BLS_PUBLIC_KEY_BYTES_LEN];
        PublicKey::try_from(z.as_ref()).expect("can make a zero public key");
    }

    #[test]
    #[should_panic(expected = "short public key")]
    fn short_public_key() {
        let z = [0u8; BLS_PUBLIC_KEY_BYTES_LEN - 1];
        PublicKey::try_from(z.as_ref()).expect("can make a short public key");
    }

    #[test]
    fn infinity_as_public_key() {
        PublicKey::try_from(INFINITY_COMPRESSED_PUBLIC_KEY.as_ref())
            .expect("can make infinity public key");
    }

    #[test]
    #[should_panic]
    fn long_public_key() {
        let z = [0u8; BLS_PUBLIC_KEY_BYTES_LEN + 1];
        PublicKey::try_from(z.as_ref()).expect("can make a long public key");
    }

    #[test]
    fn test_aggregate_verify() {
        let n = 20;
        let mut rng = thread_rng();
        let sks: Vec<_> = (0..n)
            .map(|_| SecretKey::random(&mut rng).unwrap())
            .collect();
        let pks: Vec<_> = sks.iter().map(|sk| sk.public_key()).collect();
        let msgs: Vec<Vec<u8>> = (0..n)
            .map(|_| (0..64).map(|_| rand::thread_rng().gen()).collect())
            .collect();

        let signatures: Vec<_> = msgs
            .iter()
            .zip(&sks)
            .map(|(msg, sk)| sk.sign(msg))
            .collect();

        let msgs = msgs.iter().map(|r| &r[..]).collect::<Vec<_>>();

        let sig = aggregate(signatures.as_ref()).unwrap();
        let v = aggregate_verify(pks.as_slice(), msgs.as_ref(), &sig);

        assert!(v.is_ok());
    }

    #[test]
    fn test_fast_aggregated_verify() {
        let n = 20;
        let mut rng = thread_rng();
        let sks: Vec<_> = (0..n)
            .map(|_| SecretKey::random(&mut rng).unwrap())
            .collect();
        let pks: Vec<_> = sks.iter().map(|sk| sk.public_key()).collect();
        let msg = "message".as_bytes();

        let signatures: Vec<_> = sks.iter().map(|sk| sk.sign(msg)).collect();

        let pks = pks.iter().collect::<Vec<_>>();
        let sig = aggregate(signatures.as_slice()).unwrap();
        let v = fast_aggregate_verify(&pks, msg, &sig);

        assert!(v.is_ok());
    }

    #[test]
    fn test_can_make_default_signature() {
        let _ = Signature::default();
    }

    #[test]
    fn test_can_sign() {
        let secret_key_hex = "40094c5c6c378857eac09b8ec64c87182f58700c056a8b371ad0eb0a5b983d50";
        let secret_key_bytes = hex::decode(secret_key_hex).expect("is hex");
        let encoding: &[u8] = secret_key_bytes.as_ref();
        let secret_key = SecretKey::try_from(encoding).unwrap();

        let signature_hex = "a01e49276730e4752eef31b0570c8707de501398dac70dd144438cd1bd05fb9b9bb3e1a9ceef0a68cc08904362cafa3f1005e5b699a41847fff6f5552260468846de5bdbf94a9aedeb29bc6cdb2c1d34922d9e9af4c0593a69ae978a90b5aba6";
        let signature_bytes = hex::decode(signature_hex).expect("can decode hex");
        let expected_signature = Signature::try_from(signature_bytes.as_ref()).unwrap();

        let message = b"blst is such a blast";
        let signature = secret_key.sign(message);
        assert_eq!(expected_signature, signature);
    }

    #[test]
    fn test_roundtrip_bytes() {
        let mut rng = thread_rng();
        let secret_key = SecretKey::random(&mut rng).unwrap();
        let public_key = secret_key.public_key();
        let msg = b"blst is such a blast";
        let signature = secret_key.sign(msg);

        let publicy_key_bytes = public_key.0.as_ref();
        let signature_bytes = signature.0.as_ref();

        let recovered_public_key = PublicKey::try_from(publicy_key_bytes).unwrap();
        assert_eq!(public_key, recovered_public_key);
        let recovered_signature = Signature::try_from(signature_bytes).unwrap();
        assert_eq!(signature, recovered_signature);

        assert!(verify_signature(&public_key, msg, &signature).is_ok());
    }

    #[test]
    fn test_serde() {
        let mut rng = thread_rng();
        let sk = SecretKey::random(&mut rng).unwrap();
        let pk = sk.public_key();
        let msg = "message".as_bytes();
        let sig = sk.sign(msg);

        let serialized_key = serde_json::to_string(&pk).unwrap();
        log::info!("{serialized_key}");
        let recovered_key: PublicKey = serde_json::from_str(&serialized_key).unwrap();
        assert_eq!(pk, recovered_key);
        let serialized_signature = serde_json::to_string(&sig).unwrap();
        log::info!("{serialized_signature}");
        let recovered_signature: Signature = serde_json::from_str(&serialized_signature).unwrap();
        assert_eq!(sig, recovered_signature);
    }
}
