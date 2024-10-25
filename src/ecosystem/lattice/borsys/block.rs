// Account Data
use crate::internals::serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoAccount(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoBlockHash(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoPublicKey(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoNonce(u64);

// ======Signatures=====

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct SignatureED25519(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct SignatureDillyn(String);

impl BorneoBlockHash {
    pub fn new<T: AsRef<str>>(s: T) -> Self {
        return Self(s.as_ref().to_string())
    }
}