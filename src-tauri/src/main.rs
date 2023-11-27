use std::{collections::HashMap, vec, fs::File, io::Read};

use dfile::DisplayDirectory;

use walkdir::{WalkDir};
use std::fs;
use base64;
mod service;

use crate::{dfile::DFile, service::f_factory::FileReaderImpl};
#[path = "model/dfile.rs"] mod dfile;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![list_files, get_file_content])
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
                    println!("{:?}",e.path().extension());

                    let os_str = e.path().file_name().unwrap_or_default();
                    let os_ext = e.path().extension().map_or_else(|| "".into(), |ext| ext.to_string_lossy());
                    
                    let mut p: String = String::new();

                    if let Some(parent) = e.path().parent().and_then(|p| p.file_name()) {
                        println!("Parent directory name: {:?}", parent);
                        p = parent.to_string_lossy().to_string();
                    } else {
                        println!("No parent directory found");
                    };

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
        res.push(DisplayDirectory {
            name: key,
            files: value,
        });
    }

    Ok(res)
}

#[tauri::command]
fn get_file_content(fullPath: String) -> String {
    let path = fullPath.replace("\\", "\\\\");
    let err: String = String::new();

    println!("--reading---");
    println!("{}", path);
    // Read the file content

    if path.contains(".pdf") {
        if let Ok(mut file) = File::open(path) {

        // Create a buffer to read the file in chunks
        let mut buffer = Vec::new();
        if let Ok(res) = file.read_to_end(&mut buffer) {
            // Encode the binary content as base64
            let base64_encoded = base64::encode(&buffer);

            // Return base64-encoded string
            base64_encoded
        } else {
            err
        }
        } else {
            err
        }
    } else if path.contains(".json") || path.contains(".txt") || path.contains(".xml") {
        if let Ok(content) = fs::read_to_string(path) {
            content
        }
        else {
            err
        }
    } else if path.contains(".doc") || path.contains(".docx") {
        println!("inside doc");
        let doc_file = service::f_factory::FileReader {
            path,
            ftype: service::f_factory::FileType::DOC,
        };
        match doc_file.read() {
            Ok(result) => println!("Read result: {}", result),
            Err(err) => eprintln!("Error reading document: {}", err),
        }
        "".to_string()
    }
     else {

        if let Ok(content) = fs::read(path) {
            // Encode binary content as base64
            let base64_encoded = base64::encode(&content);
            base64_encoded
        }
        else {
            err
        }
    }
}
