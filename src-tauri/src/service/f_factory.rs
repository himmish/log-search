use derive_more::Display;
use std::{io::Result, sync::{MutexGuard, Mutex}};

use lazy_static::lazy_static;

use crate::service::d_read::DocReader;
use super::search_engine::SearchEngine;


lazy_static!{
    static ref DOC_READER :Mutex<DocReader> = Mutex::new(DocReader::new());
}


#[derive(Display, PartialEq, Eq, Debug)]
pub(crate) enum FileType {
    PDF,
    DOC,
}

pub(crate) struct FileReader {
}

impl FileReader {
    
    pub(crate) fn new() -> FileReader {
        println!("File Reader initialized");
        FileReader {}
    }
    pub(crate) fn read(&self, ftype :FileType, path :String, mut search_engine: MutexGuard<'_, SearchEngine>) -> Result<String> {
        if ftype == FileType::DOC {
            println!("DOC Type Selected");
            let doc = DOC_READER.lock().unwrap();
            match doc.read(path, search_engine) {
                Ok(result) => return Ok(result),
                Err(err) => return Err(err),
            }
        }
        println!("Unsupported file type: {:?}", ftype);
        Ok("".to_string())
    }
}