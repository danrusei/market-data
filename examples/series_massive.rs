use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{MarketClient, Massive};
use std::env::var;

lazy_static! {
    static ref TOKEN: String =
        var("Massive_TOKEN").expect("Massive_TOKEN env variable is required");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Select a Publisher
    let site = Massive::new(TOKEN.to_string());
    // Create the MarketClient
    let client = MarketClient::new(site);

    // Create a request
    let request = client
        .site
        .daily_series("GOOGL", "2026-01-01", "2026-02-15", 5000);

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
