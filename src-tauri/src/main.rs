use std::mem::replace;
use std::time::UNIX_EPOCH;
use std::{collections::HashMap, vec, fs::File, io::Read};
use std::sync::Mutex;
use lazy_static::lazy_static;

use dfile::{DisplayDirectory, DFile};
#[path = "model/dfile.rs"] mod dfile;

use service::search_engine::SearchEngine;
use walkdir::WalkDir;
use std::fs::{self, metadata};
use base64;
mod service;

use crate::service::f_factory::{FileReader, FileType};
use crate::service::util::is_valid_file;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

lazy_static!{
    static ref SEARCH_ENGINE :Mutex<SearchEngine> = Mutex::new(SearchEngine::new().unwrap());
    static ref FILE_READER :Mutex<FileReader> = Mutex::new(FileReader::new());
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![list_files, get_file_content, search_content])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn list_files(folderPath: String) -> Result<Vec<DisplayDirectory>, String> {
    
    println!("{}", folderPath);
    println!("--- printing list ---");
    let mut entriesMap: HashMap<String, Vec<DFile>> = HashMap::new();

    WalkDir::new(folderPath).into_iter().for_each(|entry| {
        match entry {
            Ok(e) => {
                if e.file_type().is_file() && is_valid_file(e.path().to_string_lossy().to_string()) {
                    let os_str = e.path().file_name().unwrap_or_default();
                    let os_ext = e.path().extension().map_or_else(|| "".into(), |ext| ext.to_string_lossy());

                    let parent = e.path().parent().and_then(|p| p.file_name()).unwrap();
                    let p: String = parent.to_string_lossy().to_string();

                    let last_modified = metadata(e.path()).unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();

                    let entry = DFile {
                        name: os_str.to_string_lossy().to_string(),
                        extension: os_ext.to_string(),
                        url: e.path().to_string_lossy().to_string(),
                    };
                    
                    if entriesMap.contains_key(&p) {
                        entriesMap.entry(p).or_insert_with(Vec::new).push(entry);
                    } else {
                        entriesMap.insert(p, vec![entry]);
                    }
                    let url = e.path().to_string_lossy().to_string();
                    println!("url: {}", url);
                    if url.contains(".docx") {
                        println!("found a doc");
                        read_docs(url, last_modified);
                    }

                }
            }
            Err(err) => {
                // Handle errors in a specific way, or ignore them if desired
                println!("Error: {:?}", err);
            }
        }
        
    });

    let mut res = Vec::new();
    for (key, value) in entriesMap {
        res.push(DisplayDirectory { name: key, files: value, });
    }
    Ok(res)
}

#[tauri::command]
async fn search_content(query: String) -> Vec<String> {
    println!("{}", query);
    {
        let mut search_engine = SEARCH_ENGINE.lock().unwrap();
        match search_engine.search(query.as_str()) {
            Ok(msg) => return msg,
            Err(err) => {
                println!("{:?}",err);
            }
        }

    }
    Vec::new()
}

#[tauri::command]
async fn get_file_content(path: String) -> String {
    let err: String = String::new();

    println!("--reading---");
    println!("{}", path);
    // Read the file content

    if path.contains(".pdf") {
        if let Ok(mut file) = File::open(path) {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer);
            let base64_encoded = base64::encode(&buffer);
            return base64_encoded;
        }
    } else if path.contains(".json") || path.contains(".txt") || path.contains(".xml") {
        if let Ok(content) = fs::read_to_string(path) {
            return content;
        }
    } else if path.contains(".doc") || path.contains(".docx") {
        let last_modified = metadata(path.clone()).unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let npath = path.replace("\\", "\\\\");
        read_docs(npath, last_modified);
        return "".to_string();
    }
     else {
        if let Ok(content) = fs::read(path) {
            let base64_encoded = base64::encode(&content);
            return base64_encoded;
        }
    }
    return err;
}


fn read_docs(path: String, last_modified: u64) {
    {
        println!("inside doc");
        let doc_file = FILE_READER.lock().unwrap();
        let mut search_engine = SEARCH_ENGINE.lock().unwrap();
    
        match doc_file.read(FileType::DOC, path, last_modified, search_engine) {
            Ok(result) => println!("Read result: {}", result),
            Err(err) => eprintln!("Error reading document: {}", err),
        }
    }
}
