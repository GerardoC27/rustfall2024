use crate::config::Config;
use std::time::{Duration, Instant};
use std::fmt;

/// Struct to represent the status of a website
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: std::time::SystemTime,
}

impl fmt::Display for WebsiteStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.status {
            Ok(code) => write!(f, "Website: {}, Status Code: {}, Response Time: {:?}, Timestamp: {:?}", self.url, code, self.response_time, self.timestamp),
            Err(err) => write!(f, "Website: {}, Error: {}, Response Time: {:?}, Timestamp: {:?}", self.url, err, self.response_time, self.timestamp),
        }
    }
}

/// Check the status of a website
pub fn check_website(url: &str, config: &Config) -> WebsiteStatus {
    let mut attempts = 0;
    let start_time = Instant::now();

    // Attempt to make a request with retries
    let mut result = Err("Max retries reached".to_string());
    while attempts <= config.max_retries {
        attempts += 1;
        match ureq::get(url).timeout(config.timeout).call() {
            Ok(response) => {
                result = Ok(response.status());
                break;
            }
            Err(e) => {
                if attempts > config.max_retries {
                    result = Err(format!("Failed after {} retries: {}", attempts, e));
                }
            }
        }
    }

    let response_time = start_time.elapsed();
    WebsiteStatus {
        url: url.to_string(),
        status: result,
        response_time,
        timestamp: std::time::SystemTime::now(),
    }
}
