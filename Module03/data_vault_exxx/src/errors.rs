use thiserror::Error;

#[derive(Debug)]
pub struct PasswordHashError(pub argon2::password_hash::Error);

impl std::fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Password hash error: {}", self.0)
    }
}

impl std::error::Error for PasswordHashError {}

#[derive(Error, Debug)]
pub enum VaultError {
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

pub type Result<T> = std::result::Result<T, VaultError>;