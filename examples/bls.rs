use ethereum_consensus::crypto::SecretKey;

/// A simple example that adopts the original BLST's example:
/// https://github.com/supranational/blst/tree/master/bindings/rust
fn main() {
    let sk = SecretKey::random();
    let pk = sk.public_key();

    let msg = b"blst is such a blast";
    let sig = sk.sign(msg);

    let valid = sig.verify(pk, msg,);
    println!("Signature is valid: {:?}", valid);
}
