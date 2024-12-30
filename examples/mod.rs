extern crate libborneo;

pub mod almac;

use libborneo::ecosystem::lattice::almac_root::almac::AlmacBlock;
use libborneo::keys::GenerateKeypair;


fn main() {
    let (sk,pk) = GenerateKeypair::new();
    let almac = AlmacBlock::genesis(sk, libborneo::primitives::almac::AlmacDefinitiveType::AuthorityLinkTrust);
    almac.pretty_print();
}