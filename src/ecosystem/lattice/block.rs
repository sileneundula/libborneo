use crate::internals::crypto::blake2b;
use crate::internals::serde::{Serialize,Deserialize};
use crate::internals::crypto::blake3;
use crate::internals::serde::serde_json;

use crate::ecosystem::lattice::borsys::block::*;
use crate::ecosystem::lattice::borsys::payment::BoreAmount;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub account: BorneoAccount,
    pub previous: BorneoBlockHash,
    pub balance: BoreAmount,
    pub nonce: BorneoNonce,
    pub signature: SignatureED25519,
}

impl Block {
    pub fn new(account: String, previous: String, balance: u64, signature: String) -> Self {
        Block { account, previous, balance, signature }
    }

    pub fn hash(&self) -> BorneoBlockHash {
        let serialized = serde_json::to_string(self).unwrap()
        let hash = blake2b::BorneoBLAKE2B::new(serialized.as_bytes(), 40usize);
        return BorneoBlockHash::new(hash)
    }
}