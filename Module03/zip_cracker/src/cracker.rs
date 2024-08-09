use zip::ZipWriter;
use rand::Rng;

pub fn crack_password(entry: &ZipWriter, max_attempts: u32) -> Option<String> {
    for _ in 0..max_attempts {
        let password = generate_random_password();
        if entry.decrypt(password.as_bytes()).is_ok() {
            return Some(password);
        }
    }
    None
}

fn generate_random_password() -> String {
    let mut rng = rand::thread_rng();
    let pass_length = rng.gen_range(8..16);
    (0..pass_length)
        .map(|_| rng.gen_range(0..256) as u8 as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_password() {
        let password = generate_random_password();
        assert_eq!(password.len() >= 8 && password.len() <= 16, true);
    }
}