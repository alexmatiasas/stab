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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_read_csv_valid_file() {
        let path = Path::new("tests/fixtures/sample.csv");
        let df = read_csv(path).unwrap();

        assert_eq!(df.height(), 3);
        assert_eq!(df.width(), 3);
    }

    #[test]
    fn test_read_csv_column_names() {
        let path = Path::new("tests/fixtures/sample.csv");
        let df = read_csv(path).unwrap();

        let cols = df.get_column_names();
        assert_eq!(cols, vec!["name", "age", "city"]);
    }

    #[test]
    fn test_read_csv_invalid_path() {
        let path = Path::new("tests/fixtures/nonexistent.csv");
        let result = read_csv(path);

        assert!(result.is_err());
    }
}
