use crate::app::App;
use crate::graphs::HistogramExample;
use crate::processor::process_file;
use crate::timerange_picker;
use chrono::DateTime;
use chrono::NaiveTime;
use eframe::egui;
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
                        Ok(res) => {
                            let mut file = File::create("processed_output.parquet").unwrap();
                            ParquetWriter::new(&mut file)
                                .finish(&mut res.clone())
                                .unwrap();
                            self.main_df = Some(res.clone());
                            self.data = Some(res.clone());
                            let start_time = DateTime::from_timestamp_micros(
                                res.column("timestamp")
                                    .unwrap()
                                    .cast(&DataType::Time)
                                    .unwrap()
                                    .time()
                                    .unwrap()
                                    .get(0)
                                    .unwrap(),
                            )
                            .unwrap();
                            self.start_time = start_time;
                            self.is_processing = false;
                            self.show_graph = true;
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
                        self.main_df
                            .as_ref()
                            .unwrap()
                            .clone()
                            .lazy()
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
                }
            }

            if self.is_processing {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Processing data, please wait...");
                });
            }
            if ui
                .add_sized(
                    [800.0, 1.0],
                    egui::Slider::new(&mut self.group_id, 0..=self.max_group_id).text("My value"),
                )
                .changed()
            {
                let e = self.remake_bars(self.group_id);
                match e {
                    Err(e) => eprintln!("{:?}", e),
                    Ok(_) => {}
                }
            }
            /*
                        if ui.button("draw").clicked() {
                            self.show_graph = true;
                        }

                        if ui.button("redraw").clicked() {
                            let e = self.remake_bars(3);
                            match e {
                                Err(e) => eprintln!("{:?}", e),
                                Ok(_) => {}
                            }
                            self.histogram.show_plot(ui);
                        }
            */

            if self.show_graph {
                self.histogram.show_controls(ui);
                self.histogram.show_plot(ui);
            }
        });
    }
}
