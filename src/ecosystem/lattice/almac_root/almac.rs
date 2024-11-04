// Current-State

use crate::ecosystem::lattice::borsys::block::*;
use crate::ecosystem::lattice::borsys::almac::*;

pub struct AlmacBlockHeader {
    bhash: String,
}

pub struct AlmacBlock {
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


}

pub struct AlmacContainerBlock<T> {
    // ALMAC CONTAINER
    almac_meta: T,
    almac_container: T,
}

pub struct AlmacBlockFooter {
    contentshash: BorneoContentsHash,
    signature: SignatureED25519,
}