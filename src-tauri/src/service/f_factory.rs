use derive_more::Display;
use std::io::{Result};

use super::d_read::{Doc, DocReader};

#[derive(Display, PartialEq, Eq, Debug)]
pub(crate) enum FileType {
    PDF,
    DOC,
}


pub(crate) struct FileReader {
    pub(crate) path: String,
    pub(crate) ftype: FileType,
}

pub(crate) trait FileReaderImpl {
    fn read(&self) -> Result<String>;
}

impl FileReaderImpl for FileReader {
    fn read(&self) -> Result<String> {
        if self.ftype == FileType::DOC {
            println!("DOC Type Selected");
            let doc = Doc { path: self.path.clone() };
            match doc.read() {
                Ok(result) => println!("Read result: {}", result),
                Err(err) => eprintln!("Error reading document: {}", err),
            }
        } else {
            // Handle other file types if needed
            // For now, we'll just print a message
            println!("Unsupported file type: {:?}", self.ftype);
        }

        Ok("SUCCESS".to_string())
    }
}