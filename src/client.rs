use crate::{errors::MarketResult, publishers::Publisher};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub data: Vec<Series>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub date: String,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub volume: f32,
}
