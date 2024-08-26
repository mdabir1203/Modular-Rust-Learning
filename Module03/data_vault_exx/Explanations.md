# Rust Vault Application Documentation

## Overview
This Rust application provides a secure vault system for storing key-value pairs, protected by a hashed password. It includes features for password hashing, serialization, error handling, file I/O, command-line argument parsing, progress bar display, and terminal input handling.

## Features

### Password Hashing and Verification
- **Crate**: `argon2`
- **Usage**:
  - `PasswordHash`
  - `PasswordHasher`
  - `PasswordVerifier`
  - `SaltString`
- **Description**: Provides functionality for hashing and verifying passwords using the Argon2 algorithm.

### Serialization and Deserialization
- **Crate**: `serde`
- **Usage**:
  - `Serialize`
  - `Deserialize`
- **Description**: Enables serialization and deserialization of the `Vault` struct to and from JSON format.

### Error Handling
- **Crate**: `thiserror`
- **Usage**:
  - Custom error type `VaultError`
  - Implements `std::error::Error` for `PasswordHashError`
- **Description**: Defines custom error types for handling various error scenarios in the application.

### File I/O
- **Module**: `std::fs::File`
- **Usage**:
  - Reading from and writing to files
  - `std::io::{self, Read, Write}`
- **Description**: Manages file operations for saving and loading the vault data.

### Command Line Arguments
- **Module**: `std::env::args`
- **Description**: Handles command-line arguments to determine whether to save or load the vault.

### Progress Bar
- **Crate**: `indicatif`
- **Usage**:
  - `ProgressBar`
  - `ProgressStyle`
- **Description**: Displays a progress bar during save and load operations to indicate progress.

### Terminal Input Handling
- **Crate**: `termion`
- **Usage**:
  - `termion::input::TermRead`
  - `termion::raw::IntoRawMode`
- **Description**: Manages terminal input, including hiding password input for security.

## Data Structures

### Vault
- **Type**: `struct`
- **Fields**:
  - `data: HashMap<String, String>`
- **Methods**:
  - `new() -> Self`: Creates a new `Vault`.
  - `add_entry(&mut self, key: String, value: String)`: Adds a key-value pair to the vault.
  - `get_entry(&self, key: &str) -> Option<&String>`: Retrieves a value by key.
  - `save_to_file(&self, filename: &str, password: &str) -> Result<()>`: Serializes the vault and saves it to a file with a hashed password.
  - `load_from_file(filename: &str, password: &str) -> Result<Self>`: Loads and deserializes the vault from a file after verifying the password.

### PasswordHashError
- **Type**: `struct`
- **Fields**:
  - `0: argon2::password_hash::Error`
- **Implements**:
  - `fmt::Display`
  - `std::error::Error`

### VaultError
- **Type**: `enum`
- **Variants**:
  - `Io(#[from] std::io::Error)`: For I/O errors.
  - `Serialization(#[from] serde_json::Error)`: For serialization errors.
  - `Argon2(String)`: For Argon2-specific errors.
  - `PasswordHash(#[from] PasswordHashError)`: For password hash errors.
  - `InvalidPassword`: For invalid password errors.
  - `InvalidFileFormat`: For invalid file format errors.

## Functions

### is_password_strong(password: &str) -> bool
- **Description**: Checks if a password meets certain strength criteria (length, digit, uppercase letter, special character).
- **Parameters**:
  - `password: &str`: The password to check.
- **Returns**: `bool` indicating whether the password is strong.

### main() -> std::result::Result<(), Box<dyn std::error::Error>>
- **Description**: The entry point of the program. Handles command-line arguments to either save or load the vault. Manages user input for passwords and displays progress bars.
- **Returns**: `std::result::Result<(), Box<dyn std::error::Error>>`