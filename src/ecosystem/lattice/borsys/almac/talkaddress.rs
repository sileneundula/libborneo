use serde::{Serialize,Deserialize};

/// # TalkAddr
/// 
/// A 48-byte string encoded in Base58.
#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub struct TalkAddr(String);

use crate::internals::crypto::hash::blake2b::BorneoBLAKE2B;
use crate::internals::encoding::bs58::base58::*;

impl TalkAddr {
    pub fn from_str<T: AsRef<str>>(talkaddr: T) -> Self {
        return Self(talkaddr.as_ref().to_string())
    }
    pub fn init<T: AsRef<[u8]>>(input: T) {
        let hash = BorneoBLAKE2B::new(input.as_ref(), 48);
        let bytes = hex::decode(hash).expect("Failed To Decode To Bytes");
        let talkaddr = bytes.to_base58();
    }
}