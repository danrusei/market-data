# market-data

A Rust lib to fetch & enhance historical time-series stock market data.

Default is the Sync version that is using [ureq](https://crates.io/crates/ureq) as http client. An Async version using [reqwest](https://crates.io/crates/reqwest) http client can be enabled using the "use-async" feature.

To enable async feature:

```market-data = { version = "*", features = ["use-async"] }```

## Usage

Check the [Examples folder](https://github.com/danrusei/market-data/tree/main/examples) for examples per publisher.

```rust
// Select a Publisher from the available ones
let mut site = Twelvedata::new(TOKEN.to_string());

// configure to retrieve Daily, Weekly or Intraday series, check the available methods for each publisher
// output_size is mandatory for Twelvedata - and supports values in the range from 1 to 5000 , default is 30.
// multiple requests can be added
site.weekly_series("GOOGL", 40);
site.daily_series("GOOGL", 30);

// create the MarketClient
let mut client = MarketClient::new(site);

// creates the query URL & download the raw data
client = client.create_endpoint()?.get_data()?;
// transform into MarketSeries, that can be used for further processing
let data = client.transform_data();

// prints the data
data.iter().for_each(|output| match output {
    Ok(data) => println!("{}\n\n", data),
    Err(err) => println!("{}", err),
});

// the client can be reused for additional series
client
    .site
    .intraday_series("MSFT", 200, Interval::Hour2)?;

// the consuming the client pattern, the client can't be reused for configuring new series
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
            .with_rsi(14)
            .with_macd(12, 26, 9)
            .calculate()
    })
    .collect();

enhanced_data
    .into_iter()
    .for_each(|enhanced_series| println!("{}", enhanced_series));
```

// Print example of the Enhanced Series on GOOGL Daily - with SMA 10, EMA 20, RSI 14, MACD (12, 26, 9):

```bash
Date: 2023-12-27, Open: 141.59, Close: 140.37, High: 142.08, Low: 139.89, Volume: 19628600.00, SMA 10: 137.17, EMA 20: 136.40, RSI 14: 62.03, MACD (12, 26, 9): 1.87, 1.03, 0.84, ,

Date: 2023-12-28, Open: 140.78, Close: 140.23, High: 141.14, Low: 139.75, Volume: 16045700.00, SMA 10: 137.94, EMA 20: 136.77, RSI 14: 61.58, MACD (12, 26, 9): 1.93, 1.21, 0.72, ,

Date: 2023-12-29, Open: 139.63, Close: 139.69, High: 140.36, Low: 138.78, Volume: 18727200.00, SMA 10: 138.71, EMA 20: 137.05, RSI 14: 59.79, MACD (12, 26, 9): 1.92, 1.35, 0.57, ,

Date: 2024-01-02, Open: 138.55, Close: 138.17, High: 139.45, Low: 136.48, Volume: 23711200.00, SMA 10: 139.27, EMA 20: 137.15, RSI 14: 54.93, MACD (12, 26, 9): 1.76, 1.43, 0.33, ,

Date: 2024-01-03, Open: 137.25, Close: 138.92, High: 139.63, Low: 137.08, Volume: 24212100.00, SMA 10: 139.58, EMA 20: 137.32, RSI 14: 56.79, MACD (12, 26, 9): 1.68, 1.48, 0.20, ,

Date: 2024-01-04, Open: 138.42, Close: 136.39, High: 139.16, Low: 136.35, Volume: 27137700.00, SMA 10: 139.55, EMA 20: 137.23, RSI 14: 49.37, MACD (12, 26, 9): 1.40, 1.47, -0.07, ,

Date: 2024-01-05, Open: 136.75, Close: 135.73, High: 137.16, Low: 135.15, Volume: 22506000.00, SMA 10: 139.29, EMA 20: 137.09, RSI 14: 47.62, MACD (12, 26, 9): 1.11, 1.39, -0.29, ,
```

## Supported Publishers

Implementation is available for several sites that offer also Free Tier, besides the paid subscriptions. Additional details can be found in [Publishers.md](https://github.com/danrusei/market-data/blob/main/Publishers.md) file. 

So far the following are supported:

* [x] [Alpha Vantage](https://www.alphavantage.co/documentation/)
* [x] [Twelvedata](https://twelvedata.com/docs#time-series)
* [x] [Polygon.io](https://polygon.io/docs/stocks/getting-started)
* [] [Yahoo Finance](https://finance.yahoo.com/)
* [x] [Iex cloud](https://iexcloud.io/docs/api/#rest-how-to) - may not work unless a paid subscribtions is used

New Publishers can be added (as [Nasdaq Data Link - WIKIP](https://data.nasdaq.com/databases/WIKIP#usage), [Marketstack](https://marketstack.com/documentation#historical_data) and others).

Contribution is welcome, if you need other Publishers(source sites) raise a PR or create an issue.

## Supported Market Technical Indicators

* [x] [Simple Moving Average (SMA)](https://www.investopedia.com/terms/s/sma.asp)
* [x] [Exponential Moving Averages (EMA)](https://www.investopedia.com/terms/e/ema.asp)
* [x] [Relative Strength Index (RSI)](https://www.investopedia.com/terms/r/rsi.asp)
* [x] [Stochastic Oscillator](https://www.investopedia.com/terms/s/stochasticoscillator.asp)
* [x] [Moving Average Convergence/Divergence (MACD)](https://www.investopedia.com/terms/m/macd.asp)

Contribution is welcome,  if you need other indicators raise a PR or create an issue.


## For development

Export the API Keys, as: export Publisher_TOKEN=<your_toke_here>

Default feature in Cargo.toml is use-sync, if working on async version change the default to use-async.

Run the examples:

```bash
cargo run --example example_name

// for async
cargo run --example async_example_name --features="use-async" --no-default-features
```



