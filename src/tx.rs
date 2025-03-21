use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TXInput {
  pub txid: String,
  pub vout: i32,
  pub script_sig: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TXOutput {
  pub value: i32,
  pub script_pub_key: String,
}



impl TXInput {
  pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
    self.script_sig == unlocking_data
  }
}

impl TXOutput {
  pub fn can_be_unlock_with(&self, unlocking_data: &str) -> bool {
    self.script_pub_key == unlocking_data
  }
}

