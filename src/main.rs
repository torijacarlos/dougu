use std::{env::current_dir, error::Error, ffi::OsString, fs, path::PathBuf};

fn execute_in_directory(
    directory: &str,
    f: fn(&std::path::PathBuf) -> Option<String>,
) -> Result<Vec<Option<String>>, Box<dyn Error>> {
    let modified: Vec<Option<String>> = fs::read_dir(directory)?
        .map(|file| match file {
            Ok(value) => f(&value.path()),
            Err(..) => None,
        })
        .collect();

    return Ok(modified);
}

fn format_jfif_to_jpeg(path: &PathBuf) -> Option<String> {
    // Changes jfif files to jpg
    if let Some(file) = path.extension() {
        if let Some(value) = file.to_str() {
            if value == "jfif" {
                let mut new_path = path.to_owned();
                new_path.set_extension("jpg");

                return match fs::rename(path, new_path) {
                    Ok(..) => Some(String::from(path.to_str().unwrap().to_owned())),
                    Err(..) => None,
                };
            }
        }
    }
    return None;
}

fn main() {
    let current_directory = current_dir();

    let directory: OsString = match current_directory {
        Ok(path_buf) => path_buf.into_os_string(),
        Err(error) => panic!("{:?}", error),
    };

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

            let sth = execute_in_directory(directory.to_str().unwrap(), format_jfif_to_jpeg);
            match sth {
                Ok(result) => {
                    for r in result {
                        if let Some(value) = r {
                            println!("File {} renamed", value);
                        }
                    }
                }
                Err(..) => println!("Failed to change jfif extension on images"),
            }
        }
        _ => {
            println!("No valid command specified")
        }
    }
}
