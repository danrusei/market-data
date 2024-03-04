use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{MarketClient, Twelvedata};
use std::env::var;

lazy_static! {
    static ref TOKEN: String =
        var("Twelvedata_TOKEN").expect("AlphaVantage_TOKEN env variable is required");
}

fn main() -> Result<()> {
    let mut site = Twelvedata::new(TOKEN.to_string());
    // output_size - supports values in the range from 1 to 5000 , default is 30.
    site.for_daily_series("AAPL".to_string(), 50);

    let client = MarketClient::new(site);
    // Creates the query URL, download raw data and transform into MarketSeries struct
    let data = client.create_endpoint()?.get_data()?.transform_data()?;

    println!("{}", data);

    Ok(())
}
