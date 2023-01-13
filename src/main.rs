use anyhow::Result;

mod cli;
mod models;
mod utils;

fn main() -> Result<()> {
    cli::read_args()?;
    Ok(())
}
