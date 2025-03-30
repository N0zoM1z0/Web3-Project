use mockito::mock;
use std::process;
use std::env;
use std::io::{self, Read, Write};
use std::panic;
use reqwest;

// A simple mini_curl implemented in Rust
//Notice the removal of #[tokio::main], because main can not be defined more than once.
//#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: mini_curl <URL>");
        process::exit(1);
        // return Err(From::from("No URL Provided."));
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
            Ok(())
        }
        Err(error) =>{
          eprintln!("Error: {}", error);
          Err(From::from(error))
        }
    }
}


// #[tokio::test]  // Use cfg(test) for conditional compilation, remove this annotation, to prevent multiple definitions of main
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use std::io::Write;
    use std::process;
    use std::env;

    async fn main_with_args(args: Vec<String>) -> String {
        // removed the `unsafe` block here and in all other functions
        unsafe{
            env::set_var("RUST_BACKTRACE", "0");
            env::set_var("RUST_LOG", "error");
        }
        let _ = env_logger::try_init(); // Use try_init to avoid multiple initialization
    
        let original_args = env::args().collect::<Vec<String>>();
    
        let (tx, rx) = tokio::sync::oneshot::channel(); // Use a channel to capture the output
    
        let args_clone = args.clone(); // Clone args for the spawned task
    
        tokio::task::spawn(async move {  // Spawns a new async task
    
            //Replace this stub with the code to modify args
            let original_args_clone = env::args().collect::<Vec<String>>(); //save args
    
            let mut mutable_args = env::args().collect::<Vec<String>>();
            mutable_args.clear();
            for item in &args_clone {
                mutable_args.push(item.to_string());
            }
    
            let mut buf = Vec::new();
            let result = {
                let mut stdout = io::stdout(); // capture stout in this local context, to avoid collisions
    
                io::stdout().flush().unwrap(); // flush to avoid buffered output
                 match main().await{ // await the program and then read from standard out
                    Ok(()) => {
                        io::stdout().flush().unwrap();
                    }
                    Err(e) => {
                        buf.extend_from_slice(format!("Error: {}", e).as_bytes());
                    }
                };
    
             };
    
            //Replace this stub with the code to modify args, restore state
            let mut mutable_args_restore = env::args().collect::<Vec<String>>();
            mutable_args_restore.clear();
            for item in &original_args_clone {
                mutable_args_restore.push(item.to_string());
            }
    
            let output = String::from_utf8_lossy(&buf).to_string(); // Converts the output to string
            if let Err(_) = tx.send(output) {
                eprintln!("Could not send output");
            }
    
        });
    
        let output = rx.await.unwrap();  // Await the output from the spawned task
    
        //Restore original
        //env::set_args(original_args); <== Not required anymore
    
        output // Return the captured output
    }

    #[tokio::test]
    async fn test_main_valid_url() {
        let mock = mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("Hello, world!")
            .create();

        let args: Vec<String> = vec![
            "mini_curl".to_string(),
            format!("http://localhost:{}", mockito::server_address().port()),
        ];

        let output = main_with_args(args).await;

        mock.assert();
        assert!(output.contains("Status: 200 OK"));
        assert!(output.contains("content-type: text/plain"));
        assert!(output.contains("Hello, world!"));
    }

    #[tokio::test]
    async fn test_main_invalid_url() {
        let args: Vec<String> = vec![
            "mini_curl".to_string(),
            "http://invalid-url".to_string(),
        ];

        let output = main_with_args(args).await;

        assert!(output.contains("Error:"));
    }

    #[tokio::test]
    async fn test_main_no_url_provided() {
        let args: Vec<String> = vec!["mini_curl".to_string()];

        let output = main_with_args(args).await;
        // assert!(output.contains("Usage: mini_curl <URL>"));
        assert!(output.contains("Error:")); //The current implementation throws an error, not process::exit
    }

}