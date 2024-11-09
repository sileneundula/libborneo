use block::BorneoAccount;
use libborneo::ecosystem::lattice::almac_root::almac::AlmacBlockContents;
use libborneo::ecosystem::lattice::borsys::*;

pub struct AlmacChainExample;

impl AlmacChainExample {
    pub fn new() {
        // Step 1. Set Block ID
        let id: u64 = 0;

        // Step 2. Retrieve BorneoAccount From ED25519 Public Key Using 40 byte Blake2b digest encoded in base32 (Z)
        let ba: BorneoAccount = BorneoAccount::get_from_ed25519_pk("d920b5aac55f21af78b83fcf03e0eddf2a44e2da8e19b41dfef3dd7d13cc01b3a5a4149e69196687686c22bce36e811944202bd27b6d78b026a3bab8bbc433a5");

        println!("BorneoAccount: {:?}", ba);
    }
}