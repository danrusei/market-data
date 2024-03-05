use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{Iex, MarketClient};
use std::env::var;
//use std::fs::File;

lazy_static! {
    static ref TOKEN: String = var("IEX_TOKEN").expect("IEX_TOKEN env variable is required");
}

fn main() -> Result<()> {
    let mut site = Iex::new(TOKEN.to_string());
    site.for_series("AAPL".to_string(), "3m".to_string());

    let client = MarketClient::new(site);

    // Creates the query URL & download raw data and
    let client = client.create_endpoint()?.get_data()?;

    // you can write the downloaded data to anything that implements std::io::Write , in this case a file
    // let buffer = File::create("raw_iex_json.txt")?;
    // client.to_writer(buffer)?;

    // or transform into MarketSeries struct for further processing
    let data = client.transform_data()?;

    // println!("{}", data);
    // Prints:
    // Date: 2024-02-26, Open: 182.24, Close: 181.16, High: 182.76, Low: 180.65, Volume: 40867420
    // Date: 2024-02-27, Open: 181.1, Close: 182.63, High: 183.9225, Low: 179.56, Volume: 54318852
    // Date: 2024-02-28, Open: 182.51, Close: 181.42, High: 183.12, Low: 180.13, Volume: 48953940

    // the data can be enhanced with the calculation of a series of indicators
    let enhanced_data = data
        .enhance_data()
        .with_sma(10)
        .with_ema(20)
        .with_ema(6)
        .with_rsi(14)
        .calculate();

    println!("{}", enhanced_data);

    // Prints:
    // Date: 2024-02-26, Open: 182.24, Close: 181.16, High: 182.76, Low: 180.65, Volume: 40867420.00, SMA 10: 183.44, EMA 20: 185.25, EMA 6: 182.72, RSI 14: 30.43,
    // Date: 2024-02-27, Open: 181.10, Close: 182.63, High: 183.92, Low: 179.56, Volume: 54318852.00, SMA 10: 182.99, EMA 20: 185.00, EMA 6: 182.69, RSI 14: 29.80,
    // Date: 2024-02-28, Open: 182.51, Close: 181.42, High: 183.12, Low: 180.13, Volume: 48953940.00, SMA 10: 182.63, EMA 20: 184.66, EMA 6: 182.33, RSI 14: 27.31,

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
