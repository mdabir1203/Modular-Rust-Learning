use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use crate::errors::{Result, VaultError, PasswordHashError};


#[derive(Serialize, Deserialize)]
pub struct Vault {
    data: HashMap<String, String>,
}

impl Vault {
    pub fn new() -> Self {
        Vault {
            data: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get_entry(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn save_to_file(&self, filename: &str, password: &str) -> Result<()> {
        let serialized = serde_json::to_string(&self)?;
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| VaultError::Argon2(e.to_string()))?
            .to_string();
        let mut file = File::create(filename)?;
        file.write_all(password_hash.as_bytes())?;
        file.write_all(b"\n")?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(filename: &str, password: &str) -> Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut lines = contents.lines();
        let password_hash = lines.next().ok_or(VaultError::InvalidFileFormat)?;
        let serialized = lines.collect::<Vec<&str>>().join("\n");

        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|e| VaultError::PasswordHash(PasswordHashError(e)))?;

        if Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            let vault: Vault = serde_json::from_str(&serialized)?;
            Ok(vault)
        } else {
            Err(VaultError::InvalidPassword)
        }
    }
}