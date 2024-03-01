mod iexcloud;

use crate::errors::MarketResult;
use chrono::NaiveDate;

pub trait DataRetrieval {
    fn create_endpoint(&mut self) -> MarketResult<()>;
    async fn get_data(&mut self) -> MarketResult<()>;
    fn transform_data(&self) -> MarketData;
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
