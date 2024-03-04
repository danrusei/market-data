use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{AlphaVantage, MarketClient, OutputSize};
use std::env::var;

lazy_static! {
    static ref TOKEN: String =
        var("AlphaVantage_TOKEN").expect("AlphaVantage_TOKEN env variable is required");
}

fn main() -> Result<()> {
    let mut site = AlphaVantage::new(TOKEN.to_string());
    // OutputSize::Compact - returns only the latest 100 data points
    // OutputSize::Full - returns the full-length time series of 20+ years of historical data
    site.for_daily_series("AAPL".to_string(), OutputSize::Compact);

    let client = MarketClient::new(site);
    // Creates the query URL, download raw data and transform into MarketSeries struct
    let data = client.create_endpoint()?.get_data()?.transform_data()?;

    println!("{}", data);

    Ok(())
}
