use anyhow::Result;
use market_data::{MarketClient, YahooFin, YahooRange};

#[tokio::main]
async fn main() -> Result<()> {
    // Select a Publisher
    let site = YahooFin::new();
    // Create the MarketClient
    let client = MarketClient::new(site);

    // Create a request
    let request = client.site.daily_series("AAPL", YahooRange::Month6);

    // Fetch the data
    let data = client.fetch(request).await?;

    // Print the data
    println!("{}", data);

    Ok(())
}
