use polars::prelude::*;
use std::fs::File;
use uuid::Uuid;

use anyhow::{Context, Result};
use std::path::Path;
use std::path::PathBuf;

pub fn process_file<'a>(file_path: &'a Path) -> Result<PathBuf> {
    println!("began processing file");
    let lf = LazyFrame::scan_parquet(file_path, ScanArgsParquet::default())?;

    println!("reached0");
    let lf = lf
        .with_columns([
            col("side").cast(DataType::String),
            col("price").cast(DataType::Float64),
            col("amount").cast(DataType::Float64),
        ])
        .sort(["timestamp"], SortMultipleOptions::default())
        .with_column(
            when(col("side").eq(lit("bid")).or(col("side").eq(lit("Bid"))))
                .then(lit("green"))
                .otherwise(lit("red"))
                .alias("color"),
        )
        .with_column(
            col("timestamp")
                .cast(DataType::Datetime(TimeUnit::Milliseconds, None))
                .alias("datetime"),
        )
        .with_column(
            col("timestamp")
                .rank(
                    RankOptions {
                        method: RankMethod::Dense,
                        descending: false,
                    },
                    None,
                )
                .over(["timestamp"])
                .alias("group_id"),
        );
    println!("reached1");
    match lf.collect() {
        Ok(df) => {}
        Err(e) => eprint!("{:?}", e),
    };
    println!("reached2");

    let output_name = format!("{}_processed.parquet", Uuid::new_v4());
    let mut output_path = file_path.to_path_buf().clone();
    output_path.set_file_name(output_name);
    //    processed_lf.sink_parquet(&output_path, ParquetWriteOptions::default())?;

    println!("done");
    Ok(output_path)
}
