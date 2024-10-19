use crate::internals::serde::{Serialize,Deserialize};


pub const boretotalsupplies: u64 = 23_625_225_000_000_000u64;
pub const boreblockreward: u64 = 50_000_000_000u64;




#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,PartialOrd,Hash)]
pub struct BoreAmount(u64);