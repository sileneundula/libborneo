// Account Data
use crate::internals::serde::{Serialize,Deserialize};
use crate::internals::crypto::blake2b::BorneoBLAKE2B;
use crate::internals::encoding::bs32::base32;

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BlockID(u64);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoAccount(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoBlockHash(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoPublicKey(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoNonce(u64);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoLinkBlock(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoContentsHash(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoContainerHash(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoFooterHash(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoSidechainHash(String);

// ======Signatures=====

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct SignatureED25519(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct SignatureDillyn(String);

impl BorneoBlockHash {
    /// Must be in hexadecimal and 80 characters long.
    /// 
    /// TODO: Add Validity check
    pub fn from_str<T: AsRef<str>>(s: T) -> Self {
        assert_eq!(s.as_ref().len(),80usize);
        
        return Self(s.as_ref().to_string())
    }
}

impl BorneoAccount {
    /// Prefixes Each Account Address With BA59_
    pub const BAPREFIX: &str = "BA59";
    pub const BAPREFIX_UNDERSCORE: &str = "_";

    pub fn from_str<T: AsRef<str>>(s: T) -> Self {
        return Self(s.as_ref().to_string())
    }
    pub fn get_from_ed25519_pk<T: AsRef<str>>(s: T) -> Self {
        let mut borneoaccount: String = String::new();
        
        let res = BorneoBLAKE2B::new(s.as_ref(), 40);
        let decoded_bytes = hex::decode(res).expect("[GET FROM ED@5519] Failed To Convert From Hex");
        let bs32_z_encoded = base32::encode(base32::Alphabet::Z, &decoded_bytes);

        borneoaccount.push_str(Self::BAPREFIX);
        borneoaccount.push_str(Self::BAPREFIX_UNDERSCORE);
        borneoaccount.push_str(&bs32_z_encoded);

        return Self(borneoaccount)
    }
}