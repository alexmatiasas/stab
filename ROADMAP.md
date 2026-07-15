# stab — Roadmap

> **Vision:** `stab` aims to be what `git` is to source code — a versioning,
> diffing, validation, and transformation layer for data files. Fast by default,
> composable by design, and useful from day one.

---

## Guiding principles

- **Stateless first** — every command works standalone, no setup required
- **Stateful when needed** — `stab init` unlocks data versioning and lineage
- **Unix composable** — pipes in, pipes out, plays well with others
- **Rust performance** — no compromises on speed or memory
- **Honest UX** — explain what you're doing, warn before destructive ops

---

## Status

| Version | Status      | Focus |
|---------|-------------|-------|
| v0.1.0  | ✅ Released | Foundation — read, describe, JSON output |
| v0.2.0  | 🔄 In progress | I/O formats + conversion |
| v0.3.0  | 📋 Planned  | Pipes + explain mode |
| v0.4.0  | 📋 Planned  | TUI + pagination |
| v0.5.0+ | 💡 Vision   | See below |

---

## v0.2.0 — I/O and formats
> _Multi-format support and file conversion_

- `stab data convert` — CSV ↔ Parquet ↔ JSON ↔ Excel ↔ Feather
- `stab data schema` — detailed column type inspection
- `--output <file>` flag — export result to file
- Data validation on read (nulls, type mismatches)
- Integration tests with fixture files per format

**Supported formats:** CSV, JSON, Parquet, Excel (.xlsx), Feather

---

## v0.3.0 — Pipes and explain mode
> _Unix composability and transparency_

- Stdin support — read from pipes (`cat data.csv | stab data describe -`)
- `--explain` flag — show execution plan before running
- `--dry-run` flag — validate without executing
- Composable output for scripting

---

## v0.4.0 — TUI and presentation
> _Interactive terminal interface_

- Bidirectional pagination (rows and columns) with `ratatui`
- Smart truncation with summary of hidden rows/cols
- Column-level stats always visible in header
- Keyboard navigation — arrows, `q` to quit, `/` to search

---

## v0.5.0 — Smart Type Inference
> _Semantic type detection beyond basic dtypes_

- Detect semantic types: email, date, URL, phone, ID, category, boolean
- Warn about likely type mismatches (numeric IDs stored as integers, etc.)
- `stab data infer` — full inference report with confidence scores
- Suggest casts with `--apply` flag to execute them

---

## v0.6.0 — Data Profiling
> _The command people will share_

- `stab data profile` — comprehensive dataset report
- ASCII distribution histograms per numeric column
- Missing values heatmap
- Distinct value counts and top-N frequencies
- Exportable to HTML or Markdown
- Correlation matrix with visual heatmap

---

## v0.7.0 — I/O Benchmarking
> _Only possible with Rust performance_

- `stab bench io` — compare read/write speed and size across formats
- Real timing with criterion-quality measurements
- Recommendations based on dataset characteristics
- Memory usage tracking per format

---

## v0.8.0 — stab init + Lineage
> _Git for data — the core_

- `stab init` — initialize data tracking in a project
- `stab track` — register files for tracking
- `stab lineage show` — full transformation history
- `stab lineage blame` — which operation changed a column
- Stored in `.stab/lineage.toml`

---

## v0.9.0 — Snapshots + Fingerprinting + Semantic Diff
> _Versioning for data files_

- `stab snapshot save/restore/list` — point-in-time snapshots
- `stab data fingerprint` — reproducible hash of schema + content
- `stab data diff --semantic` — statistical diff between dataset versions
- Delta storage — only changes, not full copies
- `.stabignore` for excluding large files

---

## v0.10.0 — Data Contracts
> _great_expectations as a lightweight CLI_

- `stab contract create` — infer contract from reference dataset
- `stab contract verify` — validate dataset against contract
- Constraints: type, range, nullability, uniqueness, regex patterns
- CI-friendly exit codes for pipeline integration
- Row delta limits between versions

---

## v0.11.0 — Transformations and Recipes
> _Reproducible data pipelines_

- `stab data filter/select/sort/cast/fillna/drop-nulls`
- `stab recipe save/run/list` — save and replay transformation pipelines
- `--commit` flag — auto-snapshot after transformation
- Pipeline composition with pipes

---

## v0.12.0 — ML Features
> _The bridge to machine learning_

- `stab ml split` — train/test/validation split (random + stratified)
- `stab ml encode` — one-hot, label, ordinal encoding
- `stab ml scale` — min-max, standard, robust scaling
- `stab ml sample` — random and stratified sampling
- `stab ml balance` — oversample/undersample by label

---

## v0.13.0 — Query Language
> _SQL meets Unix pipes_

- `stab query` — custom pipeline query syntax
- Composable: `age > 30 | select name, salary | sort salary desc | top 10`
- Offline, predictable, no external dependencies
- Query validation with `--explain`

---

## v0.14.0 — External Sources
> _Bring data from anywhere_

- `stab db query` — SQL databases (SQLite, PostgreSQL, MySQL)
- `stab fetch` — REST APIs with auto-pagination
- `stab db export` — database table to any supported format
- Cloud storage (S3, GCS) — long-term

---

## v0.15.0 — Configuration
> _Personalize stab to your workflow_

- `~/.config/stab/config.toml` — global user preferences
- `.stab/config.toml` — per-project overrides
- CLI flags always win
- Configurable: colors, max rows, default format, null representations

---

## v1.0.0 — Public Release
> _Production ready_

- Pre-compiled binaries for macOS (arm64 + x86), Linux, Windows
- Published on crates.io
- Homebrew tap
- Complete documentation with examples
- Full test coverage
- Stable public API

---

## Beyond v1.0.0

- VSCode extension — dataset preview powered by `stab`
- `stab watch` — monitor live datasets with configurable alerts
- `stab repro` — full reproducibility capture for pipelines
- Arrow IPC, Avro, ORC format support
- All pandas I/O formats

---

_This roadmap is a living document. Features may shift between versions as_
_priorities evolve. Each version is designed to be useful on its own._