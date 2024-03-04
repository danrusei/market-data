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

    let mut client = MarketClient::new(site);
    client.create_endpoint()?;
    client.get_data()?;
    let data = client.transform_data();
    if let Ok(data) = data {
        println!("{}", data);
    }
    Ok(())
}
