use std::{path::PathBuf, fs::File};
use std::io::{Read, Result};

use docx_rs::*;
use serde::Serialize;


pub(crate) struct Doc {
    pub(crate) path: String,
}

pub(crate) trait DocReader {
    fn read(&self) -> Result<String>;
}

impl DocReader for Doc {
    fn read(&self) -> Result<String> {
        let file_contents_result = read_to_vec(&PathBuf::from(&self.path));
        if let Err(err) = file_contents_result {
            return Ok(err.to_string())
        }
        println!("file read from path");

        let file_contents = file_contents_result.unwrap();  // Safe to unwrap after the check

        println!("reading docx from file");
        let original_docx_result = read_docx(&file_contents);
        if let Err(err) = original_docx_result {
            return Ok(err.to_string());
        }

        let original_docx = original_docx_result.unwrap();  // Safe to unwrap after the check

        println!("read docx");

        let d: Document = original_docx.document;
        for document_child in d.children {
            // Now you can serialize your struct using Serde's `to_string` function.
            let serialized = serde_json::to_string(&document_child);

            match serialized {
                Ok(json) => {
                    println!("Serialized JSON: {}", json);
                    // You can now use the serialized JSON as needed
                }
                Err(err) => eprintln!("Error serializing: {}", err),
            }
        }
        // Debug print the original_docx
        // dbg!(&original_docx);

        Ok("SUCCESS".to_string())
    }
}

fn read_to_vec(file_name: &PathBuf) -> Result<Vec<u8>> {

    let mut f = File::open(file_name)?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}