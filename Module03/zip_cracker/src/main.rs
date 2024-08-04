use std::env;
use zip::ZipArchive;

mod reader;
mod extractor;
mod cracker;

#[tokio::main]
async fn main() {
    if let Some(filename) = env::args().nth(1) {
        if let Ok(archive) = reader::read_zip(&filename) {
            if let Ok(password_protected_entries) = extractor::extract_password(&archive) {
                for entry in password_protected_entries {
                    match cracker::crack_password(&entry, 1000) {
                        Ok(Some(pass)) => println!("Password cracked: {}: {}", entry.name(), pass),
                        Ok(None) => println!("Failed to crack password of the entry"),
                        Err(err) => eprintln!("Error cracking password: {}", err),
                    }
                }
                return;
            }
        }
    }
    eprintln!("Failed to process the zip file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_main() {
        let file = File::open("test.zip").unwrap();
        let archive = ZipArchive::new(file).unwrap();
        let password_protected_entries = extract_password(&archive);
        assert_eq!(password_protected_entries.len(), 1);
    }
}