use zip::ZipWriter;
use rand::Rng;

pub fn crack_password(entry: &ZipWriter, max_attempts: u32) -> Option<String> {
    let mut attempts = 0;
    let mut password = String::new();
    while attempts < max_attempts {
        // Generate random password
        password = generate_random_password();
        // Attempt decrypting entry with the generated password
        if entry.decrypt(password.as_bytes()).is_ok() {
            return Some(password);
        }
        attempts += 1;
    }
    None
}

// function takes a ZipEntry object and a maximum number of attempts as input 
// and returns an Option containing the cracked password if successful.
fn generate_random_password() -> String {
    // Generating random pass with cryptic secure random generator
    let mut rng = rand::thread_rng();
    let pass_length = rng.gen_range(8..16);
    let mut password = String::new();
    for _ in 0..pass_length {
        password.push(rng.gen_range(0..256) as u8 as char);
    }
    password
}