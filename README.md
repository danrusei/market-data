# market-data

A Rust lib to fetch & enhance historical time-series stock market data.

The Sync version using the [ureq](https://crates.io/crates/ureq) http client is the default. An Async version using [reqwest](https://crates.io/crates/reqwest) http client can be selected by enabling the "use-async" feature.

To enable async feature:

```market-data = { version = "*", features = ["use-async"] }```

## Usage

Check the [Examples folder](https://github.com/danrusei/market-data/tree/main/examples) for more examples.

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
    .intraday_series("MSFT", 60, Interval::Hour2)?;

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
            .with_ema(6)
            .with_rsi(14)
            .calculate()
    })
    .collect();

enhanced_data
    .into_iter()
    .for_each(|enhanced_series| println!("{}", enhanced_series));

// Prints:
// Date: 2024-02-23, Open: 410.14, Close: 410.32, High: 410.85, Low: 409.84, Volume: 1814939.00, SMA 10: 405.48, EMA 20: 405.94, EMA 6: 408.56, RSI 14: 59.19,
// Date: 2024-02-23, Open: 410.11, Close: 410.12, High: 410.78, Low: 409.53, Volume: 1998775.00, SMA 10: 406.31, EMA 20: 406.34, EMA 6: 409.00, RSI 14: 57.86,
// Date: 2024-02-23, Open: 410.77, Close: 410.11, High: 410.86, Low: 408.97, Volume: 2621471.00, SMA 10: 407.10, EMA 20: 406.70, EMA 6: 409.32, RSI 14: 64.33,
// Date: 2024-02-23, Open: 415.67, Close: 410.73, High: 415.86, Low: 410.09, Volume: 6230853.00, SMA 10: 408.32, EMA 20: 407.08, EMA 6: 409.72, RSI 14: 68.58,
```

## Supported Publishers

Implementation is available for a number of sites that offer Free Tier. New Publishers can be added, your contribution is welcome.  Additional details can be found in [Publishers.md](https://github.com/danrusei/market-data/blob/main/Publishers.md) file. So far the following are supported:

* [x] [Alpha Vantage](https://www.alphavantage.co/documentation/)
* [x] [Twelvedata](https://twelvedata.com/docs#time-series)
* [x] [Iex cloud](https://iexcloud.io/docs/api/#rest-how-to) - may not work unless you use the paid subscribtions

Alternative providers, to be added:

* [] [Polygon](https://polygon.io/docs/stocks/get_v2_aggs_ticker__stocksticker__range__multiplier___timespan___from___to)
* [] [Nasdaq Data Link - WIKIP](https://data.nasdaq.com/databases/WIKIP#usage)
* [] [Marketstack](https://marketstack.com/documentation#historical_data)
* [] [Tradier](https://documentation.tradier.com/brokerage-api/markets/get-history)
* [] [Yahoo Finance site - maybe?](https://finance.yahoo.com/)
* [] [Stook parse site- maybe?](https://stooq.com/q/d/?s=aapl.us&i=d&d1=20230907&d2=20240229)


## Supported Market Technical Indicators

* [x] [Simple Moving Average (SMA)](https://www.investopedia.com/terms/s/sma.asp)
* [x] [Exponential Moving Averages (EMA)](https://www.investopedia.com/terms/e/ema.asp)
* [x] [Relative Strength Index (RSI)](https://www.investopedia.com/terms/r/rsi.asp)

Others to be implemented:

* [Stochastic Oscillator](https://www.investopedia.com/terms/s/stochasticoscillator.asp)
* [Moving Average Convergence/Divergence](https://www.investopedia.com/terms/m/macd.asp)
* and others


## For development

Export the API Keys, as: export Publisher_TOKEN=<your_toke_here>

Default feature in Cargo.toml is use-sync, if working on async version change the default to use-async.

Run the examples:

```bash
cargo run --example example_name

// for async
cargo run --example async_example_name --features="use-async" --no-default-features
```



