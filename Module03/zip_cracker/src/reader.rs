pub mod reader {
    use std::fs::File;
    use std::io::Read;
    use zip::ZipArchive;

    pub fn read_zip(filename: &str) -> Result<ZipArchive<std::fs::File>, zip::result::ZipError> {
        let file = File::open(filename)?;
        let archive = ZipArchive::new(file)?;
        Ok(archive)
    }
}