use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{Iex, MarketClient};
use std::env::var;

lazy_static! {
    static ref TOKEN: String = var("IEX_TOKEN").expect("IEX_TOKEN env variable is required");
}

fn main() -> Result<()> {
    let mut site = AlphaVantage::new(TOKEN.to_string());
    site.for_series("AAPL".to_string(), "3m".to_string());

    let mut client = MarketClient::new(site);
    client.create_endpoint()?;
    client.get_data()?;
    let data = client.transform_data();
    if let Some(data) = data {
        println!("{}", data);
    }
    Ok(())
}
