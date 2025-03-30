// A simple mini_curl implemented in Rust
use reqwest;
use std::process;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: mini_curl <URL>");
        process::exit(1);
    }

    let url = &args[1];
    match reqwest::get(url).await {
        Ok(response) => {
            println!("Status: {}", response.status());
            println!("Headers:");
            for (key, value) in response.headers() {
                println!("{}: {}", key, value.to_str().unwrap());
            }
            println!("Body:");
            println!("{}", response.text().await.unwrap());
        }
        Err(error) =>{eprintln!("Error: {}", error)}
    }
}
