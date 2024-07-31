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