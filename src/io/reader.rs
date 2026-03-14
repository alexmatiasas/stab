use crate::errors::{StabError, StabResult};
use polars::prelude::*;
use std::path::{Path, PathBuf};

pub fn read_csv(path: &Path) -> StabResult<DataFrame> {
    CsvReadOptions::default()
        .with_has_header(true)
        .with_infer_schema_length(Some(100))
        .try_into_reader_with_file_path(Some(path.to_path_buf()))
        .map_err(|e| StabError::FileRead {
            path: PathBuf::from(path),
            source: e,
        })?
        .finish()
        .map_err(|e| StabError::FileRead {
            path: PathBuf::from(path),
            source: e,
        })
}
