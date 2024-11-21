mod heap;
use heap::MinHeap;
use eframe::{egui};

struct MyApp {
    min_heap: MinHeap,
    input_value: String,
    output_message: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            min_heap: MinHeap::new(),
            input_value: String::new(),
            output_message: String::new(),
        }
    }
}

impl MyApp {
    // Handle the insertion logic
    fn handle_insert(&mut self) {
        match self.validate_input() {
            Ok(num) => {
                self.min_heap.insert(num);
                self.set_message(&format!("Inserted {}", num));
                self.input_value.clear();
            }
            Err(msg) => self.set_message(msg),
        }
    }

    // Handle the extraction logic
    fn handle_extract_min(&mut self) {
        match self.min_heap.extract_min() {
            Some(min_val) => {
                self.set_message(&format!("Extracted minimum value: {}", min_val));
            }
            None => {
                self.set_message("Heap is empty!");
            }
        }
    }

    // Validate input and handle parsing errors
    fn validate_input(&self) -> Result<i32, &str> {
        self.input_value.trim().parse::<i32>().map_err(|_| "Invalid number!")
    }

    // Set output messages
    fn set_message(&mut self, msg: &str) {
        self.output_message = msg.to_string();
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Min Heap GUI"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Min Heap Operations");

            ui.horizontal(|ui| {
                ui.label("Value:");
                ui.text_edit_singleline(&mut self.input_value);
                
                if ui.button("Insert").clicked() {
                    self.handle_insert();
                }

                if ui.button("Extract Min").clicked() {
                    self.handle_extract_min();
                }
            });

            ui.label("Current Min Heap:");
            ui.label(self.min_heap.display());

            ui.label(&self.output_message);

            // Handle exit button
            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });

        // Set window title and size
        ctx.set_window_title("Min Heap GUI");
        ctx.set_window_size(egui::vec2(400.0, 300.0));
    }
}

fn main() -> Result<(), eframe::Error> {    
    eframe::run_native(
        "Min Heap GUI", 
        eframe::NativeOptions::default(), 
        Box::new(MyApp::default())
    )
}
