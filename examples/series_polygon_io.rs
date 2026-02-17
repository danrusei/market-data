use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{MarketClient, Polygon};
use std::env::var;

lazy_static! {
    static ref TOKEN: String =
        var("Polygon_TOKEN").expect("Polygon_TOKEN env variable is required");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Select a Publisher
    let site = Polygon::new(TOKEN.to_string());
    // Create the MarketClient
    let client = MarketClient::new(site);

    // Create a request
    let request = client
        .site
        .daily_series("GOOGL", "2024-01-01", "2024-03-01", 5000);

    // Fetch the data
    let data = client.fetch(request).await?;

    // Enhance the data with indicators
    let enhanced_data = data
        .enhance_data()
        .with_sma(10)
        .with_ema(20)
        .with_rsi(14)
        .with_macd(12, 26, 9)
        .calculate();

    // Print the enhanced data
    println!("{}", enhanced_data);

    Ok(())
}
