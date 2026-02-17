use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{Interval, MarketClient, Twelvedata};
use std::env::var;

lazy_static! {
    static ref TOKEN: String =
        var("Twelvedata_TOKEN").expect("Twelvedata_TOKEN env variable is required");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Select a Publisher
    let site = Twelvedata::new(TOKEN.to_string());
    // Create the MarketClient
    let client = MarketClient::new(site);

    // Create requests
    let weekly_req = client.site.weekly_series("GOOGL", 40);
    let intraday_req = client.site.intraday_series("MSFT", 40, Interval::Hour2)?;

    // Fetch the data
    let weekly_data = client.fetch(weekly_req).await?;
    let intraday_data = client.fetch(intraday_req).await?;

    // Print the data
    println!("Weekly Data:\n{}\n", weekly_data);
    println!("Intraday Data:\n{}\n", intraday_data);

    // Enhance data with indicators
    let enhanced_data = weekly_data
        .enhance_data()
        .with_sma(10)
        .with_ema(20)
        .with_rsi(14)
        .calculate();

    println!("Enhanced Data:\n{}", enhanced_data);

    Ok(())
}
