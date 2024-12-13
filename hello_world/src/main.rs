// File: main.rs

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use reqwest::blocking::Client;
use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};
use signal_hook::{consts::SIGINT, iterator::Signals};

// Struct to store website status information
#[derive(Debug)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>, // HTTP status code or error message
    response_time: Duration,     // Time taken to get a response
    timestamp: DateTime<Utc>,    // When the check was performed
}

// Configuration for monitoring
struct Config {
    num_threads: usize,
    timeout: Duration,
    max_retries: usize,
}

// Function to check the status of a website
fn check_website(url: &str, timeout: Duration, retries: usize) -> WebsiteStatus {
    let client = Client::builder().timeout(timeout).build().unwrap();
    let start_time = Instant::now();
    let mut attempts = 0;

    // Try to send the request with retries
    let mut result = Err("Max retries reached".to_string());
    while attempts <= retries {
        attempts += 1;
        match client.get(url).send() {
            Ok(response) => {
                result = Ok(response.status().as_u16());
                break;
            }
            Err(e) => {
                if attempts > retries {
                    result = Err(format!("Failed after {} retries: {}", retries, e));
                }
            }
        }
    }

    let response_time = start_time.elapsed();
    WebsiteStatus {
        url: url.to_string(),
        status: result,
        response_time,
        timestamp: Utc::now(),
    }
}

fn main() {
    // Configuration
    let config = Config {
        num_threads: 4,
        timeout: Duration::from_secs(5),
        max_retries: 2,
    };

    // List of websites to monitor
    let urls = vec![
        "https://www.google.com",
        "https://www.rust-lang.org",
        "https://www.github.com",
        "https://www.thiswebsitedoesnotexist.com", // Invalid URL for testing
    ];

    // Shared signal for graceful shutdown
    let shutdown = Arc::new(Mutex::new(false));
    let shutdown_clone = Arc::clone(&shutdown);

    // Setup signal handling for SIGINT (Ctrl+C)
    thread::spawn(move || {
        let mut signals = Signals::new(&[SIGINT]).expect("Failed to register SIGINT");
        for _ in signals.forever() {
            let mut shutdown = shutdown_clone.lock().unwrap();
            *shutdown = true;
            break;
        }
    });

    // Channel for worker communication
    let (tx, rx) = mpsc::channel();

    // Spawn worker threads
    thread::scope(|s| {
        for chunk in urls.chunks(config.num_threads) {
            let tx_clone = tx.clone();
            let urls_chunk = chunk.to_vec();
            s.spawn(move || {
                for url in urls_chunk {
                    let status = check_website(url, config.timeout, config.max_retries);
                    tx_clone.send(status).unwrap();
                }
            });
        }
    });

    // Close the sender
    drop(tx);

    // Collect and display results
    for result in rx {
        match &result.status {
            Ok(code) => println!(
                "Website: {}, Status Code: {}, Response Time: {:?}, Checked At: {}",
                result.url, code, result.response_time, result.timestamp
            ),
            Err(err) => println!(
                "Website: {}, Error: {}, Checked At: {}",
                result.url, err, result.timestamp
            ),
        }
    }

    println!("Shutting down.");
}
