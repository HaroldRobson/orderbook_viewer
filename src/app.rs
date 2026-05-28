use std::path::PathBuf;

use eframe::egui;
use egui_file_dialog::FileDialog;

pub struct App {
    pub file_dialog: FileDialog,
    pub picked_file: Option<PathBuf>,
    pub processed_file: Option<PathBuf>,
    pub is_processing: bool,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            file_dialog: FileDialog::new(),
            picked_file: None,
            processed_file: None,
            is_processing: false,
        }
    }
}
