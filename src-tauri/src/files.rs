use std::fs;
use std::io;

pub(crate) fn get_files_in_dir(dir: &str) -> io::Result<Vec<String>> {
    let mut file_list = Vec::new();

    for entry_result in fs::read_dir(dir)? {
        let entry = entry_result?;
        let path = entry.path();

        if path.is_file() {
            if path.extension().and_then(|e| e.to_str()) == Some("lua") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    file_list.push(stem.to_string());
                }
            }
        }
    }

    Ok(file_list)
}

pub(crate) fn get_file_content(path: String) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| String::new())
}
