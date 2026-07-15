# stab

A fast, composable CLI for data inspection built in Rust. Inspired by pandas.
Designed for the terminal.

![demo](assets/demo.gif)

## What is stab?

`stab` is a command-line tool for working with data files directly from your
terminal — no Python environment, no Jupyter, no overhead.

It combines the inspection power of pandas, the performance of Polars, and the
composability of Unix pipelines into a single fast binary.

**Long-term vision:** `stab` aims to be what `git` is to code — a versioning,
diffing, and validation layer for data files. See [ROADMAP.md](ROADMAP.md)
for the full plan.

## Features (v0.1.0)

- Read CSV files and display as a formatted ASCII table
- Inspect schema — column names, dtypes, null counts, sample values
- Export to JSON for pipeline integration
- Built on [Polars](https://pola.rs) — handles large files with ease

## Installation

> Requires [Rust](https://rustup.rs) installed.

```bash
cargo install stab
```

Or build from source:

```bash
git clone https://github.com/alexmatiasas/stab
cd stab
cargo build --release
./target/release/stab --help
```

## Usage

### Read a file

```bash
stab data read data.csv
stab data read data.csv --output json
```

### Inspect schema

```bash
stab data describe data.csv
```

## Project Structure

```
src/
├── main.rs             # CLI entry point
├── lib.rs              # Library root
├── errors.rs           # Custom error types
├── cli/
│   ├── mod.rs          # CLI definition (clap)
│   └── data.rs         # data subcommands
├── io/
│   ├── reader.rs       # CSV reader (Polars)
│   └── writer.rs       # Placeholder for future writes
├── display/
│   └── table.rs        # Table and JSON rendering
├── stats/              # Placeholder — descriptive statistics
└── transform/          # Placeholder — data transformations
```

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust (2024 edition) |
| CLI | [clap](https://docs.rs/clap) with derive |
| Data | [Polars](https://pola.rs) |
| Display | [comfy-table](https://docs.rs/comfy-table) |
| Errors | [thiserror](https://docs.rs/thiserror) + [anyhow](https://docs.rs/anyhow) |
| Testing | `cargo test` + [assert_cmd](https://docs.rs/assert_cmd) |

## Development

### Prerequisites

- Rust (via [rustup](https://rustup.rs))
- pre-commit (`brew install pre-commit`)

### Setup

```bash
git clone https://github.com/alexmatiasas/stab
cd stab
pre-commit install
pre-commit install --hook-type commit-msg
cargo build
cargo test
```

### Running tests

```bash
cargo test                  # all tests
cargo test io::             # only io module tests
cargo test integration      # only integration tests
```

## License

ISC — see [LICENSE](LICENSE).
