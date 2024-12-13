mod config;
mod monitor;
mod utils;

use config::Config;
use monitor::WebsiteMonitor;

fn main() {
    // Create configuration
    let config = Config::new(4, 5, 2);

    // List of URLs to monitor
    let urls = vec![
        "https://www.google.com".to_string(),
        "https://www.rust-lang.org".to_string(),
        "https://www.github.com".to_string(),
        "https://www.thiswebsitedoesnotexist.com".to_string(),
    ];

    // Create a website monitor
    let monitor = WebsiteMonitor::new(urls, config);

    // Run the monitoring process
    monitor.run();
}
