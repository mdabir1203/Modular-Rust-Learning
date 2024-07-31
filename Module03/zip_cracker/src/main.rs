use std::env;
use zip::ZipArchive;

mod reader;
mod extractor;
mod cracker;




#[tokio::main]
async fn main() {
    let filename = env::args().nth(1).expect("Please provide a filename path");
    let archive = match reader::read_zip(&filename) {
        Ok(archive) => archive,
        Err(err) => {
            eprintln!("Failed to read zip file: {}", err);
            return;
        }
    };
    let password_protected_entries = match extractor::extract_password(&archive) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to extract password protected entries: {}", err);
            return;
        }
    };
    for entry in password_protected_entries {
        match cracker::crack_password(&entry, 1000) {
        Ok(Some(pass)) => println!("Password cracked: {}: {}", entry.name(), pass),
        Ok(None) => println!("Failed to crack password of the entry"),
        Err(err) => eprintln!("Error cracking password: {}", err),
        }
    }
}