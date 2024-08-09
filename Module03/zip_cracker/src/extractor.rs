use zip::ZipArchive;

pub fn extract_password(archive: &ZipArchive<File>) -> Vec<zip::ZipEntry> {
    let mut pass_entry = Vec::new();
    for entry in archive.entries() {
        if entry.is_password_protected() {
            password_protected_entries.push(entry);
        }
    }
    password_protected_entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_extract_password() {
        let file = File::open("test.zip").unwrap();
        let archive = ZipArchive::new(file).unwrap();
        let password_protected_entries = extract_password(&archive);
        assert_eq!(password_protected_entries.len(), 1);
    }
}