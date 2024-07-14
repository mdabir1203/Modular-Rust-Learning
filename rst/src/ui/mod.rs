use eframe::egui;

pub struct MyApp;

impl Default for MyApp {
    fn default() -> Self {
        Self
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::CtxRef, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Content goes here...");
        });
    }
}
