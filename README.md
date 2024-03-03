# market-data
A Rust Library to retrieve historical time-series stock data. 

## Usage

Check the [Example folder](https://github.com/danrusei/market-data/tree/master/examples) for more examples.

```rust
use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{Iex, MarketClient};
use std::env::var;

lazy_static! {
    static ref TOKEN: String = var("IEX_TOKEN").expect("IEX_TOKEN env variable is required");
}

fn main() -> Result<()> {

    // select a Publisher from where to download the market-data
    // most of them requires an API Key, that can be obtain by creating an account
    // check below the available Publishers
    let mut site = Iex::new(TOKEN.to_string());
    site.for_series("AAPL".to_string(), "3m".to_string());

    // use MarketClient to create the request, retrieve the data and transform into MarketData struct
    let mut client = MarketClient::new(site);
    client.create_endpoint()?;
    client.get_data()?;
    let data = client.transform_data();
    if let Some(data) = data {
        println!("{}", data);
    }
    Ok(())
}

```

## Supported Publishers

Trying to make the library easy to integrate new Publishers, therefore your contribution is welcome.
So far the following are supported.

* [x] [Iex cloud](https://iexcloud.io/docs/api/#rest-how-to)
* [] [Alpha Vantage](https://www.alphavantage.co/documentation/)
* [] [Finazon](https://finazon.io/docs/api/latest#)
* More to come

