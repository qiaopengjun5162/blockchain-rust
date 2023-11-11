use cli::Cli;
use errors::Result;

mod block;
mod blockchain;
mod cli;
mod errors;
mod transaction;
mod tx;
mod wallet;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}
