use clap::{arg, Command};

use crate::blockchain::Blockchain;
use crate::utils::Result;





pub struct Cli {
  bc: Blockchain
}

impl Cli {
    pub fn new () -> Result<Cli> {
      Ok(Cli {
        bc: Blockchain::new()?
      })
    }

    pub fn run (&mut self) -> Result<()> {
      let matches = Command::new("rust-blockchain-cli")
        .version("0.1")
        .author("Maria Volkova")
        .about("a simple blockchain for learning")
        .subcommand(
          Command::new("printchain")
          .about("print all of the chain blocks")
        )
        .subcommand(
          Command::new("addblock")
          .about("add block in the blockchain")
          .arg(arg!(<DATA>" 'the blockchain data'"))
        )
        .get_matches();

      if let Some(ref matches)  = matches.subcommand_matches("addblock") {
        if let Some(data) = matches.get_one::<String>("DATA") {
          self.addblock(String::from(data))?;
        } else {
          println!("Not printing testing lists...")
        }
      }

      if let Some(_) = matches.subcommand_matches("printchain") {
        self.print_chain();
      }


      Ok(())
    }

    fn addblock(&mut self, data: String) -> Result<()> {
      self.bc.add_block(data);
      Ok(())
    }

    fn print_chain(&self) -> Result<()> {
      for b in self.bc.iter() {
        println!("Block: {:#?}", b);
      }
      Ok(())
    }

}