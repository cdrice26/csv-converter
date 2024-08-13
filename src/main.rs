mod converter;
mod normalizer;

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

    let from_unit = match get_arg(3) {
        Ok(unit) => unit,
        Err(e) => {
            println!("Error getting unit: {}", e);
            process::exit(1);
        }
    };

    let to_unit = match get_arg(4) {
        Ok(unit) => unit,
        Err(e) => {
            println!("Error getting unit: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = run(&absolute_path, &header, &from_unit, &to_unit) {
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

/// Modify single column in a csv::StringRecord
fn modify_record<F, T, R>(
    record: &csv::StringRecord,
    header_index: usize,
    modifier: F,
) -> csv::StringRecord
where
    F: Fn(T) -> R,
    T: std::str::FromStr + std::fmt::Display,
    R: std::fmt::Display,
{
    let mut new_record = record.clone(); // Clone the original record
    if let Some(_) = new_record.get(header_index) {
        // Replace the field with the modified value
        let mut fields: Vec<String> = Vec::with_capacity(new_record.len());

        for (i, f) in new_record.iter().enumerate() {
            if i == header_index {
                match f.parse::<T>() {
                    Ok(value) => fields.push(modifier(value).to_string()),
                    Err(_) => fields.push("ERROR".to_string()),
                }
            } else {
                fields.push(f.to_string());
            }
        }

        new_record = csv::StringRecord::from(fields);
    }
    new_record
}

/// Modify the given column in the given vector of records, return a new vector with the modified column
fn modify_column<F>(
    header_index: usize,
    records: &Vec<csv::StringRecord>,
    modifier: F,
) -> Vec<csv::StringRecord>
where
    F: Fn(f64) -> f64,
{
    records
        .into_iter()
        .map(|record| modify_record(record, header_index, &modifier))
        .collect()
}

/// Edit the given column in the given csv file
fn run(
    absolute_path: &str,
    header: &str,
    from_unit: &str,
    to_unit: &str,
) -> Result<(), Box<dyn Error>> {
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
    let modified_records: Vec<csv::StringRecord> = modify_column(header_index, &records, |f| {
        match converter::convert(f, from_unit, to_unit) {
            Ok(f) => f,
            Err(e) => {
                println!("Error converting: {}", e);
                f
            }
        }
    });

    // Write the modified records back to the same file
    let file = File::create(absolute_path)?;
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);

    // Write the headers
    let new_headers = modify_record(&headers, header_index, |f: String| f);
    wtr.write_record(&new_headers)?;

    // Write the modified records
    for record in modified_records {
        wtr.write_record(&record)?;
    }

    // Flush the writer to ensure all data is written
    wtr.flush()?;

    Ok(())
}
