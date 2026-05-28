use std::path::PathBuf;

use eframe::egui;
use egui_file_dialog::FileDialog;

pub mod app;
pub mod graphs;
use app::App;

mod file_picker;

pub mod processor;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "File dialog example",
        eframe::NativeOptions::default(),
        Box::new(|ctx| Ok(Box::new(App::new(ctx)))),
    )
}
