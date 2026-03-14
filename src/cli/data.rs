use crate::display::table::{display_dataframe, display_dataframe_json};
use crate::errors::StabResult;
use crate::io::reader::read_csv;
use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct DataArgs {
    #[command(subcommand)]
    pub command: DataCommands,
}

#[derive(Debug, Subcommand)]
pub enum DataCommands {
    /// Read a file and display it as a table
    Read(ReadArgs),

    /// Show basic information about a file (shape, dtypes, nulls)
    Describe(DescribeArgs),
}

#[derive(Debug, Args)]
pub struct ReadArgs {
    /// Path to the input file
    pub file: PathBuf,

    /// Output format: table (default), json
    #[arg(long, short, default_value = "table")]
    pub output: OutputFormat,
}

#[derive(Debug, Args)]
pub struct DescribeArgs {
    /// Path to the input file
    pub file: PathBuf,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}

pub fn handle(args: DataArgs) -> StabResult<()> {
    match args.command {
        DataCommands::Read(read_args) => handle_read(read_args),
        DataCommands::Describe(describe_args) => handle_describe(describe_args),
    }
}

fn handle_read(args: ReadArgs) -> StabResult<()> {
    let df = read_csv(&args.file)?;

    match args.output {
        OutputFormat::Table => display_dataframe(&df)?,
        OutputFormat::Json => display_dataframe_json(&df)?,
    }

    Ok(())
}

fn handle_describe(args: DescribeArgs) -> StabResult<()> {
    let df = read_csv(&args.file)?;

    // TODO v0.1.0 — describe output
    println!("shape: {:?}", df.shape());
    println!("columns: {:?}", df.get_column_names());

    Ok(())
}
