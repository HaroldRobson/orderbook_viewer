use crate::app::App;
use crate::graphs::HistogramExample;
use crate::processor::process_file;
use crate::timerange_picker;
use chrono::DateTime;
use chrono::NaiveTime;
use eframe::egui;
use eframe::egui::{Color32, RichText, Ui};
use egui_file_dialog::FileDialog;
use polars::prelude::*;
use std::fs::File;
use std::path::PathBuf;

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if ui.button("Picked file").clicked() {
                self.file_dialog.pick_file();
            }

            ui.label(format!("Picked file: {:?}", self.picked_file));

            if let Some(path) = self.file_dialog.update(ui).picked() {
                self.picked_file = Some(path.to_path_buf());
                if ui.button("Process").clicked() && self.picked_file.is_some() {
                    self.is_processing = true;
                    let path = self.picked_file.as_ref().unwrap().as_path();
                    let result = process_file(path);
                    match result {
                        Ok(lf) => {
                            let options = ParquetWriteOptions::default();
                            /*
                                                        lf.clone()
                                                            .with_streaming(true)
                                                            .sink_parquet("processed_output.parquet", options)
                                                            .unwrap();
                            */
                            self.main_lf = Some(lf.clone());
                            self.data = Some(lf.clone().limit(2000).collect().unwrap());

                            let stats = lf
                                .select([
                                    col("timestamp").first().alias("start_time"),
                                    col("timestamp").last().alias("end_time"),
                                    col("group_id").last().alias("max_group_id"),
                                ])
                                .collect()
                                .unwrap();
                            // reallt proud of this error handling.....

                            self.start_time = DateTime::from_timestamp_micros(
                                stats
                                    .column("start_time")
                                    .unwrap()
                                    .i64()
                                    .unwrap()
                                    .get(0)
                                    .unwrap(),
                            )
                            .unwrap();

                            self.end_time = DateTime::from_timestamp_micros(
                                stats
                                    .column("end_time")
                                    .unwrap()
                                    .i64()
                                    .unwrap()
                                    .get(0)
                                    .unwrap(),
                            )
                            .unwrap();

                            self.max_group_id = stats
                                .column("max_group_id")
                                .unwrap()
                                .u32()
                                .unwrap()
                                .get(0)
                                .unwrap();

                            self.is_processing = false;
                        }
                        Err(e) => eprintln!("{:?}", e),
                    }
                }

                let tdp = timerange_picker::DateTimeRangePicker::new(
                    &mut self.start_time,
                    &mut self.end_time,
                )
                .ui(ui);
                if ui.button("truncate").clicked() {
                    self.data = Some(
                        self.main_lf
                            .as_ref()
                            .unwrap()
                            .clone()
                            .with_streaming(true)
                            .filter(col("timestamp").gt_eq(lit(self.start_time.timestamp_micros())))
                            .filter(col("timestamp").lt_eq(lit(self.end_time.timestamp_micros())))
                            .collect()
                            .unwrap(),
                    );
                    self.max_group_id = self
                        .data
                        .as_ref()
                        .unwrap()
                        .column("group_id")
                        .unwrap()
                        .u32()
                        .unwrap()
                        .max()
                        .unwrap();
                    self.show_graph = true;
                }
            }

            if self.is_processing {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Processing data, please wait...");
                });
            }

            if self.show_graph {
                if ui
                    .add_sized(
                        [ui.available_width(), 20.0],
                        egui::Slider::new(&mut self.group_id, 0..=self.max_group_id)
                            .text("My value"),
                    )
                    .changed()
                {
                    let e = self.remake_bars(self.group_id);
                    match e {
                        Err(e) => eprintln!("{:?}", e),
                        Ok(_) => {}
                    }
                }
                let formatted = self
                    .current_orderbook_timestamp
                    .format("%Y-%m-%d %H:%M:%S%.6f")
                    .to_string();
                ui.label(
                    RichText::new(formatted)
                        .monospace()
                        .color(ui.visuals().strong_text_color()),
                );
                self.histogram.show_controls(ui);
                self.histogram.show_plot(ui);
            }
        });
    }
}
