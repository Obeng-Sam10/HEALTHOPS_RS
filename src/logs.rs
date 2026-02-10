use std::fs;

pub fn scan_logs() {
    println!("Scanning logs...");

    let file_path = "test.log";

    let content = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(_) => {
            println!("Could not read logs file.");
            return;
        }
    };

    let total_lines = content.lines().count();
    let error_count = content.matches("ERROR").count();

    println!("Logs file: {}", file_path);
    println!("Total lines: {}", total_lines);
    println!("ERROR lines: {}", error_count);

    if error_count > 5 {
        println!("⚠️ Warning: High number of errors detected!");
    }
}
