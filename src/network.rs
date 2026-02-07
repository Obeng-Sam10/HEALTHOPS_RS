use std::process::Command;

pub fn scan_network() {
    println!("Scanning network connections...");

    // Run system command
    let output = Command::new("netstat")
        .arg("-an")
        .output();

    let output = match output {
        Ok(o) => o,
        Err(_) => {
            println!("Failed to run netstat command.");
            return;
        }
    };

    // Convert output to text
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Count connections (each line ≈ one connection)
    let connection_count = stdout.lines().count();

    println!("Total network connections: {}", connection_count);

    // Simple alert rule (shortcut & approximation)
    if connection_count > 20 {
        println!("⚠️ Warning: Unusually high number of connections detected!");
    }
}
