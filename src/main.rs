use std::{env::current_dir, error::Error, fs, path::PathBuf};

fn execute_in_directory(
    directory: &str,
    filter: fn(&PathBuf) -> bool,
    f: fn(&PathBuf) -> Option<String>,
) -> Result<Vec<Option<String>>, Box<dyn Error>> {
    let modified: Vec<Option<String>> = fs::read_dir(directory)?
        .filter(|file| match file {
            Ok(value) => filter(&value.path()),
            Err(..) => false,
        })
        .map(|file| match file {
            Ok(value) => f(&value.path()),
            Err(..) => None,
        })
        .collect();

    return Ok(modified);
}

fn is_jfif(path: &PathBuf) -> bool {
    if let Some(file) = path.extension() {
        if let Some(value) = file.to_str() {
            return value == "jfif";
        }
    }
    return false;
}

fn set_extension_to_jpg(path: &PathBuf) -> Option<String> {
    let mut new_path = path.to_owned();
    new_path.set_extension("jpg");
    return match fs::rename(path, new_path) {
        Ok(..) => Some(String::from(path.to_str().unwrap().to_owned())),
        Err(..) => None,
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let directory = current_dir()?;

    let command: String = if let Some(value) = std::env::args().nth(1) {
        String::from(value)
    } else {
        String::from("")
    };

    match command.as_str() {
        "images" => {
            println!(
                "Setting naming convention in current directory ({})",
                directory.to_str().unwrap()
            );

            if let Some(dir) = directory.to_str() {
                 let modified = execute_in_directory(dir, is_jfif, set_extension_to_jpg)?;
                 for m in modified {
                     if let Some(result) = m {
                         println!("File {} changed successfully", result);
                     }
                 }
            }
        }
        _ => {
            println!("No valid command specified");
        }
    }
    return Ok(());
}
