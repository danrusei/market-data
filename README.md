# market-data

A Rust lib to fetch & enhance historical time-series stock market data.

This library provides a stateless, async-first API to download historical stock data from various providers and enhance it with common technical indicators.

## Features

- **Async-first**: Built on `tokio` and `reqwest`.
- **Stateless Architecture**: Reusable clients and immutable request objects.
- **Multiple Publishers**: Supports Finnhub, Alpha Vantage, Massive (formerly Polygon.io), Twelvedata, and Yahoo Finance.
- **Technical Indicators**: Built-in support for SMA, EMA, RSI, MACD, and Stochastic Oscillator.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
market-data = "0.4"
tokio = { version = "1.0", features = ["full"] }
```

## Usage

Each publisher provides a set of methods to create request objects, which are then passed to the `MarketClient`.

```rust
use market_data::{MarketClient, Twelvedata, Interval};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Select a Publisher (e.g., Twelvedata)
    let site = Twelvedata::new("YOUR_TOKEN".to_string());
    
    // 2. Create the MarketClient
    let client = MarketClient::new(site);

    // 3. Create a stateless request
    let request = client.site.intraday_series("MSFT", 150, Interval::Min15)?;

    // 4. Fetch the data
    let data = client.fetch(request).await?;

    // 5. Enhance the data with technical indicators
    let enhanced_data = data
        .enhance_data()
        .with_sma(10)
        .with_ema(20)
        .with_rsi(14)
        .with_macd(12, 26, 9)
        .calculate();

    // 6. Print the results
    println!("{}", enhanced_data);

    Ok(())
}
```

### Supported Publishers

Details on rate limits and historical data depth can be found in [Publishers.md](Publishers.md).

- [x] [Finnhub](https://finnhub.io/docs/api)
- [x] [Alpha Vantage](https://www.alphavantage.co/documentation/)
- [x] [Massive](https://massive.com/docs/rest/stocks/aggregates/custom-bars) (formerly Polygon.io)
- [x] [Twelvedata](https://twelvedata.com/docs#time-series)
- [x] [Yahoo Finance](https://finance.yahoo.com/) (Unofficial, no token required)

### Supported Market Technical Indicators

- [x] [Simple Moving Average (SMA)](https://www.investopedia.com/terms/s/sma.asp)
- [x] [Exponential Moving Averages (EMA)](https://www.investopedia.com/terms/e/ema.asp)
- [x] [Relative Strength Index (RSI)](https://www.investopedia.com/terms/r/rsi.asp)
- [x] [Stochastic Oscillator](https://www.investopedia.com/terms/s/stochasticoscillator.asp)
- [x] [Moving Average Convergence/Divergence (MACD)](https://www.investopedia.com/terms/m/macd.asp)

## For Development

To run the examples, export your API keys:

```bash
export Finnhub_TOKEN=<your_token>
export Twelvedata_TOKEN=<your_token>
# etc...
```

Run an example:

```bash
cargo run --example series_finnhub
```

## Contributing

Contributions are welcome! If you'd like to add a new publisher or technical indicator, please raise a PR or create an issue.

## License

Apache-2.0
