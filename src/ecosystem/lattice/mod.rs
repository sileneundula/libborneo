pub mod borsys;

use crate::internals::crypto::blake2b;
use crate::internals::serde::{Serialize,Deserialize};
use crate::internals::crypto::blake3;

use borsys::block::*;
use borsys::payment::BoreAmount;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub account: BorneoAccount,
    pub previous: BorneoBlockHash,
    pub balance: BoreAmount,
    pub nonce: u64,
    pub signature: SignatureED25519,
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
