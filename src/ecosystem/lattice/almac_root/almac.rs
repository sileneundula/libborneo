/// # ALMAC (Account Ledger Management)
/// 
/// The ALMAC is the basic structure of libborneo. It consists of a single chain that is used to create new blocks or chains.
/// 
/// It uses ED25519 to verify digital signatures.

// Current-State

use libsumatracrypt_rs::signatures::ed25519::ED25519SecretKey;
use libsumatracrypt_rs::signatures::ed25519::ED25519Signature;
use crate::ecosystem::lattice::borsys::almac::talkaddress::TalkAddr;

use crate::ecosystem::lattice::borsys::block::*;
use crate::ecosystem::lattice::borsys::almac::*;
use crate::internals::crypto::hash::blake2b::BorneoBLAKE2B;

use crate::internals::serde::{Serialize,Deserialize};
use crate::primitives::proofofwork::NonceThresholdLevel;

/*
MULTICAST

A multicast can be done to several chains

*/
#[derive(Serialize,Deserialize,Clone)]

/// # ALMACBLOCK
pub struct AlmacBlock {
    // Contents + Nonce
    contents: AlmacBlockContents,
    nonce: AlmacBlockNonce,

    // Pivot
    footer: AlmacBlockFooter,

    //talkaddr: AlmacBlockTalkAddress,
}

impl AlmacBlock {
    pub fn genesis(sk: ED25519SecretKey, almac_type: AlmacDefinitiveType) -> Self {
        // Contents
        let contents = AlmacBlockContents::genesis(sk.clone(), almac_type);
        let hash = contents.digest();

        let nonce = AlmacBlockNonce::calculate(hash);
        let nonce_hash = nonce.digest();

        let footer = AlmacBlockFooter::new(&contents, &nonce, sk.clone());

        Self {
            contents: contents,
            nonce: nonce,
            footer: footer,
        }
    }
    pub fn serialize(&self) -> String {
        serde_json::to_string_pretty(&self).expect("Failed to convert")
    }
    pub fn pretty_print(&self) {
        println!("{}",self.serialize())
    }
    pub fn add_block(block: AlmacBlock, tx_type: AlmacTxType, action: AlmacAction, to: BorneoAccount) {
    }
    pub fn verify_block(block: AlmacBlock) {
        let contents = block.contents;
        let nonce = block.nonce;
        let footer = block.footer;
    }
}


#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub struct AlmacBlockPublicHeader {
    bhash: String,
}

/// # ALMAC BLOCK CONTENTS
#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub struct AlmacBlockContents {
    id: BlockID,
    ba: BorneoAccount,
    pk: BorneoPublicKey,
    
    entry_link_block: Option<BorneoLinkBlock>,
    link_hash: BorneoBlockHash,
    to: BorneoAccount,
    
    // ALMAC
    almac_version: AlmacVersion,
    almac_definitive_type: AlmacDefinitiveType,
    almac_tx: AlmacTxType,
    almac_action: Option<AlmacAction>,


}

#[derive(Serialize,Deserialize,Clone)]

pub struct AlmacBlockFooter {
    footerhash: BorneoFooterHash,
    signature: ED25519Signature,
}

impl AlmacBlockFooter {
    pub fn new(almac: &AlmacBlockContents, nonce: &AlmacBlockNonce, sk: ED25519SecretKey) -> Self {
        let mut output = String::new();
        let almac_hash = almac.digest();
        let nonce_hash = nonce.digest();

        output.push_str(&almac_hash);
        output.push_str(&nonce_hash);
        
        let digest = BorneoBLAKE2B::new(output.as_bytes(), 40);
        let signature = sk.sign(digest.clone());

        Self {
            footerhash: BorneoFooterHash::from(digest.clone()),
            signature: signature,
        }

    }
}

#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub struct AlmacBlockNonce {
    contents: String,
    nonce: BorneoNonce,
    target: NonceThresholdLevel,
}

/// # ALMAC BLOCK TALK ADDR
/// 
/// **TalkAddr** is the root point of the talkr address.
/// 
/// A hidden address can also be created by using Talkr to create a hash using a key.
#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub struct AlmacBlockTalkAddress {
    // TalkrAddr
    address: talkaddress::TalkAddr,
    
    // HeaderAddress
    public_header_address: talkaddress::TalkAddr,
    
    // Sidechain
    ownedaddress: Option<talkaddress::TalkAddr>,
}

impl AlmacBlockContents {
    pub fn genesis(sk: ED25519SecretKey, type_of_almac: AlmacDefinitiveType) -> Self {

        // Public Key
        let pk = sk.to_public_key();
        
        // Calculate BorneoAccount
        let ba: BorneoAccount = BorneoAccount::get_from_ed25519_pk(pk.to_string().as_str());

        // BorneoPublicKey
        let borpk = BorneoPublicKey::from_str(&pk.to_string().as_str());

        // Link Hash For Init
        let link_hash = BorneoBlockHash::from_str("InitialALMAC");

        // Almac Version
        let almac_version = AlmacVersion::from_str("Testnet");

        // Almac Transaction
        let almac_tx = AlmacTxType::Genesis;

        // Nonce
        let nonce = BorneoNonce::from(0);
        
        // Contents Block
        let block = Self {
                id: BlockID::from(0u64),
                ba: ba.clone(),
                pk: borpk,
                entry_link_block: None,
                link_hash: link_hash,
                almac_version: almac_version,
                almac_definitive_type: type_of_almac,
                to: ba.clone(),
                almac_tx: almac_tx,
                almac_action: None,
            };
        // Hash
        return block
    }
    pub fn new(id: BlockID, ba: BorneoAccount, pk: BorneoPublicKey, link_hash: BorneoBlockHash, to: BorneoAccount, version: AlmacVersion, almac_def_type: AlmacDefinitiveType, almac_tx: AlmacTxType) -> Self {
        Self {
            id: id,
            ba: ba,
            pk: pk,
            entry_link_block: None,
            link_hash: link_hash,
            to: to,
            
            almac_version: version,
            almac_definitive_type: almac_def_type,
            almac_tx: almac_tx,
            almac_action: None,
        }
    }
    pub fn serialize(&self) -> String {
        let serialized = serde_json::to_string(&self).expect("Failed To Serialize");
        return serialized
    }
    pub fn deserialize<T: AsRef<str>>(serde: T) -> Self {
        return serde_json::from_str(serde.as_ref()).expect("Failed to deserialize");
    }
    pub fn digest(&self) -> String {
        let serialized = serde_json::to_string(&self).expect("Failed To Serialize");
        let hash = BorneoBLAKE2B::new(serialized.as_bytes(),40usize);
        return hash
    }
    pub fn pretty_print(&self) {
        let serialized = serde_json::to_string_pretty(&self).expect("Failed To Serialize");
        println!("{}",serialized);

    }
}

impl AlmacBlockNonce {
    pub fn calculate<T: AsRef<str>>(contents_hash: T) -> Self {
        let hash = contents_hash.as_ref();
        let bytes = Self::decode_from_hex(&contents_hash);

        let x=  Self::new(&bytes, 0xffffffc000000000u64);
        
        Self {
            nonce: BorneoNonce::from(x),
            contents: hash.to_owned(),
            target: NonceThresholdLevel::level3(),
        }
    }
    fn decode_from_hex<T: AsRef<str>>(s: T) -> Vec<u8> {
        return hex::decode(s.as_ref()).expect("Failed To Decode From Hex")
    }
    pub fn new(input: &[u8], threshold: u64) -> u64 {
        let nonce: u64 = blake2b_pow::mine(input, threshold);

        return nonce
    }
    pub fn verify(input: &[u8], nonce: u64, threshold: u64) -> bool {
        return blake2b_pow::verify_nonce(input, threshold, nonce)
    }
    pub fn serialize(&self) -> String {
        return serde_json::to_string(self).expect("Failed to convert to JSON For Nonce")
    }
    pub fn digest(&self) -> String {
        return BorneoBLAKE2B::new(self.serialize().as_bytes(), 40)
    }
}

#[test]
fn nonce() {
    println!("Starting PoW");
    //let x = AlmacBlockNonce::new(&[27u8;40],0xffffffc000000000);
    let x = AlmacBlockNonce::verify(&[27u8;40], 10919841u64, 0xffffffc000000000);
    println!("Nonce: {}",x);
}

#[test]
fn creation() {
    use crate::keys::GenerateKeypair;
    let (sk,pk) = GenerateKeypair::new();

    let block = AlmacBlockContents::genesis(sk, AlmacDefinitiveType::General);
    block.pretty_print();
}