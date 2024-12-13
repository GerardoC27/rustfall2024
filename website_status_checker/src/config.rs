use std::time::Duration;

/// Configuration struct for monitoring settings
#[derive(Clone)]
pub struct Config {
    pub num_threads: usize,
    pub timeout: Duration,
    pub max_retries: usize,
}

impl Config {
    /// Create a new configuration
    pub fn new(num_threads: usize, timeout_secs: u64, max_retries: usize) -> Self {
        Config {
            num_threads,
            timeout: Duration::from_secs(timeout_secs),
            max_retries,
        }
    }
}
