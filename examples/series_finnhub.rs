use anyhow::Result;
use lazy_static::lazy_static;
use market_data::{Finnhub, MarketClient};
use std::env::var;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref TOKEN: String =
        var("Finnhub_TOKEN").expect("Finnhub_TOKEN env variable is required");
}

fn main() -> Result<()> {
    // Select a Publisher from the available ones
    let mut site = Finnhub::new(TOKEN.to_string());

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64;
    let month_ago = now - 30 * 24 * 60 * 60;

    // configure to retrieve Daily, Weekly or Intraday series
    site.daily_series("AAPL", month_ago, now);
    site.weekly_series("GOOGL", month_ago, now);

    // create the MarketClient
    let client = MarketClient::new(site);

    // creates the query URL & download the raw data
    let mut client = client.create_endpoint()?.get_data()?;
    // transform into MarketSeries, that can be used for further processing
    let data = client.transform_data();

    // print the data
    data.iter().for_each(|output| match output {
        Ok(data) => println!("{}

", data),
        Err(err) => println!("{}", err),
    });

    Ok(())
}
