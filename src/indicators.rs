use crate::indicators::moving_averages::simple_moving_average;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub(crate) mod moving_averages;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EnhancedMarketSeries {
    pub symbol: String,
    pub data: Vec<EnhancedSeries>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EnhancedSeries {
    pub date: NaiveDate,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub volume: f32,
    // Simple Moving Average
    pub sma: Option<f32>,
    //Exponential Moving Average
    pub ema: Option<f32>,
}

impl EnhancedMarketSeries {
    fn sma(&mut self, window: usize) -> () {
        let averages = simple_moving_average(&self.data, window);
        assert!(averages.len() == self.data.len());
        averages
            .iter()
            .enumerate()
            .for_each(|(index, &item)| self.data[index + window].sma = Some(item));

        // TODO validate the logic !!!!
    }
}
