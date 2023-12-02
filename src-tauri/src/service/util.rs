pub(crate) fn is_valid_file(path: String) -> bool {
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