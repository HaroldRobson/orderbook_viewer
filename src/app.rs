use crate::graphs::HistogramExample;
use chrono::{DateTime, Utc};
use eframe::egui;
use egui_file_dialog::FileDialog;
use egui_plot::Bar;
use polars::prelude::*;
use std::path::PathBuf;

pub struct App {
    pub file_dialog: FileDialog,
    pub picked_file: Option<PathBuf>,
    pub main_lf: Option<polars::prelude::LazyFrame>,
    pub data: Option<polars::prelude::DataFrame>,
    pub histogram: HistogramExample,
    pub show_graph: bool,
    pub is_processing: bool,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub group_id: u32,
    pub max_group_id: u32,
    pub current_orderbook_timestamp: DateTime<Utc>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        let time = chrono::Utc::now();
        Self {
            file_dialog: FileDialog::new(),
            picked_file: None,
            main_lf: None,
            data: None,
            show_graph: false,
            is_processing: false,
            start_time: time.clone(),
            end_time: time.clone(),
            histogram: HistogramExample::default(),
            group_id: 0,
            max_group_id: 20,
            current_orderbook_timestamp: time,
        }
    }
    pub fn remake_bars(&mut self, group_id: u32) -> Result<(), PolarsError> {
        let filtered_df = self
            .data
            .as_ref()
            .unwrap()
            .clone()
            .lazy()
            .filter(
                col("group_id")
                    .cast(DataType::UInt32)
                    .eq(lit(group_id as u32)),
            )
            .collect()?;
        let price = filtered_df.column("price")?.f64()?;
        let amount = filtered_df.column("amount")?.f64()?;
        let color = filtered_df.column("color")?.str()?;
        let bars: Vec<Bar> = price
            .into_no_null_iter()
            .zip(amount.into_no_null_iter())
            .zip(color.into_no_null_iter())
            .map(|((p, s), c)| {
                let fill = match c {
                    "red" => egui::Color32::RED,
                    _ => egui::Color32::GREEN,
                };
                Bar::new(p, s).fill(fill)
            })
            .collect();
        self.histogram.bars = bars;
        self.current_orderbook_timestamp = DateTime::from_timestamp_micros(
            filtered_df
                .column("timestamp")
                .unwrap()
                .i64()
                .unwrap()
                .get(0)
                .unwrap(),
        )
        .unwrap();
        Ok(())
    }
}
