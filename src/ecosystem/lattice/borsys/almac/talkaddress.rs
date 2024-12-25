/// # TalkAddress (taddr) Type System
/// 
/// This module consists of the TalkAddress type system.
/// 
/// It consists of using a digital signature to sign TalkAddr Spaces.

use serde::{Serialize,Deserialize};

/// # TalkAddr
/// 
/// A 48-byte string encoded in Base58.
#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub struct TalkAddr(String);

pub const TALKR_PREFIX: &str = "TALKR";
pub const TALKR_PREFIX_UNDERSCORE: &str = "_";

use crate::internals::crypto::hash::blake2b::BorneoBLAKE2B;
use crate::internals::encoding::bs58::base58::*;

impl TalkAddr {


    pub fn from_str<T: AsRef<str>>(talkaddr: T) -> Self {
        return Self(talkaddr.as_ref().to_string())
    }
    pub fn new<T: AsRef<[u8]>>(input: T) -> Self {
        let mut output = String::new();
        
        let hash = BorneoBLAKE2B::new(input.as_ref(), 48);
        let bytes = hex::decode(hash).expect("Failed To Decode To Bytes");
        let talkaddr = bytes.to_base58();

        output.push_str(TALKR_PREFIX);
        output.push_str(TALKR_PREFIX_UNDERSCORE);
        output.push_str(&talkaddr);

        
        
        return Self(output)
    }
    pub fn as_str(&self) -> &str {
        return self.0.as_str()
    }
    pub fn to_string(&self) -> String {
        return self.0.to_string()
    }
    pub fn address_only(&self) -> String {
        let s = &self.0;
        let output = s.replace("TALKR_","");
        return output
    }
}

#[test]
fn init() {
    let taddr = TalkAddr::new("");
    println!("{}",taddr.as_str());
}