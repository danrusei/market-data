use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{MarketClient, Twelvedata};
use std::{env::var, fs::File};

lazy_static! {
    static ref TOKEN: String =
        var("Twelvedata_TOKEN").expect("AlphaVantage_TOKEN env variable is required");
}

fn main() -> Result<()> {
    let mut site = Twelvedata::new(TOKEN.to_string());
    // output_size - supports values in the range from 1 to 5000 , default is 30.
    site.for_daily_series("AAPL".to_string(), 50);

    let client = MarketClient::new(site);

    // Creates the query URL & download raw data
    let client = client.create_endpoint()?.get_data()?;

    // you can write the downloaded data to anything that implements std::io::Write , in this case a file
    // let buffer = File::create("raw_twelvedata_json.txt")?;
    // client.to_writer(buffer)?;

    // or transform into MarketSeries struct for further processing
    let data = client.transform_data()?;

    println!("{}", data);

    Ok(())
}
