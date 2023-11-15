use cli::Cli;
use errors::Result;

mod block;
mod blockchain;
mod cli;
mod errors;
mod server;
mod transaction;
mod tx;
mod utxoset;
mod wallet;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}
