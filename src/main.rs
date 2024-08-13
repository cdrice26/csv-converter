use std::{error::Error, process};

fn main() {
    // Create a CSV parser that takes file path from the first command-line argument 
    let raw_path = std::env::args().nth(1).expect("Please provide an input file path");
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

fn get_file_path(raw_path: &str) -> Result<String, Box<dyn Error>> {
    let full_path = std::path::Path::new(&raw_path);
    if !full_path.exists() {
        return Err(From::from("File does not exist"));
    }
    let absolute_path = full_path.canonicalize()?;
    Ok(absolute_path.display().to_string())
}

fn run(absolute_path: &str) -> Result<(), Box<dyn Error>> {
    let mut parser = csv::Reader::from_path(absolute_path).unwrap();

    // Iterate over each record
    for record in parser.records() {
        match record {
            Ok(record) => println!("{:?}", record),
            Err(e) => return Err(From::from(e))
        }
    }
    Ok(())
}
