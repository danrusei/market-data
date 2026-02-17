# Supported Publishers (Time-Series Data)

This library supports multiple market data providers. Most require an API token, which can usually be obtained via a free tier on their respective websites.

## Quick Comparison (Free Tiers - Feb 2026)

| Publisher | Daily Limit | Rate Limit | History | Key Requirement |
|-----------|-------------|------------|---------|-----------------|
| [Finnhub](#finnhub) | Uncapped* | 60 req/min | 1 year+ | API Token |
| [Alpha Vantage](#alphavantage) | 25 requests | - | 20+ years | API Token |
| [Polygon.io](#polygon_io) | Uncapped* | 5 req/min | 2 years | API Token |
| [Twelvedata](#twelvedata) | 800 credits | 8 req/min | Variable | API Token |
| [Yahoo Finance](#yahoo-finance) | Uncapped | - | Max | None (Unofficial) |

*\*Limits apply to the free tier; check the provider's website for the latest details.*

---

## [Finnhub](https://finnhub.io/docs/api)<a name="finnhub"></a>

Finnhub provides a robust API with generous rate limits for the free tier (60 requests/minute for US stocks).

### Implementation

```rust
let mut site = Finnhub::new(TOKEN.to_string());

// Unix timestamps are required for 'from' and 'to' parameters
site.daily_series("AAPL", 1672531200, 1675209600);
site.intraday_series("MSFT", 1672531200, 1675209600, Interval::Min15)?;
```

**Check the [Finnhub Example](https://github.com/danrusei/market-data/blob/main/examples/series_finnhub.rs)**

---

## [Alpha Vantage](https://www.alphavantage.co/documentation/#time-series-data)<a name="alphavantage"></a>

A veteran in the space, offering extensive historical data and technical indicators.

### Implementation

```rust
let mut site = AlphaVantage::new(TOKEN.to_string());

site.daily_series("AAPL", OutputSize::Compact);
site.weekly_series("GOOGL", OutputSize::Full);
```

**output_size**:

* **Compact**: Returns the latest 100 data points.
* **Full**: Returns the full-length time series (up to 20+ years).

**Check the [Alpha Vantage Example](https://github.com/danrusei/market-data/blob/main/examples/series_alphavantage.rs)**

---

## [Polygon.io](https://polygon.io/docs/stocks/get_v2_aggs_ticker__stocksticker__range__multiplier___timespan___from___to)<a name="polygon_io"></a>

Known for high-quality US equity data and a clean API.

### Implementation

```rust
let mut site = Polygon::new(APIKEY.to_string());

site.daily_series("GOOGL", "2024-01-01", "2024-03-01", 5000);
site.intraday_series("MSFT", "2024-03-06", "2024-03-06", Interval::Min30, 2000)?;
```

**Check the [Polygon.io Example](https://github.com/danrusei/market-data/blob/main/examples/series_polygon_io.rs)**

---

## [Twelvedata](https://twelvedata.com/docs#time-series)<a name="twelvedata"></a>

Offers a wide variety of assets and built-in technical indicators.

### Implementation

```rust
let mut site = Twelvedata::new(TOKEN.to_string());

site.daily_series("AAPL", 100);
site.intraday_series("MSFT", 200, Interval::Hour1);
```

**output_size**: Range from 1 to 5000 (default is 30).

**Check the [Twelvedata Example](https://github.com/danrusei/market-data/blob/main/examples/series_twelvedata.rs)**

---

## [Yahoo Finance](https://finance.yahoo.com/)<a name="yahoo-finance"></a>

The only provider in this list that doesn't require an API token. Useful for personal projects.

### Implementation

```rust
let mut site = YahooFin::new();

site.daily_series("GOOGL", YahooRange::Month6);
site.intraday_series("MSFT", Interval::Hour1, YahooRange::Day5)?;
```

**YahooRange**: `Day1`, `Day5`, `Month1`, `Month3`, `Month6`, `Year1`, `Year2`, `Year5`, `Year10`, `Ytd`, `Max`.

**Check the [Yahoo Finance Example](https://github.com/danrusei/market-data/blob/main/examples/series_yahoo_finance.rs)**
