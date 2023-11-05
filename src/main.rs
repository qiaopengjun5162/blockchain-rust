use cli::Cli;
use errors::Result;

mod block;
mod blockchain;
mod cli;
mod errors;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}
