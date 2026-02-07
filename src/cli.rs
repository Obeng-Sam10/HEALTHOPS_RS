use crate::log;
use crate::network;

pub fn run() {
    let args: Vec<String> = std::env::args().collect();

    // If user types nothing
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "scan" => handle_scan(&args),
        "help" => print_help(),
        _ => {
            println!("Unknown command.");
            print_help();
        }
    }
}

fn handle_scan(args: &Vec<String>) {
    if args.len() < 3 {
        println!("Please specify what to scan: logs | network");
        return;
    }

    match args[2].as_str() {
        "log" => log::scan_logs(),
        "network" => network::scan_network(),
        _ => println!("Unknown scan type."),
    }
}

fn print_help() {
    println!("HealthOps_rs – Healthcare Operations CLI");
    println!();
    println!("Usage:");
    println!("  healthops scan logs");
    println!("  healthops scan network");
    println!("  healthops help");
}
