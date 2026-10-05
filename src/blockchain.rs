use crate::block::Block;

pub struct BlockChain {
    pub chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        let genesis_block = vec![Block::new(0, "genesis".to_string(), String::new())];

        BlockChain {
            chain: genesis_block,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();
        let index = last_block.index + 1;
        let prev_hash: String = last_block.hash.clone();

        let new_block = Block::new(index, data, prev_hash);

        self.chain.push(new_block)
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            if self.chain[i - 1].hash != self.chain[i].prev_hash {
                return false;
            };

            
			let block = &self.chain[i];
            let recalculated_hash = Block::calculate_hash(block.index, &block.timestamp, &block.prev_hash, &block.data);
            if recalculated_hash != block.hash {
            	return false;
            }
        }

        true

    }
}
