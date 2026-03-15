pub mod data;

use clap::{Parser, Subcommand};

/// stab — statistics for table CLI for data science and ML.
/// A fast CLI for data inspection and transformation, built on Rust and Polars.
#[derive(Debug, Parser)]
#[command(
    name = "stab",
    version,
    about = "A fast CLI for data inspection and transformation",
    long_about = None,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Data ingestion and inspection commands
    Data(data::DataArgs),
}
