use libsumatracrypt_rs::signatures::ed25519::ED25519PublicKey;

// Account Data
use crate::internals::serde::{Serialize,Deserialize};
use crate::internals::crypto::hash::blake2b::BorneoBLAKE2B;
use crate::internals::encoding::bs32::base32;

/// # \[Borsys::0x0001] BlockID
/// 
/// **BorsysIndex:** 0x0001
/// **InternalType:** `u64`
/// 
/// This includes the `u64` of a block id starting with zero. This type is used for all chains.
/// 
/// It includes the functions:
/// 
/// - from()
/// - block_id()
/// 
/// 
/// ## Code Example
/// 
/// ```rust
/// use libborneo::primitives::block::BlockID;
/// 
/// fn main() {
/// 
///     // Primtive Type For Block ID At Block Index Zero
///     let bid_zero: BlockID = BlockID::from(0u64);
/// 
///     let bid_one: BlockID = BlockID::from(1u64);
/// 
///     let block_id: u64 = bid_zero.block_id();
/// 
/// 
/// }
/// ```
#[derive(Serialize,Deserialize,Clone,Copy,Debug,PartialEq,PartialOrd,Hash)]
pub struct BlockID(u64);

/// # \[Borsys::0x0002] BorneoAccount
/// 
/// **BorsysIndex:** 0x0002
/// 
/// **Prefix:** BA59_
/// 
/// **Encoding:** BASE32-z
/// 
/// **Process:** BLAKE2B(40,PK) -> Decode Hex Into Bytes -> Encode In Base32-z with Prefix
/// 
/// ## Description
/// 
/// **BorneoAccount** is the `borsys primitive type` for addresses. It is used to send transactions and as a result can be looked as the account address. It is computed using the ED25519 Public Key encoded in hexadecimal.
/// 
/// ## Methods
/// 
/// - `from_str(s: T)`
/// 
/// - `get_from_pk_ed25519()`
/// 
/// - `borneo_account()`
#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoAccount(String);

/// # \[Borsys::0x0003] BorneoBlockHash
/// 
/// **BorsysIndex:** 0x0003
/// 
/// **HashFunction:** BLAKE2B(36)
/// 
/// **Encoding:** Hexadecimal (Upper)
/// 
/// **Length:** 72-characters
/// 
/// ## Description
/// 
/// The BorneoBlockHash is a `borsys primitive type` that acts as the block hash for a specific block.
#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoBlockHash(String);

/// # \[BorSys::0x0004] BorneoPublicKey
/// 
/// **Encoding:** Hexadecimal (Upper)
/// 
/// **SignatureAlgorithm:** ED25519
/// 
/// ## Description
/// 
/// The BorneoPublicKey exists as a `primitive type` for verifying digital signatures and computing the `BorneoAccount`.
#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoPublicKey(String);

/// # \[Borsys::0x0005] BorneoNonce
/// 
/// **Type:** `u64`
/// 
/// ## Description
/// 
/// The BorneoNonce acts as a nonce that can be used to calculate the pow required to make a tx.
#[derive(Serialize,Deserialize,Clone,Copy,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoNonce(u64);

/// # \[Borsys::0x0005::01]
/// 
/// **Type:** `u64`
/// 
/// ## Description
/// 
/// BorneoNonceThreshold is a way of measuring different security levels for BorneoNonces.
/// 
/// It uses BLAKE2B to generate the hash. The nonce must cause the hash to be larger than a certain number to pass.
#[derive(Serialize,Deserialize,Clone,Copy,Debug,PartialEq,PartialOrd,Hash)]

pub struct BorneoNonceThreshold(u64);


/// # \[Borsys::0x0006] BorneoLinkBlock
/// 
/// ## Description
/// 
/// The BorneoLinkBlock is used to link blocks to the previous transaction
#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoLinkBlock(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoContentsHash(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoContainerHash(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoFooterHash(String);

impl BorneoFooterHash {
    pub fn from<T: AsRef<str>>(s: T) -> Self {
        Self(s.as_ref().to_string())
    }
}

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BorneoSidechainHash(String);

// ======Signatures=====

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct SignatureED25519(String);

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct SignatureDillyn(String);


impl BlockID {
    pub fn from(id: u64) -> Self {
        Self(id)
    }
    pub fn block_id(&self) -> u64 {
        return self.0
    }
    pub fn increment(&self) -> Self {
        return Self(self.0 + 1u64)
    }
}

impl BorneoBlockHash {
    /// Must be in hexadecimal and 80 characters long.
    /// 
    /// TODO: Add Validity check
    pub fn from_str<T: AsRef<str>>(s: T) -> Self {
        //assert_eq!(s.as_ref().len(),80usize);
        
        return Self(s.as_ref().to_string())
    }
    pub fn hash<T: AsRef<[u8]>>(bytes: T) -> Self {
        return BorneoBlockHash(BorneoBLAKE2B::new(bytes.as_ref(), 40usize))
    }
    pub fn get_hash(&self) -> &str {
        return &self.0
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

        return Self(borneoaccount.as_str().to_string())
    }
    pub fn borneo_account(&self) -> &str {
        return &self.0
    }
}

impl BorneoLinkBlock {
    pub fn from_str<T: AsRef<str>>(s: T) -> Self {
        return Self(s.as_ref().to_string())
    }
    pub fn block_link(&self) -> &str {
        return &self.0
    }
    pub fn compare_link<T: AsRef<str>>(&self, s: T) -> bool {
        if &self.0 == s.as_ref() {
            return true
        }
        else {
            return false
        }
    }
}

impl BorneoPublicKey {
    pub fn from_str<T: AsRef<str>>(pk: T) -> BorneoPublicKey {
        BorneoPublicKey(pk.as_ref().to_string())
    }
    pub fn to_key(&self) -> ED25519PublicKey {
        ED25519PublicKey::new(&self.0)
    }
}

impl BorneoNonce {
    pub fn from(int: u64) -> Self {
        return Self(int)
    }
    pub fn nonce(&self) -> u64 {
        return self.0
    }
}