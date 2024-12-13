use crate::config::Config;
use crate::utils::{check_website, WebsiteStatus};
use std::sync::mpsc;
use std::thread;

/// Struct to manage website monitoring
pub struct WebsiteMonitor {
    urls: Vec<String>,
    config: Config,
}

impl WebsiteMonitor {
    /// Create a new WebsiteMonitor instance
    pub fn new(urls: Vec<String>, config: Config) -> Self {
        WebsiteMonitor { urls, config }
    }

    /// Run the monitoring process
    pub fn run(&self) {
        let (tx, rx) = mpsc::channel();

        // Spawn threads for concurrent monitoring
        thread::scope(|s| {
            for chunk in self.urls.chunks(self.config.num_threads) {
                let tx_clone = tx.clone();
                let urls_chunk = chunk.to_vec();
                let config = self.config.clone();
                s.spawn(move || {
                    for url in urls_chunk {
                        let status = check_website(&url, &config);
                        tx_clone.send(status).unwrap();
                    }
                });
            }
        });

        drop(tx); // Close sender channel

        // Collect and display results
        for result in rx {
            println!("{}", result);
        }
    }
}
