use csv::WriterBuilder;
use std::{env, error::Error, fs::File, process};

fn main() {
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

    let header = match get_arg(2) {
        Ok(header) => header,
        Err(e) => {
            println!("Error getting header: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = run(&absolute_path, &header) {
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

/// Returns the index of the given header in the given csv::StringRecord.
fn get_index_of_header(headers: &csv::StringRecord, header: &str) -> Option<usize> {
    headers.iter().position(|h| h == header)
}

/// Run the program
fn run(absolute_path: &str, header: &str) -> Result<(), Box<dyn Error>> {
    let mut parser = csv::Reader::from_path(absolute_path).unwrap();
    let mut records: Vec<csv::StringRecord> = Vec::new();

    // Read headers
    let headers = parser.headers()?.clone();

    // Get the index of the user-provided header
    let index = get_index_of_header(&headers, header);
    if index.is_none() {
        return Err(From::from(format!("Missing {} header", header)));
    }
    let header_index = index.unwrap();

    // Iterate over each record, building a vector
    for record in parser.records() {
        match record {
            Ok(record) => records.push(record),
            Err(e) => return Err(From::from(e)),
        }
    }

    // Create a new vector for modified records
    let modified_records: Vec<csv::StringRecord> = records
        .into_iter()
        .map(|record| {
            let mut new_record = record.clone(); // Clone the original record
            if let Some(_) = new_record.get(header_index) {
                // Replace the field with the modified value
                new_record = new_record
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        if i == header_index {
                            match f.parse::<f64>() {
                                Ok(f) => (f + 1.0).to_string(),
                                Err(_) => "ERROR".to_string(),
                            }
                        } else {
                            f.to_string()
                        }
                    })
                    .collect::<csv::StringRecord>();
            }
            new_record // Return the modified record
        })
        .collect();

    // Write the modified records back to the same file
    let file = File::create(absolute_path)?;
    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(file);

    // Write the headers
    wtr.write_record(&headers)?;

    // Write the modified records
    for record in modified_records {
        wtr.write_record(&record)?;
    }

    // Flush the writer to ensure all data is written
    wtr.flush()?;

    Ok(())
}
