use crate::block::{self, Block};
use crate::utils::Result;
use crate::r#const::TARGET_HEXT;

#[derive(Debug)]
pub struct Blockchain {
  current_hash: String,
  db: sled::Db,
}

impl Blockchain {
  
  pub fn new () -> Result<Blockchain> {

    let db = sled::open("data/blocks")?;

    match db.get("LAST")? {
      Some(hash) => {
        let last_hash = String::from_utf8(hash.to_vec())?;

        Ok(Blockchain {
          current_hash: last_hash,
          db
        })
      },
      None => {
        let block = Block::new_genesis_block();
        db.insert(block.get_hash(), bincode::serialize(&block)?)?;
        db.insert("LAST", block.get_hash().as_bytes())?;

        let bc = Blockchain {
          current_hash: block.get_hash(),
          db
        };

        bc.db.flush();

        Ok(bc) 
      },
    }
  }

  pub fn add_block(&mut self, data: String) -> Result<()> {
    let last_hash = self.db.get("LAST")?.unwrap();


    let new_block = Block::new_block(data, String::from_utf8(last_hash.to_vec())?, TARGET_HEXT)?;

    self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
    self.db.insert("LAST", new_block.get_hash().as_bytes())?;
    self.current_hash = new_block.get_hash();

    Ok(())
  }

  pub fn iter (&self) -> BlockChainIter {
    BlockChainIter {
      current_hash: self.current_hash.clone(),
      bc: &self
    }
  }

}

pub struct BlockChainIter<'a> {
  current_hash: String,
  bc: &'a Blockchain,
}

impl <'a> Iterator for BlockChainIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
      if let Ok(encode_block) = self.bc.db.get(&self.current_hash) {
        return match encode_block {
            Some(b) => {
              if let Ok(block) = bincode::deserialize::<Block>(&b) {
                self.current_hash = block.get_prev_hash();
                Some(block)
              } else {
                None
              }
            },
            None => None,
        };
      }
      
      None
    }


}





#[cfg(test)]
#[warn(unused_must_use)]
mod test {
  use super::*;

  #[test]

  fn test_blockchain() {
    let mut blockchain = Blockchain::new().unwrap();
    blockchain.add_block("data".to_string()).unwrap();
    blockchain.add_block("data2".to_string()).unwrap();
    blockchain.add_block("data3".to_string()).unwrap();

    for item in blockchain.iter() {
      println!("item {:?}", item)
    }


    // dbg!(blockchain);
  }

}