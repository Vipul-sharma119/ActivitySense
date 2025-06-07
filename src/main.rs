mod app;
mod event_log;
mod tracker;
mod history;
mod tray;

use app::TaskTrackerApp;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Activity Sense",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(TaskTrackerApp::new(cc))),
    )
}
