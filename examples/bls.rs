use ethereum_consensus::crypto;
use ethereum_consensus::crypto::SecretKey;

fn main() {
    let sk = SecretKey::random();
    let pk = sk.public_key();
    println!("{:?}", pk);
}