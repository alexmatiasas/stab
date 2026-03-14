use crate::errors::StabResult;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use polars::prelude::*;

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
