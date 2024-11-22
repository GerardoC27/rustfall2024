use chrono::Utc;
use serde::Deserialize;
use std::{
    any::Any,
    fs::OpenOptions,
    io::Write,
    thread,
    time::Duration,
};

// Struct Definitions
#[derive(Deserialize, Debug)]
pub struct Bitcoin;

#[derive(Deserialize, Debug)]
pub struct Ethereum;

#[derive(Deserialize, Debug)]
pub struct SP500;

// Trait Definition
pub trait Pricing {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>>;
    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn std::error::Error>>;
    fn as_any(&self) -> &dyn Any; // Allow downcasting
}

// Implementations for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let response = ureq::get("https://api.coindesk.com/v1/bpi/currentprice/BTC.json")
            .call()?
            .into_reader();
        let response: serde_json::Value = serde_json::from_reader(response)?;
        let price = response["bpi"]["USD"]["rate_float"]
            .as_f64()
            .ok_or("Failed to parse Bitcoin price")?;
        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = Utc::now();
        let formatted_data = format!("{} - Bitcoin: ${:.2}\n", timestamp, price);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("bitcoin_price.txt")?;
        file.write_all(formatted_data.as_bytes())?;
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Implementations for Ethereum
impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
            .call()?
            .into_reader();
        let response: serde_json::Value = serde_json::from_reader(response)?;
        let price = response["ethereum"]["usd"]
            .as_f64()
            .ok_or("Failed to parse Ethereum price")?;
        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = Utc::now();
        let formatted_data = format!("{} - Ethereum: ${:.2}\n", timestamp, price);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("ethereum_price.txt")?;
        file.write_all(formatted_data.as_bytes())?;
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Implementations for S&P 500
impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let response = ureq::get("https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=SPY&apikey=YOUR_API_KEY")
            .call()?
            .into_reader();
        let response: serde_json::Value = serde_json::from_reader(response)?;

        // Check for rate-limiting message
        if let Some(info_message) = response.get("Information") {
            eprintln!("Rate limit hit: {}", info_message.as_str().unwrap_or("Unknown error"));
            return Err("Rate limit exceeded".into());
        }

        // Extract the price from the response
        let price_str = response["Global Quote"]["05. price"]
            .as_str()
            .ok_or("Price field missing in S&P 500 response")?;
        let price = price_str.parse::<f64>()?;
        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = Utc::now();
        let formatted_data = format!("{} - S&P 500: ${:.2}\n", timestamp, price);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("sp500_price.txt")?;
        file.write_all(formatted_data.as_bytes())?;
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Main Function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bitcoin = Bitcoin;
    let ethereum = Ethereum;
    let sp500 = SP500;

    let assets: Vec<&dyn Pricing> = vec![&bitcoin, &ethereum, &sp500];
    let mut sp500_last_fetch = Utc::now() - chrono::Duration::hours(1); // Ensure initial S&P fetch

    loop {
        for asset in &assets {
            // S&P 500 fetch logic
            if let Some(_) = asset.as_any().downcast_ref::<SP500>() {
                let now = Utc::now();
                if now.signed_duration_since(sp500_last_fetch).num_seconds() < 3600 {
                    println!("Skipping S&P 500 fetch: last fetch was within 1 hour.");
                    continue;
                }
                sp500_last_fetch = now; // Update last fetch time
            }

            // Fetch and save data
            match asset.fetch_price() {
                Ok(price) => {
                    println!("Fetched price: ${:.2}", price);
                    if let Err(e) = asset.save_to_file(price) {
                        eprintln!("Failed to save data: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to fetch price: {}", e),
            }
        }

        thread::sleep(Duration::from_secs(10)); // Adjust delay
    }
}
