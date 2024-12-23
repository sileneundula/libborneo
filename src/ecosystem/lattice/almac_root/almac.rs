/// # ALMAC (Account Ledger Management)
/// 
/// The ALMAC is the basic structure of libborneo. It consists of a single chain that is used to create new blocks or chains.
/// 
/// It uses ED25519 to verify digital signatures.

// Current-State

use libsumatracrypt_rs::signatures::ed25519::ED25519SecretKey;

use crate::ecosystem::lattice::borsys::block::*;
use crate::ecosystem::lattice::borsys::almac::*;
use crate::internals::crypto::hash::blake2b::BorneoBLAKE2B;

use crate::internals::serde::{Serialize,Deserialize};

/*
MULTICAST

A multicast can be done to several chains

*/
#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]

pub struct AlmacBlock {
    // Contents
    contents: AlmacBlockContents,
    nonce: AlmacBlockNonce,

    footer: AlmacBlockFooter,
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
    
    entry_link_block: Option<BorneoLinkBlock>, //maybe
    link_hash: BorneoBlockHash,
    to: BorneoAccount,
    
    // ALMAC
    almac_version: AlmacVersion,
    almac_definitive_type: AlmacDefinitiveType,
    almac_tx: AlmacTxType,
    almac_action: Option<AlmacAction>,


}

#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]

pub struct AlmacBlockFooter {
    nonce: BorneoNonce,
    target_threshold: BorneoNonceThreshold,

    footerhash: BorneoFooterHash,
    signature: SignatureED25519,
}

#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub struct AlmacBlockNonce {
    contents: String,
    nonce: BorneoNonce,
    target: BorneoNonceThreshold,
}

impl AlmacBlockContents {
    pub fn genesis(sk: ED25519SecretKey, type_of_almac: AlmacDefinitiveType) {

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
    pub fn hash(&self) -> String {
        let serialized = serde_json::to_string(self).expect("Failed To Serialize");
        let hash = BorneoBLAKE2B::new(serialized.as_bytes(),40usize);
        return hash
    }
}

impl AlmacBlockNonce {
    pub fn new(input: &[u8], threshold: u64) -> u64 {
        let nonce: u64 = blake2b_pow::mine(input, threshold);

        return nonce
    }
    pub fn verify(input: &[u8], nonce: u64, threshold: u64) -> bool {
        return blake2b_pow::verify_nonce(input, threshold, nonce)
    }
}

#[test]
fn nonce() {
    println!("Starting PoW");
    //let x = AlmacBlockNonce::new(&[27u8;40],0xffffffc000000000);
    let x = AlmacBlockNonce::verify(&[27u8;40], 10919841u64, 0xffffffc000000000);
    println!("Nonce: {}",x);
}