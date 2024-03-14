# Supported Publishers (time-series data only)

* [Twelvedata](#twelvedata)
* [Alpha Vantage](#alphavantage)
* [Poligon.io](#polygon_io)
* [IexCloud](#iex-cloud)
* [Yahoo Finance](#yahoo-finance)

## [Twelvedata](https://twelvedata.com/docs#time-series)<a name="twelvedata"></a>

An API key is required, that can be obtained for free by signing up on the site.

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
site.daily_series("AAPL", 100);
site.intraday_series("MSFT", 200, Interval::Hour1);
site.weekly_series("GOOGL", 50); 
```

**output_size** - Supports values in the range from 1 to 5000 , default is 30.

**interval** - for intraday series: 1min, 5min, 15min, 30min, 45min, 1h, 2h, 4h

Check The [Example](https://github.com/danrusei/market-data/blob/main/examples/series_twelvedata.rs)

## [Alpha Vantage](https://www.alphavantage.co/documentation/#time-series-data)<a name="alphavantage"></a>

An API key is required, that can be obtained for free by [signing up on the site](https://www.alphavantage.co/support/#api-key).

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
site.daily_series("AAPL", OutputSize::Compact);
site.weekly_series("GOOGL", OutputSize::Compact);
site.intraday_series("MSFT", OutputSize::Compact, Interval::Min60); -- not yet supported
```

**output_size** :

* **Compact** - compact returns only the latest 100 data points
* **Full** - full returns the full-length time series of 20+ years of historical data on Daily requests and trailing 30 days of the most recent intraday for Intraday Series

**interval** - for intraday series: 1min, 5min, 15min, 30min, 60min

Check The [Example](https://github.com/danrusei/market-data/blob/main/examples/series_alphavantage.rs)

## [Polygon.io](https://polygon.io/docs/stocks/get_v2_aggs_ticker__stocksticker__range__multiplier___timespan___from___to)<a name="polygon_io"></a>

An API key is required, that can be obtained for free by [signing up on the site](https://polygon.io/).

It offers a [Free Basic plan](https://polygon.io/pricing), that offers 5 API Calls / minute with 2 Years Historical Data End of Day Data. For increased number of API calls, 15-minute delayed data and longer timespan for historical data a paid subscription is needed.

[The base aggregates are minute and daily bars](https://polygon.io/blog/aggs-api-updates), while a request for any other resolution results in those bars being calculated from one of the base aggregates.
The following illustrates the mappings of requested resolutions to their corresponding base aggregate.

The `limit` parameter, limits the number of base aggregates queried to create the aggregate results. Max 50000 and Default 5000. Read more about how limit is used to calculate aggregate results [in the article](https://polygon.io/blog/aggs-api-updates).

| Minute |	Day |
|--------|------|
|Minute, Hour |	Day, Week, Month, Quarter, Year |

### Build request

Select Polygon_io publisher:

```rust
let mut site = Polygon::new(APIKEY.to_string());
```

For the requested instrument select the time series. Multiple can be added. 
The available methods:

* *intraday_series (symbol: String, from_date: String, to_date: String, interval: Interval, limit: i32)* 
* *daily_series (symbol: String, from_date: String, to_date: String, limit: i32)*
* *weekly_series (symbol: String, from_date: String, to_date: String, limit: i32)*
* *monthly_series (symbol: String, from_date: String, to_date: String, limit: i32)*

Example:

```rust
site.daily_series("GOOGL", "2024-01-01", "2024-03-01", 200);
site.weekly_series("GOOGL", "2023-01-07", "2024-01-07", 1000);
site.intraday_series("MSFT", "2024-03-06", "2024-03-06", Interval::Min30, 2000)?;
```

**interval** - Interval::Min1, Interval::Min5, Interval::Min15, Interval::Min30, Interval::Hour1, Interval::Hour2, Interval::Hour4
**limit** -- read the above mention

Check The [Example](https://github.com/danrusei/market-data/blob/main/examples/series_polygon_io.rs)

## [IEX Cloud](https://iexcloud.io/docs/api/#historical-prices)<a name="iex-cloud"></a> 

Their API and the subscription model has changed, therefeore the below examples may not work without a paid subscribtion !

Select Iex Cloud publisher:

```rust
let mut site = Iex::new(TOKEN.to_string());
```

For the requested instrument select the time series. Multiple can be added. 
The available methods:

* *daily_series (symbol: String, range: String)*

Example:

```rust
site.daily_series("AAPL", "3m".to_string());
```

**range** - supported values : 1m (default), 3m, 6m, ytd, 1y, 2y, 5y, max (available data up to 15 years)

Check The [Example](https://github.com/danrusei/market-data/blob/main/examples/series_iexcloud.rs)

## [Yahoo Finance](https://finance.yahoo.com/)<a name="yahoo-finance"></a>

The Yahoo Finance API is free to use for personal projects. However, commercial usage of the API requires a paid subscription. This means that developers working on commercial projects will need to pay for a Yahoo Finance API subscription.

The Yahoo Finance API is updated once per day. Does not offer real-time data as per the moment I'm writing this.

### Build request

Select Alpha Vantage publisher:

```rust
let mut site = YahooFin::new();
```

For the requested instrument select the time series. Multiple can be added. 
The available methods:

* *intraday_series (symbol: String, interval: Interval, range: YahooRange)* 
* *daily_series (symbol: String, range: YahooRange)*
* *weekly_series (symbol: String, range: YahooRange)*
* *monthly_series (symbol: String, range: YahooRange)*

Example:

```rust
site.daily_series("GOOGL", YahooRange::Month6);
site.weekly_series("GOOGL", YahooRange::Year1);
site.intraday_series("MSFT", Interval::Hour1, YahooRange::Day5)?;
```

**Interval** - used for intraday series: Interval::Min1, Interval::Min5, Interval::Min15, Interval::Min30, Interval::Hour1, Interval::Hour2, Interval::Hour4

**YahooRange** - the calid ranges are: YahooRange:Day1, YahooRange:Day5, YahooRange:Month1, YahooRange:Month3,YahooRange:Month6, YahooRange:Year1, YahooRange:Year2, YahooRange:Year5, YahooRange:Year10, YahooRange:Ytd, YahooRange:Max,

Check The [Example](https://github.com/danrusei/market-data/blob/main/examples/series_yahoo_finance.rs)
