// Import necessary libraries and modules
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng; // For generating random salts
use serde::{Deserialize, Serialize}; // For JSON serialization/deserialization
use std::collections::HashMap; // For key-value storage
use std::fs::File; // For file handling
use std::io::{Read, Write}; // For reading from and writing to files
use crate::errors::{Result, VaultError, PasswordHashError}; // Custom error types

// Define the Vault struct to hold data
#[derive(Serialize, Deserialize)]
pub struct Vault {
    data: HashMap<String, String>, // A HashMap to store key-value pairs
}

impl Vault {
    // Constructor to create a new Vault instance
    pub fn new() -> Self {
        Vault {
            data: HashMap::new(), // Initialize with an empty HashMap
        }
    }

    // Method to add a new entry to the vault
    pub fn add_entry(&mut self, key: String, value: String) {
        self.data.insert(key, value); // Insert the key-value pair into the HashMap
    }

    // Method to retrieve an entry by key
    pub fn get_entry(&self, key: &str) -> Option<&String> {
        self.data.get(key) // Return the value associated with the key, if it exists
    }

    // Method to save the vault's data to a file
    pub fn save_to_file(&self, filename: &str, password: &str) -> Result<()> {
        let serialized = serde_json::to_string(&self)?; // Serialize the vault to JSON
        let salt = SaltString::generate(&mut OsRng); // Generate a random salt
        let argon2 = Argon2::default(); // Create an Argon2 instance
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt) // Hash the password with the salt
            .map_err(|e| VaultError::Argon2(e.to_string()))? // Handle errors
            .to_string(); // Convert the hash to a string
        let mut file = File::create(filename)?; // Create or open the file
        file.write_all(password_hash.as_bytes())?; // Write the password hash to the file
        file.write_all(b"\n")?; // Write a newline
        file.write_all(serialized.as_bytes())?; // Write the serialized vault data
        Ok(()) // Return success
    }

    // Method to load the vault's data from a file
    pub fn load_from_file(filename: &str, password: &str) -> Result<Self> {
        let mut file = File::open(filename)?; // Open the file
        let mut contents = String::new(); // Create a string to hold the file contents
        file.read_to_string(&mut contents)?; // Read the file contents into the string

        let mut lines = contents.lines(); // Split the contents into lines
        let password_hash = lines.next().ok_or(VaultError::InvalidFileFormat)?; // Get the first line as the password hash
        let serialized = lines.collect::<Vec<&str>>().join("\n"); // Join the remaining lines into a single string

        let parsed_hash = PasswordHash::new(password_hash) // Parse the password hash
            .map_err(|e| VaultError::PasswordHash(PasswordHashError(e)))?; // Handle errors

        // Verify the provided password against the stored hash
        if Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            let vault: Vault = serde_json::from_str(&serialized)?; // Deserialize the vault data
            Ok(vault) // Return the vault instance
        } else {
            Err(VaultError::InvalidPassword) // Return an error if the password is invalid
        }
    }
}