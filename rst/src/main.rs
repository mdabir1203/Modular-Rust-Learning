mod ui;
mod components;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Minimal GUI", options, Box::new(|_cc| Box::new(ui::MyApp::default())));
}
