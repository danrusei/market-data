use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{Iex, MarketClient};
use std::env::var;

lazy_static! {
    static ref TOKEN: String = var("IEX_TOKEN").expect("IEX_TOKEN env variable is required");
}

fn main() -> Result<()> {
    let mut site = Iex::new(TOKEN.to_string());
    site.for_series("AAPL".to_string(), "3m".to_string());

    let mut client = MarketClient::new(site);
    client.create_endpoint()?;
    client.get_data()?;
    Ok(())
}

//acceptable ranges
// max	All available data up to 15 years	Historically adjusted market-wide data
// 5y	Five years	Historically adjusted market-wide data
// 2y	Two years	Historically adjusted market-wide data
// 1y	One year	Historically adjusted market-wide data
// ytd	Year-to-date	Historically adjusted market-wide data
// 6m	Six months	Historically adjusted market-wide data
// 3m	Three months	Historically adjusted market-wide data
// 1m	One month (default)	Historically adjusted market-wide data
// 1mm	One month	Historically adjusted market-wide data in 30 minute intervals
// 5d	Five Days	Historically adjusted market-wide data by day.
