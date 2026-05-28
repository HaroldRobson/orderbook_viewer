use std::path::PathBuf;

use crate::app::App;
use crate::processor::process_file;
use eframe::egui;
use egui_file_dialog::FileDialog;

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
                            self.processed_file = Some(res);
                            self.is_processing = true;
                        }
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
            }

            if let Some(output) = &self.processed_file {
                ui.colored_label(egui::Color32::GREEN, format!("Processed: {:?}", output));
            }
        });
    }
}
