use cli::Cli;

mod block;
mod blockchain;
mod utils;
pub mod r#const;
mod cli;

fn main() -> utils::Result<()> {

  let mut cli = Cli::new()?;

  cli.run()?;

  Ok(())
}
