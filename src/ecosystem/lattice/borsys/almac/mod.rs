use crate::internals::serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Clone,Copy,Hash,PartialEq,PartialOrd)]
pub struct AlmacVersion(String);

#[derive(Serialize,Deserialize,Clone,Copy,Hash,PartialEq,PartialOrd)]
pub struct AlmacAction(u32);

#[derive(Serialize,Deserialize,Clone,Copy,Hash,PartialEq,PartialOrd)]
pub enum AlmacDefinitiveType {
    General,
    GLCC,
    ULCC(u8),
    Delegate(u8),
    Pivot(u16),
    DAO(u16),
    Registar(u16),
    Bank(u8),
    
    X59Authority(u16),
    
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
}


/*
1. Init-Chain
2. Init-Block
10. Register

Have a custom blockchain which only use is to serve to collect the user data and act as entry points

*/