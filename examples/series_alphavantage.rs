use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{AlphaVantage, EnhancedMarketSeries, MarketClient, OutputSize};
use std::env::var;
//use std::fs::File;

lazy_static! {
    static ref TOKEN: String =
        var("AlphaVantage_TOKEN").expect("AlphaVantage_TOKEN env variable is required");
}

fn main() -> Result<()> {
    // Select a Alpha Vantage as publisher
    let mut site = AlphaVantage::new(TOKEN.to_string());

    // OutputSize::Compact - returns only the latest 100 data points
    // OutputSize::Full - returns the full-length time series of 20+ years of historical data
    // multiple requests can be added
    site.daily_series("AAPL".to_string(), OutputSize::Compact);

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

    // you can reuse the client to download additional series
    // client.site.intraday_series( "MSFT".to_string(), OutputSize::Compact, AlphaInterval::Min30,);
    client
        .site
        .weekly_series("MSFT".to_string(), OutputSize::Compact);

    // pattern with consuming the client, the client can't be reused for configuring new series
    let data2 = client.create_endpoint()?.get_data()?.transform_data();

    // the data can be enhanced with the calculation of a number of  market indicators
    let enhanced_data: Vec<EnhancedMarketSeries> = data2
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

    Ok(())
}
