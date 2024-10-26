use std::collections::HashMap;

use super::account::AccountChain;

#[derive(Debug)]
pub struct BlockLattice {
    pub chains: HashMap<String, AccountChain>,
}

impl BlockLattice {
    pub fn new() -> Self {
        BlockLattice { chains: HashMap::new() }
    }

    pub fn add_block(&mut self, account: String, block: Block) {
        let chain = self.chains.entry(account.clone()).or_insert_with(AccountChain::new);
        chain.add_block(block);
    }

    pub fn get_latest_block(&self, account: &str) -> Option<&Block> {
        self.chains.get(account)?.latest_block()
    }
}