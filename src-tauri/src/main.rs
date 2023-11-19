use walkdir::{WalkDir};


use crate::dfile::DFile;
#[path = "model/dfile.rs"] mod dfile;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![list_files])
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

    let allowed_files = vec![".txt", ".json",".pdf", ".docx"];
    for ext in allowed_files {
        if path.contains(ext) {
            return true;
        }
    }
    return false;
}

#[tauri::command]
async fn list_files(folderPath: String) -> Result<Vec<DFile>, String> {
    
    println!("{}", folderPath);
    println!("--- printing list ---");

    let entries: Vec<DFile> = WalkDir::new(folderPath)
        .into_iter()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.file_type().is_file() && is_valid_file(e.path().to_string_lossy().to_string()) {
                    println!("{:?}",e.path().extension());

                    let os_str = e.path().file_name().unwrap_or_default();
                    let os_ext = e.path().extension().map_or_else(|| "".into(), |ext| ext.to_string_lossy());

                    let entry = dfile::DFile {
                        name: os_str.to_string_lossy().to_string(),
                        extension: os_ext.to_string(),
                        url: e.path().to_string_lossy().to_string(),
                    };

                    Some(entry)
                } else {
                    None
                }
            })
        })
        .collect();

    Ok(entries)
}

// #[tauri::command]
// fn select_folder() -> String {
//     // Initialize GTK
//     gtk::init().expect("Failed to initialize GTK.");

//     // Create a GTK file chooser dialog
//     let dialog = FileChooserDialog::new(
//         Some("Select Folder"),
//         Some(&tauri::Window::new().inner()),
//         FileChooserAction::SelectFolder,
//     );
//     dialog.add_button("Cancel", gtk::ResponseType::Cancel);
//     dialog.add_button("Open", gtk::ResponseType::Accept);

//     // Create a file filter to show only directories
//     let filter = FileFilter::new();
//     filter.add_pattern("*");
//     filter.set_name(Some("All Directories"));
//     dialog.add_filter(&filter);

//     // Show the dialog and handle the result
//     let result = if dialog.run() == gtk::ResponseType::Accept.into() {
//         let filename = dialog.get_filename().unwrap();
//         Some(filename.to_string_lossy().to_string())
//     } else {
//         None
//     };

//     // Cleanup GTK
//     dialog.destroy();
//     gtk::main_quit();

//     // Convert the result to a string
//     result.unwrap_or_else(String::new)

// }
