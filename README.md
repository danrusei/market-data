# market-data

A Rust lib to fetch & enhance historical time-series stock market data.
For fetching data a Sync version using [ureq](https://crates.io/crates/ureq) http client, to reduce the dependencies but also an Async version using [reqwest](https://crates.io/crates/reqwest) http client.

For async enable feature in dependency: ```market-data = { version = "*", features = ["use-async"] }```

## Usage

Check the [Examples folder](https://github.com/danrusei/market-data/tree/main/examples) for more examples.

```rust
use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{Iex, MarketClient};
use std::env::var;
//use std::fs::File;

lazy_static! {
    static ref TOKEN: String = var("IEX_TOKEN").expect("IEX_TOKEN env variable is required");
}

fn main() -> Result<()> {
    
    // Check all the providers supported as they are slightly different
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

    println!("{}", data);
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
```

## Supported Publishers

Selected a number of sites that offer Free Tier, new Publishers can be added, your contribution is welcome.
So far the following are supported.

* [x] [Alpha Vantage](https://www.alphavantage.co/documentation/)
* [x] [Twelvedata](https://twelvedata.com/docs#time-series)
* [x] [Iex cloud](https://iexcloud.io/docs/api/#rest-how-to)

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


## Running the examples during development

Make sure that the api keys are exported, like: export Publisher_TOKEN=<your_toke_here>

In Cargo.toml use-sync is the default feature.

```bash
cargo run --example series_alphavantage
cargo run --example series_iexcloud
cargo run --example series_twelvedata
cargo run --example async_series_twelvedata --features="use-async" --no-default-features
```



