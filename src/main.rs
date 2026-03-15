use anyhow::Result;
use clap::Parser;
use stab::cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Data(args) => {
            stab::cli::data::handle(args)?;
        }
    }

    Ok(())
}
