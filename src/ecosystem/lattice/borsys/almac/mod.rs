use crate::internals::serde::{Serialize,Deserialize};

/// # [ALMACSYS::0x0000] ALMACVERSION
/// 
/// The ALMACVERSION contains the current version of ALMAC being used
#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub struct AlmacVersion(String);

#[derive(Serialize,Deserialize,Clone,Copy,Hash,PartialEq,PartialOrd)]
pub struct AlmacAction(u32);

/// # \[ALMACSYS::0x0001] AlamcDefType (ADT)
/// 
/// ## Description
/// 
/// ## Types
/// 
/// - Core
/// 
/// - Contract
/// 
/// - Position
/// 
/// ### \[AlmacType::Core::0x00] General
/// 
/// The basic type used for general transactions. This chain servers as a creator for other chains/blocks/txs.
/// 
/// ### \[AlmacType::Contract::0x00] General Ledger Contract Chain (GLCC)
/// 
/// A built-in ledger chain for performing certain transactions. Uses `BridgeClosure` for confirming transactions.
/// 
/// ### \[AlmacType::Contract::0x01] User Ledger Contract Chain
/// 
/// A user custom built contract for performing certain transactions.
/// 
/// ### \[AlmacType::Position::0x00] Delegate Address
/// 
/// A delegate position. Delegates can be voted in,
#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub enum AlmacDefinitiveType {
    // Core
    General,
    
    // Contract
    GLCC,
    ULCC(u8),

    // Governance
    Delegate(u8),
    DAO(u16),

    // DB
    Database(u8),

    Pivot(u16),
    Registar(u16),
    Bank(u8),
    
    // PKI
    X59Authority(u16),
    TrustWeb,
    
    // Bridge
    GeneralBridge(u16),
    BorneoBridge(String),
    
    // Storage
    Strg(u16),
    Collection,
    
    // Executables/Scripts/Bridges
    Executable(u32),
    Script(u32),
    BorneoBridgeStrg(u32),

    // Node
    GeneralNode(u8),
}

#[derive(Serialize,Deserialize,Clone,Copy,Hash,PartialEq,PartialOrd)]
pub enum AlmacTxType {
    Genesis,
    
    // Registration
    Register,
    
    // Common
    Send,
    Receive,
    Confirm, // Confirms multiple transactions
    
    // Bridge
    AutoBridge,


    Sort,
    
    // Almac
    InitGeneralChain(u16),
    InitGeneralBlock(u32),

    InitStorageUnit,
    InitNamespace,
    InitDecDB,
    InitWebICD,
    InitBridge,
}


/*
CHAINS:
1. Payment, Voting, 
2. Certificate, Projects

Have a custom blockchain which only use is to serve to collect the user data and act as entry points

*/