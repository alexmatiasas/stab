use crate::errors::StabResult;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use polars::prelude::*;
use polars::prelude::{JsonFormat, JsonWriter, SerWriter};
use std::io::Cursor;

pub fn display_dataframe(df: &DataFrame) -> StabResult<()> {
    let mut table = Table::new();

    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(
            df.get_column_names()
                .iter()
                .map(|name| {
                    Cell::new(name)
                        .add_attribute(Attribute::Bold)
                        .fg(Color::AnsiValue(24))
                })
                .collect::<Vec<_>>(),
        );

    for i in 0..df.height() {
        let row: Vec<Cell> = df
            .get_column_names()
            .iter()
            .map(|col| Cell::new(df.column(col).unwrap().get(i).unwrap().to_string()))
            .collect();

        table.add_row(row);
    }

    println!("{table}");
    Ok(())
}

pub fn display_dataframe_json(df: &DataFrame) -> StabResult<()> {
    let mut buf = Cursor::new(Vec::new());

    JsonWriter::new(&mut buf)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df.clone())
        .map_err(|e| crate::errors::StabError::Display(e.to_string()))?;

    let json = String::from_utf8(buf.into_inner())
        .map_err(|e| crate::errors::StabError::Display(e.to_string()))?;

    println!("{json}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_df() -> DataFrame {
        df![
            "name" => ["Ana", "Bob", "Carlos"],
            "age"  => [28i64, 34, 22],
            "city" => ["CDMX", "Monterrey", "Oaxaca"]
        ]
        .unwrap()
    }

    #[test]
    fn test_display_dataframe_does_not_panic() {
        let df = make_test_df();
        let result = display_dataframe(&df);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_dataframe_json_does_not_panic() {
        let df = make_test_df();
        let result = display_dataframe_json(&df);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_dataframe_empty() {
        let df = DataFrame::empty();
        let result = display_dataframe(&df);
        assert!(result.is_ok());
    }
}
