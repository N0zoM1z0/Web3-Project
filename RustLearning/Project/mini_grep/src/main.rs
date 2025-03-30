// a mini grep implemented in Rust

use std::env;
use std::fs;
use std::error::Error;
use std::process;
use std::path::Path;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        let program_name = Path::new(&args[0]).file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or_else(|| args[0].as_str());        
        eprintln!("Usage: {} <pattern> <file>", program_name);
        process::exit(1);
    }
    let pattern = &args[1];
    let filename = &args[2];

    println!("Searching for {} in {}", pattern, filename);

    if let Err(e) = run(pattern, filename) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run(pattern: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    for line in contents.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}