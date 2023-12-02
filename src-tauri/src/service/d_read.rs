use std::sync::MutexGuard;
use std::{path::PathBuf, fs::File};
use std::io::{Read, Result};

use docx_rs::*;
use serde_json::Value;

use super::search_engine::SearchEngine;

pub(crate) struct DocReader {
}

impl DocReader {
    pub(crate) fn new() -> DocReader {
        println!("Doc Reader initialized");
        DocReader {}
    }

    pub(crate) fn read(&self, file_name: String, path :String, last_modified :u64, mut search_engine :MutexGuard<'_, SearchEngine>) -> Result<String> {
        let file_contents_result = read_to_vec(&PathBuf::from(&path));
        if let Err(err) = file_contents_result {
            return Ok(err.to_string())
        }
        let file_contents = file_contents_result.unwrap();  // Safe to unwrap after the check

        println!("reading docx from file");
        let original_docx_result = read_docx(&file_contents);
        if let Err(err) = original_docx_result {
            return Ok(err.to_string());
        }

        let original_docx = original_docx_result.unwrap();  // Safe to unwrap after the check
        let ans = parse_docx(original_docx)?;
        
        println!("file read successfully");
        search_engine.append(file_name, path, ans.join(r" \"), last_modified);

        Ok("SUCCESS".to_string())
    }
}


fn parse_docx(original_docx: Docx) -> Result<Vec<String>> {
    let data: Value = serde_json::from_str(&original_docx.json())?;
    let mut result = Vec::new();

    if let Some(children) = data["document"]["children"].as_array() {
        for node in children {
            read_children(node, &mut result);
        }
    }

    Ok(result)
}

fn read_children(node: &Value, result: &mut Vec<String>) {
    if let Some(children) = node["data"]["children"].as_array() {
        for child in children {
            if child["type"] != "text" {
                read_children(child, result);
            } else {
                result.push(child["data"]["text"].as_str().unwrap_or_default().to_string());
            }
        }
    }
}

fn read_to_vec(file_name: &PathBuf) -> Result<Vec<u8>> {
    let mut f = File::open(file_name)?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}