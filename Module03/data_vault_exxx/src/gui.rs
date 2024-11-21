use eframe::egui;
use crate::vault::Vault;
use crate::utils::{show_progress};
use std::path::Path;

#[derive(Default)]
pub struct VaultApp {
    operation: String,
    status: String,
    vault: Option<Vault>,
    vault_contents: String,
    import_key: String, // Field to hold the key for import
    import_value: String, // Field to hold the value for import
    password: String, // Field to hold the password for loading/saving
}

impl VaultApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for VaultApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Vault Operations");

            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    self.operation = "save".to_string();
                    self.status = self.save_vault(ctx);
                }

                ui.add_space(20.0); // Add space between buttons

                if ui.button("Load").clicked() {
                    self.operation = "load".to_string();
                    self.status = self.load_vault(ctx);
                }

                ui.add_space(20.0); // Add space before the Clear button

                if ui.button("Clear").clicked() {
                    self.vault = None; // Clear the vault
                    self.vault_contents.clear(); // Clear the displayed contents
                    self.status.clear(); // Clear the status message
                }

                ui.add_space(20.0); // Add space before the Import button

                if ui.button("Import").clicked() {
                    self.import_vault_data(ui);
                }
            });

            ui.label(&self.status);
            ui.label("Vault Contents:");
            ui.label(&self.vault_contents);

            // Password input field
            ui.horizontal(|ui| {
                ui.label("Password:");
                ui.text_edit_singleline(&mut self.password);
            });
        });
    }
}

impl VaultApp {
    fn save_vault(&mut self, ctx: &egui::Context) -> String {
        let vault_file = "vault.dat";
        if Path::new(vault_file).exists() {
            return "Vault already exists.".to_string();
        }
    
        if self.password.is_empty() {
            return "Password cannot be empty.".to_string();
        }
    
        let mut vault = Vault::new();
        vault.add_entry("email".to_string(), "uknowwho12@gmail.com".to_string());
        vault.add_entry("api_key".to_string(), "542342".to_string());
        vault.add_entry("secret_key".to_string(), "radish".to_string());
    
        show_progress();
    
        match vault.save_to_file(vault_file, &self.password) {
            Ok(_) => {
                self.vault = Some(vault);
                self.password.clear(); // Clear the password field
                self.status = "Vault secured successfully".to_string(); // Update status with success message
                self.vault_contents.clear(); // Optionally clear vault contents if needed
                "Vault secured successfully".to_string() // Return the success message
            },
            Err(e) => format!("Failed to save vault: {}", e),
        }
    }

    fn load_vault(&mut self, ctx: &egui::Context) -> String {
        let vault_file = "vault.dat";
        if !Path::new(vault_file).exists() {
            return "Vault file does not exist.".to_string();
        }

        if self.password.is_empty() {
            return "Password cannot be empty.".to_string();
        }

        show_progress();

        match Vault::load_from_file(vault_file, &self.password) {
            Ok(loaded_vault) => {
                self.vault = Some(loaded_vault);
                self.vault_contents = self.vault.as_ref().unwrap().data.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join("\n");
                "Vault loaded successfully".to_string()
            },
            Err(e) => format!("Failed to load vault: {}", e),
        }
    }

    fn import_vault_data(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("Import Vault Entry");

            ui.horizontal(|ui| {
                ui.label("Key:");
                ui.text_edit_singleline(&mut self.import_key);
            });

            ui.horizontal(|ui| {
                ui.label("Value:");
                ui.text_edit_singleline(&mut self.import_value);
            });

            if ui.button("Add Entry").clicked() {
                if !self.import_key.is_empty() && !self.import_value.is_empty() {
                    if let Some(vault) = &mut self.vault {
                        vault.add_entry(self.import_key.clone(), self.import_value.clone());
                        self.status = format!("Added entry: {} -> {}", self.import_key, self.import_value);
                        self.import_key.clear();
                        self.import_value.clear();
                    } else {
                        self.status = "Vault is not initialized.".to_string();
                    }
                } else {
                    self.status = "Key and Value cannot be empty.".to_string();
                }
            }
        });
    }
}