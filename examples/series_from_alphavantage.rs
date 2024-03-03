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

    let mut client = MarketClient::new(site);
    client.create_endpoint()?;
    client.get_data()?;
    let data = client.transform_data();
    if let Ok(data) = data {
        println!("{}", data);
    }
    Ok(())
}
