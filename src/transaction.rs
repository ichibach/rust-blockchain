use crypto::{digest::Digest, sha2::Sha256};
use failure::format_err;
use log::error;
use serde::{Deserialize, Serialize};
use crate::{blockchain::Blockchain, error::Result};
use crate::tx::{TXInput, TXOutput};

///Transaction represents a Bitcoin transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
  pub id: String,
  pub vin: Vec<TXInput>,
  pub vout: Vec<TXOutput>,
}

impl Transaction {
  pub fn new_coinbase (to: String, mut data: String) -> Result<Transaction> {

    if data == String::from("") {
      data += &format!("Reward to '{}'", to);
    }

    let mut tx = Transaction {
      id: String::new(),
      vin: vec![TXInput {
        txid: String::new(),
        vout: -1,
        script_sig: data
      }],
      vout: vec![TXOutput {
        value: 100,
        script_pub_key: to
      }]
    };

    Ok(tx)
  }

  fn set_id (&mut self) -> Result<()> {
    let mut hasher = Sha256::new();
    let data = bincode::serialize(self)?;

    hasher.input(&data);
    self.id = hasher.result_str();

    Ok(())
  }

  pub fn is_coinbase(&self) -> bool {
    self.vin.len() == 1 && self.vin[0].txid.is_empty() && self.vin[0].vout == -1
  } 

  pub fn new_UTXO(from: &str, to: &str, amount: i32, bc: &Blockchain) -> Result<Transaction> {
    let mut vin = Vec::new();
    let (balance, unspent_outputs) = bc.find_unspent_outputs(from, amount);

    if balance < amount {
      error!("Not Enough balance");
      return Err(format_err!("Not enough balance: current balance {}", balance));
    }

    for (txid, outs) in unspent_outputs {
      for out in outs {
        vin.push(TXInput {
          txid: txid.clone(),
          vout: out,
          script_sig: String::from(from)
        });
      }
    }

    let mut vout = vec![TXOutput {
      value: amount,
      script_pub_key: String::from(to)
    }];

    if balance > amount {
      vout.push(TXOutput {
        value: balance - amount,
        script_pub_key: String::from(from)
      });
    }

    let mut tx = Transaction {
      id: String::new(),
      vin,
      vout
    };

    tx.set_id()?;

    Ok(tx)
  }

}

