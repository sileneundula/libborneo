use crate::ecosystem::lattice::block::Block;

#[derive(Debug)]
pub struct AccountChain {
    pub blocks: Vec<Block>,
}

impl AccountChain {
    pub fn new() -> Self {
        AccountChain { blocks: Vec::new() }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}