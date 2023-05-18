use tempfile::NamedTempFile;
use std::io::Write;

pub struct FileService {}

impl FileService {

    pub fn create_tempfile(&self, content: String) -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(content.as_bytes()).expect("Unable to write to tempfile");

        return temp_file;
    }

}