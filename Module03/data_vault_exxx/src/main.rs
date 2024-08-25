mod vault;
mod errors;
mod utils;

use std::env;
use std::path::Path;

use crate::vault::Vault;
use crate::utils::{get_password, show_progress};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let operation = args.get(1).map(String::as_str);

    match operation {
        Some("save") => {
            let vault_file = "vault.dat";
            if Path::new(vault_file).exists() {
                println!("Password saved already.");
                return Ok(());
            }
            let mut vault = Vault::new();
            vault.add_entry("email".to_string(), "uknowwho12@gmail.com".to_string());
            vault.add_entry("api_key".to_string(), "542342".to_string());

            let password = get_password("Enter a password to secure the vault: ")?;
            show_progress();
            vault.save_to_file(vault_file, &password)?;
            println!("Vault secured successfully");
        }
        Some("load") => {
            let vault_file = "vault.dat";
            if !Path::new(vault_file).exists() {
                println!("Vault doesn't exist. Run the save command.");
                return Ok(());
            }
            let password = get_password("Enter the password to open the vault: ")?;
            show_progress();

            match Vault::load_from_file(vault_file, &password) {
                Ok(loaded_vault) => {
                    println!("Vault loaded successfully");
                    if let Some(email) = loaded_vault.get_entry("email") {
                        println!("Email: {}", email);
                    }
                    if let Some(api_key) = loaded_vault.get_entry("api_key") {
                        println!("API Key: {}", api_key);
                    }
                }
                Err(e) => {
                    println!("Failed to open vault: {}", e);
                }
            }
        }
        _ => {
            println!("Usage: cargo run -- [save|load]");
        }
    }
    Ok(())
}