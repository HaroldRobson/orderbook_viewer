/*

use chrono::NaiveTime;
use chrono::Timelike;
use egui::{DragValue, Response, Ui};

// full disclosure - this was written by AI since i couldnt be arsed
// although i had to rewrite a lot so maybe i should have been

pub struct TimeRangePicker<'a> {
    start: &'a mut NaiveTime,
    end: &'a mut NaiveTime,
}

impl<'a> TimeRangePicker<'a> {
    pub fn new(start: &'a mut NaiveTime, end: &'a mut NaiveTime) -> Self {
        Self { start, end }
    }

    pub fn ui(self, ui: &mut Ui) -> Response {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Start:");
                    Self::render_time_editor(ui, self.start);
                });

                ui.horizontal(|ui| {
                    ui.label("End:  ");
                    Self::render_time_editor(ui, self.end);
                });

                // Validation: Enforce End > Start
                if *self.end <= *self.start {
                    ui.colored_label(ui.visuals().error_fg_color, "⚠ End must be after Start");
                    // Optionally force-adjust:
                    // *self.end = *self.start + chrono::Duration::milliseconds(1);
                }
            });
        })
        .response
    }

    fn render_time_editor(ui: &mut Ui, time: &mut NaiveTime) {
        let mut h = time.hour();
        let mut m = time.minute();
        let mut s = time.second();
        let mut ms = time.nanosecond() / 1_000_000;

        let mut changed = false;

        ui.horizontal(|ui| {
            // Hours
            changed |= ui
                .add(DragValue::new(&mut h).range(0..=23).suffix("h"))
                .changed();
            ui.label(":");
            // Minutes
            changed |= ui
                .add(DragValue::new(&mut m).range(0..=59).suffix("m"))
                .changed();
            ui.label(":");
            // Seconds
            changed |= ui
                .add(DragValue::new(&mut s).range(0..=59).suffix("s"))
                .changed();
            ui.label(".");
            // Milliseconds
            changed |= ui
                .add(
                    DragValue::new(&mut ms)
                        .range(0..=999)
                        .speed(1.0)
                        .suffix("ms"),
                )
                .changed();
        });

        if changed {
            if let Some(new_time) = NaiveTime::from_hms_milli_opt(h, m, s, ms) {
                *time = new_time;
            }
        }
    }
}

*/

use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use egui::{DragValue, Response, Ui};

pub struct DateTimeRangePicker<'a> {
    start: &'a mut DateTime<Utc>,
    end: &'a mut DateTime<Utc>,
}

impl<'a> DateTimeRangePicker<'a> {
    pub fn new(start: &'a mut DateTime<Utc>, end: &'a mut DateTime<Utc>) -> Self {
        Self { start, end }
    }

    pub fn ui(self, ui: &mut Ui) -> Response {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Start:");
                    Self::render_datetime_editor(ui, self.start);
                });

                ui.horizontal(|ui| {
                    ui.label("End:  ");
                    Self::render_datetime_editor(ui, self.end);
                });

                // Validation: Enforce End > Start
                if *self.end <= *self.start {
                    ui.colored_label(ui.visuals().error_fg_color, "⚠ End must be after Start");

                    // Force-adjust to Start + 1ms if you want the window to "push"
                    if let valid_end = *self.start + chrono::Duration::milliseconds(1) {
                        *self.end = valid_end;
                    }
                }
            });
        })
        .response
    }

    fn render_datetime_editor(ui: &mut Ui, datetime: &mut DateTime<Utc>) {
        // Extract all components
        let mut year = datetime.year();
        let mut month = datetime.month();
        let mut day = datetime.day();
        let mut h = datetime.hour();
        let mut m = datetime.minute();
        let mut s = datetime.second();
        let mut ms = datetime.nanosecond() / 1_000_000;

        let mut changed = false;

        ui.horizontal(|ui| {
            // Date Part
            changed |= ui
                .add(DragValue::new(&mut year).range(2000..=2100).prefix("Y:"))
                .changed();
            changed |= ui
                .add(DragValue::new(&mut month).range(1..=12).prefix("M:"))
                .changed();
            changed |= ui
                .add(DragValue::new(&mut day).range(1..=31).prefix("D:"))
                .changed();

            ui.separator();

            // Time Part
            changed |= ui
                .add(DragValue::new(&mut h).range(0..=23).suffix("h"))
                .changed();
            changed |= ui
                .add(DragValue::new(&mut m).range(0..=59).suffix("m"))
                .changed();
            changed |= ui
                .add(DragValue::new(&mut s).range(0..=59).suffix("s"))
                .changed();
            changed |= ui
                .add(
                    DragValue::new(&mut ms)
                        .range(0..=999)
                        .speed(1.0)
                        .suffix("ms"),
                )
                .changed();
        });

        if changed {
            // Reconstruct the DateTime.
            // We use Utc.with_ymd_and_hms to safely handle out-of-range dates (like Feb 31st)
            if let Some(date_candidate) = Utc.with_ymd_and_hms(year, month, day, h, m, s).single() {
                if let Some(final_dt) = date_candidate.with_nanosecond(ms * 1_000_000) {
                    *datetime = final_dt;
                }
            }
        }
    }
}
