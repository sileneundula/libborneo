// Account Data
use crate::internals::serde::{Serialize,Deserialize};
use crate::internals::crypto::blake2b::BorneoBLAKE2B;
use crate::internals::encoding::bs32::base32;

/// # BlockID
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
#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BlockID(u64);

/// # BorneoAccount
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

/// # BorneoBlockHash
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


impl BlockID {
    pub fn from(id: u64) -> Self {
        Self(id)
    }
    pub fn block_id(&self) -> u64 {
        return self.0
    }
}

impl BorneoBlockHash {
    /// Must be in hexadecimal and 80 characters long.
    /// 
    /// TODO: Add Validity check
    pub fn from_str<T: AsRef<str>>(s: T) -> Self {
        assert_eq!(s.as_ref().len(),80usize);
        
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

        return Self(borneoaccount)
    }
    pub fn borneo_account(&self) -> &str {
        return &self.0
    }
}