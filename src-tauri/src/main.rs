use std::{collections::HashMap, vec, fs::File, io::Read};
use std::sync::Mutex;
use lazy_static::lazy_static;

use dfile::DisplayDirectory;

use service::search_engine::SearchEngine;
use walkdir::WalkDir;
use std::fs;
use base64;
mod service;

use crate::service::f_factory::{FileReader, FileType};
use crate::dfile::DFile;
#[path = "model/dfile.rs"] mod dfile;

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


fn is_valid_file(path: String) -> bool {
    let ignore_files= vec![".git", "node_modules"];
    for ext in ignore_files {
        if path.contains(ext) {
            return false;
        }
    }

    let allowed_files = vec![".txt", ".json",".pdf", ".docx", ".png", ".jpeg", ".jpg", ".xml"];
    for ext in allowed_files {
        if path.contains(ext) {
            return true;
        }
    }
    return false;
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
                    
                    let mut p: String = String::new();

                    if let Some(parent) = e.path().parent().and_then(|p| p.file_name()) {
                        p = parent.to_string_lossy().to_string();
                    }

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
                        read_docs(url);
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
        read_docs(path);
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


fn read_docs(path: String) {
    {
        println!("inside doc");
        let doc_file = FILE_READER.lock().unwrap();
        let mut search_engine = SEARCH_ENGINE.lock().unwrap();
    
        match doc_file.read(FileType::DOC, path, search_engine) {
            Ok(result) => println!("Read result: {}", result),
            Err(err) => eprintln!("Error reading document: {}", err),
        }
    }
}
