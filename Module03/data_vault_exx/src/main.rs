use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use thiserror::Error;
use std::fmt;
use std::env;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;
use std::path::Path;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Debug)]
struct PasswordHashError(argon2::password_hash::Error);

impl fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Password hash error: {}", self.0)
    }
}

impl std::error::Error for PasswordHashError {}

#[derive(Error, Debug)]
enum VaultError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Argon2 error: {0}")]
    Argon2(String),
    #[error("Password hash error: {0}")]
    PasswordHash(#[from] PasswordHashError),
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Invalid file format")]
    InvalidFileFormat,
}

type Result<T> = std::result::Result<T, VaultError>;

#[derive(Serialize, Deserialize)]
struct Vault {
    data: HashMap<String, String>,
}

impl Vault {
    fn new() -> Self {
        Vault {
            data: HashMap::new(),
        }
    }

    fn add_entry(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn get_entry(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn save_to_file(&self, filename: &str, password: &str) -> Result<()> {
        let serialized = serde_json::to_string(&self)?;
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| VaultError::Argon2(e.to_string()))?
            .to_string();
        let mut file = File::create(filename)?;
        file.write_all(password_hash.as_bytes())?;
        file.write_all(b"\n")?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
        


    fn load_from_file(filename: &str, password: &str) -> Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let mut lines = contents.lines();
        let password_hash = lines.next().ok_or(VaultError::InvalidFileFormat)?;
        let serialized = lines.collect::<Vec<&str>>().join("\n");
    
        let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| VaultError::PasswordHash(PasswordHashError(e)))?;
        
        if Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| VaultError::PasswordHash(PasswordHashError(e))).is_ok() {
            let vault: Vault = serde_json::from_str(&serialized)?;
            Ok(vault)
        } else {
            Err(VaultError::InvalidPassword)
        }
    }

}


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let operation = args.get(1).map(String::as_str);

    match operation {
        Some("save") => {
            
            let vault_file = "vault.dat";
            if Path::new(vault_file).exists(){
                println!("Password saved already.");
                return Ok(());
            }
            let mut vault = Vault::new();
            vault.add_entry("email".to_string(), "uknowwho12@gmail.com".to_string());
            vault.add_entry("api_key".to_string(), "542342".to_string());

            print!("Enter a password to secure the vault: ");
            io::stdout().flush()?;


            // Enable raw mode to hide input
            let mut stdin = io::stdin();
            let mut stdout = io::stdout().into_raw_mode()?;
            let password = stdin.read_passwd(&mut stdout)?.unwrap_or_default();
            let password = password.trim();

            // Disable raw mode
            write!(stdout, "\n")?;
            stdout.suspend_raw_mode()?;

            let pb = ProgressBar::new(100);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"));

            for i in 0..100 {
                pb.set_position(i + 1);
                thread::sleep(Duration::from_millis(20));
            }

            vault.save_to_file("vault.dat", password).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            pb.finish_with_message("Vault secured successfully");
        }
        Some("load") => {

            let vault_file = "vault.dat";
            if !Path::new(vault_file).exists(){
                println!("Vault doesn't exist. Run the save command.");
                return Ok(());
            }
            print!("Enter the password to open the vault: ");
            io::stdout().flush()?;

            // Enable raw mode to hide input
            let mut stdin = io::stdin();
            let mut stdout = io::stdout().into_raw_mode()?;
            let password = stdin.read_passwd(&mut stdout)?.unwrap_or_default();
            let password = password.trim();

            // Disable raw mode
            write!(stdout, "\n")?;
            stdout.suspend_raw_mode()?;

            let pb = ProgressBar::new(100);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"));

            for i in 0..100 {
                pb.set_position(i + 1);
                thread::sleep(Duration::from_millis(20));
            }

            match Vault::load_from_file(vault_file, password) {
                Ok(loaded_vault) => {
                    pb.finish_with_message("Vault loaded successfully");
                    if let Some(email) = loaded_vault.get_entry("email") {
                        println!("Email: {}", email);
                    }
                    if let Some(api_key) = loaded_vault.get_entry("api_key") {
                        println!("API Key: {}", api_key);
                    }
                }
                Err(VaultError::InvalidPassword) => {
                    pb.finish_with_message("Failed to open vault");
                    println!("Invalid password. Access denied.");
                }
                Err(e) => {
                    pb.finish_with_message("Failed to open vault");
                    return Err(Box::new(e));
                }
            }
        }
        _ => {
            println!("Usage: cargo run -- [save|load]");
            return Ok(());
        }
    }
    Ok(())
}


