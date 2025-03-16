use cli::Cli;

mod block;
mod blockchain;
mod error;
pub mod r#const;
mod cli;
mod transaction;
mod tx;

fn main() -> error::Result<()> {

  let mut cli = Cli::new()?;

  cli.run()?;

  Ok(())
}
