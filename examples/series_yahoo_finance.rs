use anyhow::Result;
use market_data::{EnhancedMarketSeries, Interval, MarketClient, YahooFin, YahooRange};

fn main() -> Result<()> {
    // Select a Publisher from the available ones
    let mut site = YahooFin::new();

    // configure to retrieve Daily, Weekly or Intraday series, check the available methods for each publisher
    // interval - use exported enum Interval
    // range - use exported enum YahooRange that alllows only the values supported by Yahoo
    // multiple requests can be added
    site.weekly_series("GOOGL", YahooRange::Year1);
    site.intraday_series("MSFT", Interval::Hour1, YahooRange::Day5)?;

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
    // client.site.daily_series("GOOGL", YahooRange::Month6);

    // pattern with consuming the client, the client can't be reused for configuring new series
    // let data2 = client.create_endpoint()?.get_data()?.transform_data();

    // the data can be enhanced with the calculation of a number of  market indicators
    // let enhanced_data: Vec<EnhancedMarketSeries> = data2
    //     .into_iter()
    //     .filter_map(|series| series.ok())
    //     .map(|series| {
    //         series
    //             .enhance_data()
    //             .with_sma(10)
    //             .with_ema(20)
    //             .with_rsi(14)
    //             .with_macd(12, 26, 9)
    //             .calculate()
    //     })
    //     .collect();

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

    // enhanced_data
    //     .into_iter()
    //     .for_each(|enhanced_series| println!("{}", enhanced_series));

    Ok(())
}
