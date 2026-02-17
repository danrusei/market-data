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

    // Enhance the data with indicators
    let enhanced_data = data
        .enhance_data()
        .with_sma(10)
        .with_ema(20)
        .with_rsi(14)
        .calculate();

    // Print the enhanced data
    println!("{}", enhanced_data);

    Ok(())
}
