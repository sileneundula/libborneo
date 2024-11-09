use crate::ecosystem::lattice::borsys::block::*;
use std::collections::HashMap;

pub struct BorneoLatticeFullInit {    
    Accounts: Vec<BorneoAccount>,
    Accounts_Map: HashMap<BorneoAccount,BorneoAccount>,
    Delegates: Vec<BorneoAccount>,
    Delegates_Map: HashMap<BorneoAccount,BorneoAccount>,
}

pub struct SumatraPivotChainsFullInit {
    num_of_chains: u16,
    
}

pub struct YopoDelegateChainsFullInit {

}

pub struct SumatraPublicChainFullInit<T> {
    Blocks: Vec<T>
}

/* =====TASKS=====
- Count addresses
- Delegate provides security

Provide VDF

Nodes can accept or decline to their blockchain
*/