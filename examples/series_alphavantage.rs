use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{AlphaVantage, MarketClient, OutputSize};
use std::env::var;

lazy_static! {
    static ref TOKEN: String =
        var("AlphaVantage_TOKEN").expect("AlphaVantage_TOKEN env variable is required");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Select a Publisher
    let site = AlphaVantage::new(TOKEN.to_string());
    // Create the MarketClient
    let client = MarketClient::new(site);

    // Create a request
    let request = client.site.daily_series("AAPL", OutputSize::Compact);

    // Fetch the data
    let data = client.fetch(request).await?;

    // Print the data
    println!("{}", data);

    Ok(())
}
