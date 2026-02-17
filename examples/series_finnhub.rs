use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{Finnhub, MarketClient};
use std::env::var;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref TOKEN: String =
        var("Finnhub_TOKEN").expect("Finnhub_TOKEN env variable is required");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Select a Publisher
    let site = Finnhub::new(TOKEN.to_string());
    // Create the MarketClient
    let client = MarketClient::new(site);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64;
    let month_ago = now - 30 * 24 * 60 * 60;

    // Create a request
    let request = client.site.daily_series("AAPL", month_ago, now);

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
