use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{EnhancedMarketSeries, Iex, MarketClient};
use std::env::var;
//use std::fs::File;

lazy_static! {
    static ref TOKEN: String = var("IEX_TOKEN").expect("IEX_TOKEN env variable is required");
}

fn main() -> Result<()> {
    // Select Iex Publisher:
    let mut site = Iex::new(TOKEN.to_string());
    site.daily_series("AAPL", "3m");
    site.daily_series("MSFT", "3m");

    // create the MarketClient
    let mut client = MarketClient::new(site);

    // creates the query URL & download the raw data
    client = client.create_endpoint()?.get_data()?;
    // transform into MarketSeries, that can be used for further processing
    let data = client.transform_data();

    // print the data
    data.iter().for_each(|output| match output {
        Ok(data) => println!("{}\n\n", data),
        Err(err) => println!("{}", err),
    });

    // println!("{}", data);
    // Prints:
    // Date: 2024-02-26, Open: 182.24, Close: 181.16, High: 182.76, Low: 180.65, Volume: 40867420
    // Date: 2024-02-27, Open: 181.1, Close: 182.63, High: 183.9225, Low: 179.56, Volume: 54318852
    // Date: 2024-02-28, Open: 182.51, Close: 181.42, High: 183.12, Low: 180.13, Volume: 48953940

    // the data can be enhanced with the calculation of a number of  market indicators
    let enhanced_data: Vec<EnhancedMarketSeries> = data
        .into_iter()
        .filter_map(|series| series.ok())
        .map(|series| {
            series
                .enhance_data()
                .with_sma(10)
                .with_ema(20)
                .with_ema(6)
                .with_rsi(14)
                .calculate()
        })
        .collect();

    enhanced_data
        .into_iter()
        .for_each(|enhanced_series| println!("{}", enhanced_series));

    // Prints:
    // Date: 2024-02-26, Open: 182.24, Close: 181.16, High: 182.76, Low: 180.65, Volume: 40867420.00, SMA 10: 183.44, EMA 20: 185.25, EMA 6: 182.72, RSI 14: 30.43,
    // Date: 2024-02-27, Open: 181.10, Close: 182.63, High: 183.92, Low: 179.56, Volume: 54318852.00, SMA 10: 182.99, EMA 20: 185.00, EMA 6: 182.69, RSI 14: 29.80,
    // Date: 2024-02-28, Open: 182.51, Close: 181.42, High: 183.12, Low: 180.13, Volume: 48953940.00, SMA 10: 182.63, EMA 20: 184.66, EMA 6: 182.33, RSI 14: 27.31,

    Ok(())
}
