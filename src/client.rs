use crate::publishers::{iexcloud::Iex, Publisher};
use chrono::NaiveDate;

pub struct MarketClient {
    inner: Box<dyn Publisher>,
}

pub enum Site {
    Iex,
}

impl MarketClient {
    pub fn new(site: Site) -> Self {
        let inner = match site {
            Site::Iex => Box::new(Iex::new()),
        };
        MarketClient { inner }
    }
    pub fn with_config(&self, token: String, symbol: String, range: String) -> () {
        self.inner.with_config(token, symbol, range);
    }
}

pub struct MarketData {
    symbol: String,
    data: Vec<Series>,
}

pub struct Series {
    date: NaiveDate,
    open: f32,
    close: f32,
    high: f32,
    low: f32,
    volume: f32,
}
