pub mod cli;
pub mod format;
pub mod utils;
pub mod writer;

use anyhow::Result;

fn main() -> Result<()> {
    cli::CliArgs::run()?;

    Ok(())
}
