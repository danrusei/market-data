use crate::{errors::MarketResult, publishers::Publisher};
use chrono::NaiveDate;

pub struct MarketClient<T: Publisher> {
    inner: T,
}

impl<T: Publisher> MarketClient<T> {
    pub fn new(site: T) -> Self {
        MarketClient { inner: site }
    }
    pub fn create_endpoint(&mut self) -> MarketResult<()> {
        self.inner.create_endpoint()?;
        Ok(())
    }
    pub fn get_data(&mut self) -> MarketResult<()> {
        self.inner.get_data()?;
        Ok(())
    }
    pub fn transform_data(&self) -> MarketResult<()> {
        let _data = self.inner.transform_data();
        // Here TODO something wth transformed Data
        Ok(())
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
