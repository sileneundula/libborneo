pub struct NonceThresholdLevel(u64);

impl NonceThresholdLevel {
    // Should take 60-180 seconds (1-3 mins)
    pub fn level3() -> u64 {
        return 0xffffffc000000000u64
    }
}

