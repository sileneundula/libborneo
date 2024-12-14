use libborneo::ecosystem::lattice::borsys::block::*;


/// # Example: BorneoAccount
pub fn example_get_borneo_account() {
    // Borneo Account From String
    let ba = BorneoAccount::from_str("BA59_unrcsnzfr1founx1hgu3byxik5r8seww3t5hg5xjbw7wqphmjgh8t5hnca9grnep");

    // ED25519 Public Key
    let ba_from_pk = BorneoAccount::get_from_ed25519_pk("d1c328c14282fb7aab6de708a401cd1c856df3957caf697ef02436842a7519271cddf6ebb321b4e1d996af0f8d7332e0a79b891bb8ae14cb4945498389dcc64c");

    println!("BorneoAccount: {:?}", ba_from_pk)
}