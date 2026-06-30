pub mod cli;
pub mod format;
pub mod writer;

use anyhow::Result;

fn main() -> Result<()> {
    cli::Args::run()?;

    Ok(())
}
