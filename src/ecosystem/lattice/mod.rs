use crate::internals::crypto::blake2b;
use crate::internals::serde::{Serialize,Deserialize};
use crate::internals::crypto::blake3;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub account: String,
    pub previous: String,
    pub balance: u64,
    pub nonce: u64,
    pub signature: String,
}

impl Block {
    pub fn new(account: String, previous: String, balance: u64, signature: String) -> Self {
        Block { account, previous, balance, signature }
    }

    pub fn hash(&self) -> String {

        hasher.update(serialized.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
