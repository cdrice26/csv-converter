use assert_cmd::Command;
use std::fs::{read_to_string, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_csv_unit_conversion_cli() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("test.csv");

    // Write a sample CSV
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Distance,Note").unwrap();
    writeln!(file, "1.0,A").unwrap();
    writeln!(file, "2.0,B").unwrap();
    writeln!(file, "invalid,C").unwrap(); // Check error handling

    // Run your binary using assert_cmd
    Command::cargo_bin("csvc")
        .unwrap()
        .arg(file_path.to_str().unwrap())
        .arg("Distance")
        .arg("meters")
        .arg("feet")
        .assert()
        .success();

    // Check output file content
    let output = read_to_string(&file_path).unwrap();
    assert!(output.contains("3.2808")); // 1.0 m to ft
    assert!(output.contains("6.5616")); // 2.0 m to ft
    assert!(output.contains("ERROR")); // Non-numeric row
}
