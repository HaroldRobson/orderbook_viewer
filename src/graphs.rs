use crate::app::App;

use eframe::egui;

use eframe::egui::Response;
use egui_plot::Bar;
use egui_plot::BarChart;
use egui_plot::Legend;
use egui_plot::Plot;

pub struct HistogramExample {
    vertical: bool,
    pub bars: Vec<egui_plot::Bar>,
}

impl Default for HistogramExample {
    fn default() -> Self {
        Self {
            vertical: true,
            bars: vec![],
        }
    }
}

impl HistogramExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Orientation:");
            ui.selectable_value(&mut self.vertical, true, "Vertical");
            ui.selectable_value(&mut self.vertical, false, "Horizontal");
        })
        .response
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let mut chart =
            BarChart::new("OrderBook", self.bars.clone()).color(egui::Color32::LIGHT_BLUE);

        if !self.vertical {
            chart = chart.horizontal();
        }

        Plot::new("OrderBook")
            .legend(Legend::default())
            .clamp_grid(true)
            .allow_zoom(egui::Vec2b::new(true, true))
            .allow_drag(egui::Vec2b::new(true, true))
            .allow_scroll(egui::Vec2b::new(true, true))
            .auto_bounds(true)
            .show(ui, |plot_ui| plot_ui.bar_chart(chart))
            .response
    }

    fn name(&self) -> &'static str {
        "histogram"
    }

    fn title(&self) -> &'static str {
        "Histogram Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to create histograms using bar charts. It displays a normal distribution with customizable orientation, zoom, drag, and scroll controls."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["histogram", "bar_chart"]
    }

    fn code_bytes(&self) -> &'static [u8] {
        include_bytes!("./app.rs")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        self.show_plot(ui)
    }
}
