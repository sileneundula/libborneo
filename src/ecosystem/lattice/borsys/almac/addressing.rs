#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub struct AddressRelationship(Relationship);
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
/// # Relationship Between Addresses
/// 
/// Child: A Child Chain is one that is initiated into the chain using same public key
/// ExternallyOwned: An Externally Owned Chain is one that has its own self
/// Signed: A Signed Chain Is One Where Others Can Communicate With
pub enum Relationship {
    Child,
    ExternalOwned,
    Signed,
}