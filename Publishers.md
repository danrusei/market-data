# Supported Publishers (time-series data only)

## [Twelvedata](https://twelvedata.com/docs#time-series)

An API key is required, that can be obtain for free by signing up on the site.

Basic Free Account offer 800 API credits per day & 8 credits pe minute, however check their site for latest information. A link that exaplain the [credits calculation](https://support.twelvedata.com/en/articles/5615854-credits).

### Build request

Select Twelvedata publisher:

```rust
let mut site = Twelvedata::new(TOKEN.to_string());
```

For the requested instrument select the time series. Multiple can be added. 
The available methods:

* *intraday_series (symbol: String, output_size: u32, interval: Interval)*
* *daily_series (symbol: String, output_size: u32)*
* *weekly_series (symbol: String, output_size: u32)*
* *monthly_series (symbol: String, output_size: u32)*

Example
```rust
site.daily_series("AAPL".to_string(), 100);
site.intraday_series("MSFT".to_string(), 200, Interval::Hour1);
site.weekly_series("GOOGL".to_string(), 50); 
```

**output_size** - Supports values in the range from 1 to 5000 , default is 30.

**interval** - for intraday series: 1min, 5min, 15min, 30min, 45min, 1h, 2h, 4h

Check The [Example](https://github.com/danrusei/market-data/blob/main/examples/series_twelvedata.rs)

## [Alpha Vantage](https://www.alphavantage.co/documentation/#time-series-data)

An API key is required, that can be obtain for free by [signing up on the site](https://www.alphavantage.co/support/#api-key).

Free stock API service covering the majority of the datasets for up to **25 requests per day**.  To target a larger API call volume, requires a premium membership.

### Build request

Select Alpha Vantage publisher:

```rust
let mut site = AlphaVantage::new(TOKEN.to_string());
```

For the requested instrument select the time series. Multiple can be added. 
The available methods:

* *intraday_series (symbol: String, output_size: OutputSize, interval: Interval)* - NOT yet supported, parsing issues
* *daily_series (symbol: String, output_size: OutputSize)*
* *weekly_series (symbol: String, output_size: OutputSize)*
* *monthly_series (symbol: String, output_size: OutputSize)*

Example:
```rust
site.daily_series("AAPL".to_string(), OutputSize::Compact);
site.weekly_series("GOOGL".to_string(), OutputSize::Compact);
site.intraday_series("MSFT".to_string(), OutputSize::Compact, Interval::Min60); -- not yet supported
```

**output_size** :

* **Compact** - compact returns only the latest 100 data points
* **Full** - full returns the full-length time series of 20+ years of historical data on Daily requests and trailing 30 days of the most recent intraday for Intraday Series

**interval** - for intraday series: 1min, 5min, 15min, 30min, 60min

Check The [Example](https://github.com/danrusei/market-data/blob/main/examples/series_alphavantage.rs)

## [IEX Cloud](https://iexcloud.io/docs/api/#historical-prices)

TODO