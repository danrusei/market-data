use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{AlphaVantage, MarketClient, OutputSize};
use std::env::var;
//use std::fs::File;

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

    // Creates the query URL & download the raw data
    let client = client.create_endpoint()?.get_data()?;

    // you can write the downloaded data to anything that implements std::io::Write , in this case a file
    // let buffer = File::create("raw_alphavantage_json.txt")?;
    // client.to_writer(buffer)?;

    // or transform into MarketSeries struct for further processing
    let data = client.transform_data()?;

    // println!("{}", data);

    // the data can be enhanced with the calculation of a series of indicators
    let enhanced_data = data
        .enhance_data()
        .with_sma(10)
        .with_ema(20)
        .with_rsi(14)
        .calculate();

    println!("{}", enhanced_data);

    Ok(())
}
