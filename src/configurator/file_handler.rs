use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

pub struct FileHandler {
    file: File,
}

impl FileHandler {
    pub fn open(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        Ok(FileHandler { file })
    }

    pub fn read(&mut self) -> io::Result<String> {
        let mut content = String::new();
        self.file.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn write(file_path: &str, data: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}



