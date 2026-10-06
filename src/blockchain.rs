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

    pub fn add_block(&mut self, data: String) -> Result<(), &'static str> {
        let last_block = self.chain.last().ok_or("chain is empty")?;
        let index = last_block.index + 1;
        let prev_hash: String = last_block.hash.clone();

        let new_block = Block::new(index, data, prev_hash);

        self.chain.push(new_block);
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.chain.len() {
            let block = &self.chain[i];
            let recalculated_hash = block.calculate_hash();
            if recalculated_hash != block.hash {
                return false;
            }
            if i > 0 && block.prev_hash != self.chain[i - 1].hash {
                return false;
            }
        }

        true
    }
}
