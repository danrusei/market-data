use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{Iex, MarketClient};
use std::env::var;

lazy_static! {
    static ref TOKEN: String = var("IEX_TOKEN").expect("IEX_TOKEN env variable is required");
}

fn main() -> Result<()> {
    let mut site = Iex::new();
    site.with_config(TOKEN.to_string(), "AAPL".to_string(), "2y".to_string());
    let mut client = MarketClient::new(site);
    client.create_endpoint()?;
    client.get_data()?;
    Ok(())
}
