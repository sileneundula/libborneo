// Current-State

use crate::ecosystem::lattice::borsys::block::*;
use crate::ecosystem::lattice::borsys::almac::*;
use crate::internals::crypto::hash::blake2b::BorneoBLAKE2B;

use crate::internals::serde::{Serialize,Deserialize};

/*
MULTICAST

A multicast can be done to several chains

*/

pub struct AlmacBlockHeader {
    bhash: String,
}

#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub struct AlmacBlockContents {
    id: BlockID,
    ba: BorneoAccount,
    pk: BorneoPublicKey,
    
    entry_link_block: Option<BorneoLinkBlock>, //maybe
    link_hash: BorneoBlockHash,
    to: BorneoAccount,
    
    nonce: BorneoNonce,

    // ALMAC

    almac_version: AlmacVersion,
    almac_definitive_type: AlmacDefinitiveType,
    almac_tx: AlmacTxType,
    almac_action: Option<AlmacAction>,


}

pub struct AlmacBlockFooter {
    footerhash: BorneoFooterHash,
    signature: SignatureED25519,
}

impl AlmacBlockContents {
    pub fn new(id: BlockID, ba: BorneoAccount, pk: BorneoPublicKey, link_hash: BorneoBlockHash, to: BorneoAccount, nonce: BorneoNonce, version: AlmacVersion, almac_def_type: AlmacDefinitiveType, almac_tx: AlmacTxType) -> Self {
        Self {
            id: id,
            ba: ba,
            pk: pk,
            entry_link_block: None,
            link_hash: link_hash,
            to: to,
            nonce: nonce,
            
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


