use std::fs::File;
Read, Seek};
use zip::ZipArchive;


pub fn read_zip(filename: &str) -> Result<ZipArchive<File>, std::io::Error> {
    let file = File::open(filename)?;
    let mut archive = ZipArchive::new(file)?;
    Ok(archive)
}