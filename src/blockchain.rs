use crate::block::Block;
use crate::utils::Result;
use crate::r#const::TARGET_HEXT;

#[derive(Debug)]
pub struct Blockchain {
  blocks: Vec<Block>,
}

impl Blockchain {
  
  pub fn new () -> Blockchain {
    Blockchain {
      blocks: vec![Block::new_genesis_block()]
    }
  }

  pub fn add_block(&mut self, data: String) -> Result<()> {
    let prev_block = self.blocks.last().unwrap();
    let new_block = Block::new_block(data, prev_block.get_hash(), TARGET_HEXT)?;

    self.blocks.push(new_block);

    Ok(())
  }
}

#[cfg(test)]
#[warn(unused_must_use)]
mod test {
  use super::*;

  #[test]

  fn test_blockchain() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("data".to_string()).unwrap();
    blockchain.add_block("data2".to_string()).unwrap();
    blockchain.add_block("data3".to_string()).unwrap();

    dbg!(blockchain);
  }

}