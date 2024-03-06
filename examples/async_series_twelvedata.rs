use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{MarketClient, Twelvedata};
use std::env::var;
//use std::fs::File;

lazy_static! {
    static ref TOKEN: String =
        var("Twelvedata_TOKEN").expect("Twelvedata_TOKEN env variable is required");
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut site = Twelvedata::new(TOKEN.to_string());
    // output_size - supports values in the range from 1 to 5000 , default is 30.
    site.for_daily_series("AAPL".to_string(), 100);

    let client = MarketClient::new(site);

    // Creates the query URL & download raw data
    let client = client.create_endpoint()?.get_data().await?;

    // you can write the downloaded data to anything that implements std::io::Write , in this case a file
    // let buffer = File::create("raw_twelvedata_json.txt")?;
    // client.to_writer(buffer)?;

    // or transform into MarketSeries struct for further processing
    let data = client.transform_data()?;

    // println!("{}", data);

    // the data can be enhanced with the calculation of a series of indicators
    let enhanced_data = data
        .enhance_data()
        .with_sma(10)
        .with_ema(20)
        .with_ema(6)
        .with_rsi(14)
        .calculate();

    println!("{}", enhanced_data);

    Ok(())
}
