use lazy_static::lazy_static;
use market_data::{MarketClient, Site};
use std::env::var;

lazy_static! {
    static ref TOKEN: String = var("IEX_TOKEN").expect("IEX_TOKEN env variable is required");
}

#[tokio::main]
async fn main() {
    let site = Site::Iex;
    let client = MarketClient::new(site);
    client.with_config(TOKEN, "AAPL".to_string(), "2Y".to_string());
}
