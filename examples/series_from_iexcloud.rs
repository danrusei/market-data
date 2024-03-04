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

    let client = MarketClient::new(site);
    // Creates the query URL, download raw data and transform into MarketSeries struct
    let data = client.create_endpoint()?.get_data()?.transform_data()?;

    println!("{}", data);

    Ok(())
}

// It prints:
//
// Series: Date: 2023-12-04, Open: 189.98, Close: 189.43, High: 190.05, Low: 187.4511, Volume: 43389520
// Series: Date: 2023-12-05, Open: 190.21, Close: 193.42, High: 194.4, Low: 190.18, Volume: 66628400
// Series: Date: 2023-12-05, Open: 190.21, Close: 193.42, High: 194.4, Low: 190.18, Volume: 66628400
//.........

//Acceptable ranges
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
