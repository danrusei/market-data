# Supported Publishers (Time-Series Data)

This library supports multiple market data providers using a stateless, async-first architecture.

## Quick Comparison (Free Tiers - Feb 2026)

| Publisher | Daily Limit | Rate Limit | History | Key Requirement |
|-----------|-------------|------------|---------|-----------------|
| [Finnhub](#finnhub) | Uncapped* | 60 req/min | 1 year+ | API Token |
| [Alpha Vantage](#alphavantage) | 25 requests | - | 20+ years | API Token |
| [Massive](#massive) | Uncapped* | 5 req/min | 2 years | API Token |
| [Twelvedata](#twelvedata) | 800 credits | 8 req/min | Variable | API Token |
| [Yahoo Finance](#yahoo-finance) | Uncapped | - | Max | None (Unofficial) |

*\*Limits apply to the free tier; check the provider's website for the latest details.*

---

## Core Architecture

The library uses a stateless `MarketClient` that works with any implementation of the `Publisher` trait.

```rust
let site = Finnhub::new(TOKEN);
let client = MarketClient::new(site);

let request = client.site.daily_series("AAPL", from_timestamp, to_timestamp);
let series = client.fetch(request).await?;
```

---

## [Finnhub](https://finnhub.io/docs/api)<a name="finnhub"></a>

### Implementation

```rust
let site = Finnhub::new(TOKEN);
let client = MarketClient::new(site);

// Unix timestamps are required for 'from' and 'to' parameters
let request = client.site.daily_series("AAPL", 1672531200, 1675209600);
let series = client.fetch(request).await?;
```

**Check the [Finnhub Example](https://github.com/danrusei/market-data/blob/main/examples/series_finnhub.rs)**

---

## [Alpha Vantage](https://www.alphavantage.co/documentation/#time-series-data)<a name="alphavantage"></a>

### Implementation

```rust
let site = AlphaVantage::new(TOKEN);
let client = MarketClient::new(site);

let request = client.site.daily_series("AAPL", OutputSize::Compact);
let series = client.fetch(request).await?;
```

**Check the [Alpha Vantage Example](https://github.com/danrusei/market-data/blob/main/examples/series_alphavantage.rs)**

---

## [Massive](https://massive.com/docs/rest/stocks/aggregates/custom-bars)<a name="massive"></a>

Formerly Polygon.io. Known for high-quality US equity data and a clean API.

### Implementation

```rust
let site = Massive::new(APIKEY);
let client = MarketClient::new(site);

let request = client.site.daily_series("GOOGL", "2024-01-01", "2024-03-01", 5000);
let series = client.fetch(request).await?;
```

**Check the [Massive Example](https://github.com/danrusei/market-data/blob/main/examples/series_massive.rs)**

---

## [Twelvedata](https://twelvedata.com/docs#time-series)<a name="twelvedata"></a>

### Implementation

```rust
let site = Twelvedata::new(TOKEN);
let client = MarketClient::new(site);

let request = client.site.daily_series("AAPL", 100);
let series = client.fetch(request).await?;
```

**Check the [Twelvedata Example](https://github.com/danrusei/market-data/blob/main/examples/series_twelvedata.rs)**

---

## [Yahoo Finance](https://finance.yahoo.com/)<a name="yahoo-finance"></a>

### Implementation

```rust
let site = YahooFin::new();
let client = MarketClient::new(site);

let request = client.site.daily_series("GOOGL", YahooRange::Month6);
let series = client.fetch(request).await?;
```

**Check the [Yahoo Finance Example](https://github.com/danrusei/market-data/blob/main/examples/series_yahoo_finance.rs)**
