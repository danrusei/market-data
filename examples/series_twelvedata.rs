use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{EnhancedMarketSeries, Interval, MarketClient, Twelvedata};
use std::env::var;
//use std::fs::File;

lazy_static! {
    static ref TOKEN: String =
        var("Twelvedata_TOKEN").expect("Twelvedata_TOKEN env variable is required");
}

fn main() -> Result<()> {
    // Select a Publisher from the available ones
    let mut site = Twelvedata::new(TOKEN.to_string());

    // configure to retrieve Daily, Weekly or Intraday series, check the available methods for each publisher
    // output_size is mandatory for Twelvedata - and supports values in the range from 1 to 5000 , default is 30.
    // multiple requests can be added
    // site.weekly_series("GOOGL".to_string(), 40);
    site.daily_series("GOOGL".to_string(), 300);

    // create the MarketClient
    let mut client = MarketClient::new(site);

    // creates the query URL & download the raw data
    client = client.create_endpoint()?.get_data()?;
    // transform into MarketSeries, that can be used for further processing
    let data = client.transform_data();

    // print the data
    // data.iter().for_each(|output| match output {
    //     Ok(data) => println!("{}\n\n", data),
    //     Err(err) => println!("{}", err),
    // });

    // you can reuse the client to download additional series
    // client
    //     .site
    //     .intraday_series("MSFT".to_string(), 200, Interval::Hour2)?;

    // pattern with consuming the client, the client can't be reused for configuring new series
    // let data2 = client.create_endpoint()?.get_data()?.transform_data();

    // the data can be enhanced with the calculation of a number of  market indicators
    let enhanced_data: Vec<EnhancedMarketSeries> = data
        .into_iter()
        .filter_map(|series| series.ok())
        .map(|series| {
            series
                .enhance_data()
                .with_sma(10)
                .with_ema(20)
                .with_rsi(14)
                .with_stochastic(14)
                .calculate()
        })
        .collect();

    // another option if you want to keep the error variants as well
    // let enhanced_data: Vec<MarketResult<EnhancedMarketSeries>> = data2
    //     .iter()
    //     .map(|output| match output {
    //         Ok(data2) => Ok(data2
    //             .enhance_data()
    //             .with_sma(10)
    //             .with_ema(20)
    //             .with_ema(6)
    //             .with_rsi(14)
    //             .calculate()),
    //         Err(err) => Err(MarketError::DownloadedData(format!(
    //             "Errors with downloaded data: {}",
    //             err
    //         ))),
    //     })
    //     .collect();

    enhanced_data
        .into_iter()
        .for_each(|enhanced_series| println!("{}", enhanced_series));

    Ok(())
}
