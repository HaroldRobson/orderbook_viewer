use crate::graphs::HistogramExample;
use eframe::egui;
use egui_file_dialog::FileDialog;
use egui_plot::Bar;
use polars::prelude::*;
use std::path::PathBuf;

pub struct App {
    pub file_dialog: FileDialog,
    pub picked_file: Option<PathBuf>,
    pub data: Option<polars::prelude::DataFrame>,
    pub histogram: HistogramExample,
    pub show_graph: bool,
    pub is_processing: bool,
    pub group_id: u32,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            file_dialog: FileDialog::new(),
            picked_file: None,
            data: None,
            show_graph: false,
            is_processing: false,
            histogram: HistogramExample::default(),
            group_id: 0,
        }
    }
    pub fn remake_bars(&mut self, group_id: u32) -> Result<(), PolarsError> {
        let filtered_df = self
            .data
            .as_ref()
            .unwrap()
            .clone()
            .lazy()
            .filter(col("group_id").eq(group_id))
            .collect()?;
        dbg!(self.data.as_ref().unwrap().schema());
        dbg!(self.data.as_ref().unwrap().column("group_id")?.unique()?);
        dbg!(&filtered_df.schema());
        println!("data height");
        dbg!(
            self.data
                .as_ref()
                .unwrap()
                .clone()
                .lazy()
                .collect()?
                .height()
        );
        println!("filtered data height");
        dbg!(filtered_df.height());
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
                    "green" => egui::Color32::GREEN,
                    _ => egui::Color32::BLUE,
                };
                Bar::new(p, s).fill(fill)
            })
            .collect();
        dbg!(&bars);
        self.histogram.bars = bars;
        Ok(())
    }
}
