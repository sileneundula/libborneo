use libborneo::primtives::*;

pub fn block_id() {
    let bid1 = block_id::new(0u64);
    let bid2 = block_id::new(1u64);

    let block_id = bid1.block_id();
}