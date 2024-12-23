use crate::internals::crypto::ed25519::*;

pub struct GenerateKeypair;

impl GenerateKeypair {
    pub fn new() -> (ED25519SecretKey,ED25519PublicKey) {
        let sk = SumatraED25519::new();
        let pk = sk.to_public_key();

        return (sk,pk)
    }
}