use std::{env::current_dir, error::Error, ffi::OsString, fs};

fn format_jfif_to_jpeg(directory: &str) -> Result<Vec<Result<(), std::io::Error>>, Box<dyn Error>> {
    // Changes jfif files to jpg

    let mut paths = fs::read_dir(directory)?;
    let mut modified: Vec<Result<(), std::io::Error>> = Vec::new();

    loop {
        let file = paths.next();

        // TODO(torijacarlos): How can I better chain options?

        if let Some(value) = file {
            let path = value?.path();

            if let Some(file) = path.extension() {
                if let Some(value) = file.to_str() {
                    
                    if value == "jfif" {
                        let mut new_path = path.to_owned();
                        new_path.set_extension("jpg");

                        println!(
                            "File {} changed to {}",
                            path.to_str().unwrap(),
                            new_path.to_str().unwrap()
                        );
                        modified.push(fs::rename(path, new_path));
                    }
                }
            }
        } else {
            break;
        }
    }

    Ok(modified)
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

            match format_jfif_to_jpeg(directory.to_str().unwrap()) {
                Ok(..) => println!("Success changing jfif files to jpg"),
                Err(..) => println!("Failed to change jfif extension on images"),
            }
        }
        _ => {
            println!("No valid command specified")
        }
    }
}
