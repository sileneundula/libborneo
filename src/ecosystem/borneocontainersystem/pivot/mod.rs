use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Data<T> {
    info: String,
    content: T,
}

#[derive(Debug, Clone)]
struct BorneoBlockChain {
    index: u64,
    timestamp: u128,
    data: Data,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: Data, previous_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let hash = Block::calculate_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u64, timestamp: u128, data: &Data, previous_hash: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data.info, previous_hash);
        format!("{:x}", md5::compute(input))
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, Data { info: "Genesis Block".to_string() }, String::new());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: Data) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(previous_block.index + 1, data, previous_block.hash.clone());
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != Block::calculate_hash(current_block.index, current_block.timestamp, &current_block.data, &current_block.previous_hash) {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(Data { info: "Block 1 Data".to_string() });
    blockchain.add_block(Data { info: "Block 2 Data".to_string() });

    println!("{:#?}", blockchain);
    println!("Is blockchain valid? {}", blockchain.is_valid());
}