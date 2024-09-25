pub mod vault;
pub mod errors;
pub mod utils;
pub mod gui;

use eframe::run_native;
use log::{error};
use arboard::Clipboard;
use std::time::Duration;
use std::thread;

fn copy_to_clipboard(data: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    
    for _ in 0..3 { // Retry up to 3 times
        match clipboard.set_text(data.to_string()) {
            Ok(_) => return Ok(()),
            Err(e) => {
                error!("Clipboard error: {}", e);
                thread::sleep(Duration::from_millis(500)); // Wait before retrying
            }
        }
    }
    
    Err("Failed to set clipboard text after multiple attempts.".to_string())
}

fn main() {
    env_logger::init();
    
    // Example usage of copying to clipboard
    if let Err(e) = copy_to_clipboard("Hello, World!") {
        error!("Error copying to clipboard: {}", e);
    }

    let native_options = eframe::NativeOptions::default();
    run_native("Vault App", native_options, Box::new(|cc| Ok(Box::new(gui::VaultApp::new(cc)))));
}