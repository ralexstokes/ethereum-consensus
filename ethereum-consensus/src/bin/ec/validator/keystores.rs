use crate::validator::keys::{KeyPair, Path};
use aes::cipher::{KeyIvInit, StreamCipher};
use ethereum_consensus::crypto::{hash, PublicKey as BlsPublicKey, SecretKey as BlsSecretKey};
use rayon::prelude::*;
use scrypt::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHasher, SaltString,
    },
    Params as ScryptParams, Scrypt,
};
use serde::{Deserialize, Serialize, Serializer};
use unicode_normalization::UnicodeNormalization;
use uuid::Uuid;

fn as_hex<S, D: AsRef<[u8]>>(data: D, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let encoding = hex::encode(data);
    s.collect_str(&encoding)
}

fn empty_string<S, D: AsRef<[u8]>>(_data: D, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.collect_str(&"")
}

fn as_json_str<S, D: Serialize>(data: D, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let encoding = serde_json::to_string(&data).unwrap();
    s.collect_str(&encoding)
}

pub type Passphrase = String;
const PASSPHRASE_LEN: usize = 32;

const VERSION: usize = 4;

const KDF_FN: &str = "scrypt";
const SCRYPT_DKLEN: usize = 32;

const CIPHER_FN: &str = "aes-128-ctr";
type CtrCipher = ctr::Ctr64BE<aes::Aes128>;
const AES_SIZE: usize = 16;

const CHECKSUM_FN: &str = "sha256";

#[derive(Debug, Serialize, Deserialize)]
struct KdfParams {
    n: usize,
    r: u32,
    p: u32,
    dklen: usize,
    #[serde(serialize_with = "as_hex")]
    salt: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Kdf {
    function: String,
    params: KdfParams,
    #[serde(serialize_with = "empty_string")]
    message: Vec<u8>,
}

impl Kdf {
    fn new_with_salt_and_params(
        passphrase: &Passphrase,
        salt: SaltString,
        params: ScryptParams,
    ) -> Self {
        let mut passphrase = passphrase.nfkd().collect::<String>();
        passphrase.retain(|c| !c.is_control());
        let mut buf = vec![0u8; salt.len()];
        let salt_bytes = salt.decode_b64(&mut buf).unwrap().to_vec();

        let password_hash = Scrypt
            .hash_password_customized(passphrase.as_bytes(), None, None, params, &salt)
            .unwrap();

        let n = 2_usize.pow(params.log_n() as u32);
        let params =
            KdfParams { n, r: params.r(), p: params.p(), dklen: SCRYPT_DKLEN, salt: salt_bytes };
        let message = password_hash.hash.unwrap().as_bytes().to_vec();
        Self { function: KDF_FN.to_string(), params, message }
    }

    fn from(passphrase: &Passphrase) -> Self {
        let salt = SaltString::generate(OsRng);
        // NOTE: `SCRYPT_DKLEN` used in `new_with_salt_and_params` matches the `recommended` params
        let params = ScryptParams::recommended();

        Self::new_with_salt_and_params(passphrase, salt, params)
    }

    fn encryption_key(&self) -> &[u8] {
        &self.message[..16]
    }

    fn checksum_salt(&self) -> &[u8] {
        &self.message[16..]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CipherParams {
    #[serde(serialize_with = "as_hex")]
    iv: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cipher {
    function: String,
    params: CipherParams,
    #[serde(serialize_with = "as_hex")]
    message: Vec<u8>,
}

impl Cipher {
    fn new_with_iv(secret: BlsSecretKey, key: &[u8], iv: [u8; AES_SIZE]) -> Self {
        let mut cipher = CtrCipher::new_from_slices(key, &iv).unwrap();
        let mut message = secret.to_bytes().to_vec();
        cipher.apply_keystream(&mut message);

        let params = CipherParams { iv: iv.to_vec() };
        Self { function: CIPHER_FN.to_string(), params, message }
    }

    fn new(secret: BlsSecretKey, key: &[u8]) -> Self {
        let mut iv = [0u8; AES_SIZE];
        OsRng.fill_bytes(&mut iv);
        Self::new_with_iv(secret, key, iv)
    }

    fn cipher_text(&self) -> &[u8] {
        &self.message
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Empty {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Checksum {
    function: String,
    params: Empty,
    #[serde(serialize_with = "as_hex")]
    message: Vec<u8>,
}

impl Checksum {
    fn new(secret: &[u8], salt: &[u8]) -> Self {
        let mut pre_image = salt.to_vec();
        pre_image.extend_from_slice(secret);
        let message = hash(pre_image).as_ref().to_vec();
        Self { function: CHECKSUM_FN.to_string(), params: Empty {}, message }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crypto {
    kdf: Kdf,
    checksum: Checksum,
    cipher: Cipher,
}

impl Crypto {
    fn from(passphrase: &Passphrase, key: BlsSecretKey) -> Self {
        let kdf = Kdf::from(passphrase);
        let cipher = Cipher::new(key, kdf.encryption_key());
        let checksum = Checksum::new(cipher.cipher_text(), kdf.checksum_salt());
        Crypto { kdf, checksum, cipher }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keystore {
    crypto: Crypto,
    #[serde(rename = "pubkey")]
    public_key: BlsPublicKey,
    path: Path,
    uuid: Uuid,
    version: usize,
}

impl Keystore {
    fn new_with_generated_passphrase(key_pair: KeyPair) -> (Self, Passphrase) {
        let mut passphrase = [0u8; PASSPHRASE_LEN];
        OsRng.fill_bytes(&mut passphrase);
        let passphrase = hex::encode(passphrase);
        (Self::new(key_pair, &passphrase), passphrase)
    }

    fn new(KeyPair { private_key, public_key, path }: KeyPair, passphrase: &Passphrase) -> Self {
        Keystore {
            crypto: Crypto::from(passphrase, private_key),
            public_key,
            path,
            uuid: Uuid::new_v4(),
            version: VERSION,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeystoreWithPassphrase {
    // NOTE: this JSON name is lighthouse specific
    #[serde(rename = "voting_keystore")]
    #[serde(serialize_with = "as_json_str")]
    keystore: Keystore,
    // NOTE: this JSON name is lighthouse specific
    #[serde(rename = "voting_keystore_password")]
    passphrase: Passphrase,
}

pub fn generate(keys: Vec<KeyPair>) -> Vec<KeystoreWithPassphrase> {
    keys.into_par_iter()
        .map(|key_pair| {
            let (keystore, passphrase) = Keystore::new_with_generated_passphrase(key_pair);
            KeystoreWithPassphrase { keystore, passphrase }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::prelude::*;

    const SCRYPT_LOG_N: u8 = 18;
    const SCRYPT_R: u32 = 8;
    const SCRYPT_P: u32 = 1;

    // Test case from EIP-2335:
    #[test]
    fn test_keystore_pubkey() {
        let secret = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";
        let secret = hex::decode(secret).unwrap();
        let secret_key = BlsSecretKey::try_from(secret.as_ref()).unwrap();
        let public_key = "9612d7a727c9d0a22e185a1c768478dfe919cada9266988cb32359c11f2b7b27f4ae4040902382ae2910c15e2b420d07";
        let public_key = hex::decode(public_key).unwrap();
        let expected_public_key = BlsPublicKey::try_from(public_key.as_ref()).unwrap();
        assert_eq!(expected_public_key, secret_key.public_key());
    }

    // Test case from EIP-2335:
    #[test]
    #[ignore = "strong key parameters take a long time to run"]
    fn test_keystore_crypto() {
        let passphrase = "𝔱𝔢𝔰𝔱𝔭𝔞𝔰𝔰𝔴𝔬𝔯𝔡🔑".to_string();
        let secret = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";
        let secret_key = BlsSecretKey::try_from(hex::decode(secret).unwrap().as_ref()).unwrap();

        let salt = hex::decode("d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3")
            .unwrap();
        let salt = &BASE64_STANDARD_NO_PAD.encode(salt);
        let salt = SaltString::from_b64(salt).unwrap();
        let params = ScryptParams::new(SCRYPT_LOG_N, SCRYPT_R, SCRYPT_P, SCRYPT_DKLEN).unwrap();
        let kdf = Kdf::new_with_salt_and_params(&passphrase, salt, params);

        let iv = hex::decode("264daa3f303d7259501c93d997d84fe6").unwrap();
        let cipher = Cipher::new_with_iv(secret_key, kdf.encryption_key(), iv.try_into().unwrap());

        let checksum = Checksum::new(cipher.cipher_text(), kdf.checksum_salt());

        assert_eq!(kdf.params.n, 262144);
        assert_eq!(
            kdf.params.salt,
            hex::decode("d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3")
                .unwrap()
        );
        assert_eq!(
            checksum.message,
            hex::decode("d2217fe5f3e9a1e34581ef8a78f7c9928e436d36dacc5e846690a5581e8ea484")
                .unwrap()
        );
        assert_eq!(cipher.params.iv, hex::decode("264daa3f303d7259501c93d997d84fe6").unwrap());
        assert_eq!(
            cipher.message,
            hex::decode("06ae90d55fe0a6e9c5c3bc5b170827b2e5cce3929ed3f116c2811e6366dfe20f")
                .unwrap()
        );
    }
}
