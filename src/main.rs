use std::{env, error::Error, process};

fn main() {
    // Create a CSV parser that takes file path from the first command-line argument
    let raw_path = match get_arg(1) {
        Ok(path) => path,
        Err(e) => {
            println!("Error getting argument: {}", e);
            process::exit(1);
        }
    };
    let absolute_path = match get_file_path(&raw_path) {
        Ok(absolute_path) => absolute_path,
        Err(e) => {
            println!("Error getting file path: {}", e);
            process::exit(1);
        }
    };
    println!("{}", absolute_path);

    if let Err(e) = run(&absolute_path) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

/// Returns the absolute path to the given file.
fn get_file_path(raw_path: &str) -> Result<String, Box<dyn Error>> {
    let full_path = std::path::Path::new(&raw_path);
    if !full_path.exists() {
        return Err(From::from("File does not exist"));
    }
    let absolute_path = full_path.canonicalize()?;
    Ok(absolute_path.display().to_string())
}

/// Returns the nth positional argument sent to this process. If there are not
/// enough positional arguments, then this returns an error.
fn get_arg(idx: usize) -> Result<String, Box<dyn Error>> {
    match env::args_os().nth(idx) {
        None => Err(From::from(format!(
            "expected {} arguments, but got none",
            idx
        ))),
        Some(arg) => arg
            .to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| From::from("argument is not valid UTF-8")),
    }
}

fn run(absolute_path: &str) -> Result<(), Box<dyn Error>> {
    let mut parser = csv::Reader::from_path(absolute_path).unwrap();

    // Iterate over each record
    for record in parser.records() {
        match record {
            Ok(record) => println!("{:?}", record),
            Err(e) => return Err(From::from(e)),
        }
    }
    Ok(())
}
