pub struct BorneoLatticeFullInit {    
    Accounts: Vec<BorneoAccount>,
    Accounts_Map: HashMap<BorneoAccount>,
    Delegates: Vec<BorneoAccount>,
    Delegates_Map: HashMap<BorneoAccount>,
    
    num_of_delegates: u16,
}

pub struct SumatraDelegatedChainsFullInit {
    csprng_chain: Vec<>
    domain_status: Vec<>
}

pub struct SumatraPublicChainFullInit {
    Blocks: Vec
}

/* =====TASKS=====
- Count addresses
- Delegate provides security

Provide VDF

Nodes can accept or decline to their blockchain
*/

pub struct BorneoApp {
    Account: Vec
    
}