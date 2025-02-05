use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Clone,Hash,PartialEq,PartialOrd)]
pub struct NonceThresholdLevel(u64);

impl NonceThresholdLevel {
    // Should take a few seconds
    pub fn level1() -> Self {
        return Self(0xffffc00000000000u64)
    }
    // Should take 30 seconds
    pub fn level2() -> Self {
        return Self(0xfffffc0000000000u64)
    }
    // Should take 60-180 seconds (1-3 mins)
    pub fn level3() -> Self {
        return Self(0xffffffc000000000u64)
    }
    pub fn target(&self) -> u64 {
        return self.0
    }
}

