use crate::display::table::{display_dataframe, display_dataframe_json};
use crate::errors::StabResult;
use crate::io::reader::read_csv;
use clap::{Args, Subcommand};
use polars::prelude::AnyValue;
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

    println!("shape: {} rows x {} cols\n", df.height(), df.width());

    let mut table = comfy_table::Table::new();

    table
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_header(vec![
            comfy_table::Cell::new("column")
                .add_attribute(comfy_table::Attribute::Bold)
                .fg(comfy_table::Color::Cyan),
            comfy_table::Cell::new("dtype")
                .add_attribute(comfy_table::Attribute::Bold)
                .fg(comfy_table::Color::Cyan),
            comfy_table::Cell::new("nulls")
                .add_attribute(comfy_table::Attribute::Bold)
                .fg(comfy_table::Color::Cyan),
            comfy_table::Cell::new("sample")
                .add_attribute(comfy_table::Attribute::Bold)
                .fg(comfy_table::Color::Cyan),
        ]);

    for col in df.get_column_names() {
        let series = df.column(col).unwrap();
        let nulls = series.null_count();
        let sample = df
            .column(col)
            .unwrap()
            .get(0)
            .map(|v| match v {
                AnyValue::String(s) => s.to_string(),
                other => other.to_string(),
            })
            .unwrap_or("-".to_string());

        table.add_row(vec![
            comfy_table::Cell::new(col),
            comfy_table::Cell::new(series.dtype().to_string()),
            comfy_table::Cell::new(nulls),
            comfy_table::Cell::new(sample),
        ]);
    }

    println!("{table}");
    Ok(())
}
