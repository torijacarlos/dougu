use std::{fs, path::PathBuf};

pub fn is_jfif(path: &PathBuf) -> bool {
    if let Some(file) = path.extension() {
        if let Some(value) = file.to_str() {
            return value == "jfif";
        }
    }
    return false;
}

pub fn set_extension_to_jpg(path: &PathBuf) -> Option<String> {
    let mut new_path = path.to_owned();
    new_path.set_extension("jpg");
    return match fs::rename(path, new_path) {
        Ok(..) => Some(String::from(path.to_str().unwrap().to_owned())),
        Err(..) => None,
    };
}

