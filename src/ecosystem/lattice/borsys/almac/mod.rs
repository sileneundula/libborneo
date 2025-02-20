use crate::internals::serde::{Serialize,Deserialize};

/// TalkAddr
pub mod talkaddress;

pub mod addressing;

/// # [ALMACSYS::0x0000] ALMACVERSION
/// 
/// The ALMACVERSION contains the current version of ALMAC being used
#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub struct AlmacVersion(String);

#[derive(Serialize,Deserialize,Clone,Copy,Hash,PartialEq,PartialOrd)]
pub struct AlmacAction(u32);

/// # \[ALMACSYS::0x0001] AlamcDefinitiveType (ADT)
/// 
/// ## Description
/// 
/// ## Types
/// 
/// - Core
/// 
/// - Contract
/// 
/// - Governance
/// 
/// - Organization
/// 
/// - Database
/// 
/// - Lament
/// 
/// - Pivot
/// 
/// - Registar
/// 
/// - Bank
/// 
/// - PKI
/// 
/// - Social
/// 
/// - Executable
/// 
/// - Bridge
/// 
/// - Connector
/// 
/// - Node
/// 
/// - Slinky
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
/// ### \[AlmacType::Governance::0x00] Delegate Address
/// 
/// A delegate position. Delegates can be voted in, can offer services, and can make payments.
/// 
/// ### \[AlmacType::Org::0x00] DAO
/// 
/// A decentralized organization
/// 
/// ### \[AlmacType::DB::0x00] OpenDatabase
/// 
/// An opendatabase that is done p2p.
/// 
/// ### \[AlmacType::Lament::0x00] LamentSearch
/// 
/// An open search engine that uses tagging for finding data.
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
    OpenDatabase(u8), // Includes dbconnector

    // LamentSearch
    LamentSearch, // (LamSer)
    LamentTagggingSystem, // (LamTagSys)
    LamentIndex(u8),
    LamentPivot,

    // Pivot
    Pivot(u16),
    
    // Registar
    Registar(u16),
    
    // Bank
    Bank(u8),
    
    // PKI
    X59Authority(u16),
    OpenTrustWeb, // (OTW)
    AuthorityLinkTrust, // (ALT) | A Way of Linking together accounts

    // Social
    FedoraSocial, // A social account
    
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
    GeneralNode(u16),

    // Connector
    Connector(u8),

    // OpenRepo
    OpenRepo(u8),

    // Slinky
    SlinkyService(u8),
}

#[derive(Serialize,Deserialize,Clone,Copy,Hash,PartialEq,PartialOrd)]
pub enum AlmacTxType {
    Genesis, // [0x0000] Genesis Tx
    
    // Registration
    Register, // [0x0001] Register Tx
    RegisterDelegate, // [0x0002] Register Delegate Tx
    RegisterDAO, // [0x0003] Register DAO Tx
    RegisterBank, // [0x0004] Register Bank Tx
    RegisterX59Authority, // [0x0005] Register X59 Authority Tx
    RegisterOpenTrustWeb, // [0x0006] Register Open Trust Web Tx
    RegisterAuthorityLinkTrust, // [0x0007] Register Authority Link Trust Tx
    RegisterFedoraSocial, // [0x0008] Register Fedora Social Tx
    RegisterOpenDatabase, // [0x0009] Register Open Database Tx
    RegisterLamentSearch, // [0x000A] Register Lament Search Tx
    RegisterLamentTaggingSystem, // [0x000B] Register Lament Tagging System Tx
    RegisterLamentIndex, // [0x000C] Register Lament Index Tx
    RegisterLamentPivot, // [0x000D] Register Lament Pivot Tx
    RegisterRegistar, // [0x000E] Register Registar Tx
    RegisterStrg, // [0x000F] Register Strg Tx
    RegisterCollection, // [0x0010] Register Collection Tx
    RegisterExecutable, // [0x0011] Register Executable Tx
    RegisterScript, // [0x0012] Register Script Tx
    RegisterBorneoBridgeStrg, // [0x0013] Register Borneo Bridge Strg Tx
    RegisterGeneralNode, // [0x0014] Register General Node Tx
    RegisterConnector, // [0x0015] Register Connector Tx
    RegisterOpenRepo, // [0x0016] Register Open Repo Tx
    RegisterSlinkyService, // [0x0017] Register Slinky Service Tx
    RegisterGeneralBridge, // [0x0018] Register General Bridge Tx
    RegisterBorneoBridge, // [0x0019] Register Borneo Bridge Tx

    InitPaymentSystem, // [0x001A] Init Payment System Tx
    InitVotingSystem, // [0x001B] Init Voting System Tx
    InitCertificateSystem, // [0x001C] Init Certificate System Tx
    
    // Common
    Send,
    Receive,
    Confirm, // Confirms multiple transactions
    
    // Bridge
    AutoBridge,


    Sort,
    
    // Almac
    InitSetupChain, // Setup Chain

    InitGeneralChain,

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

impl AlmacVersion {
    pub fn from_str<T: AsRef<str>>(s: T) -> Self {
        Self(s.as_ref().to_string())
    }
}