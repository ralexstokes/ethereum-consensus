use clap::Args;
use ethereum_consensus::crypto::SecretKey;
use rand::prelude::*;

#[derive(Debug, Args)]
#[clap(about = "generate a random BLS12-381 keypair")]
pub struct Command;

impl Command {
    pub fn execute(self) -> eyre::Result<()> {
        let mut rng = thread_rng();
        let secret_key = SecretKey::random(&mut rng).unwrap();
        let public_key = secret_key.public_key();
        println!("secret key: {secret_key:?}");
        println!("public key: {public_key:?}");
        Ok(())
    }
}
