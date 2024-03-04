use crate::indicators::{rsi::calculate_rsi, sma::calculate_sma};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub(crate) mod rsi;
pub(crate) mod sma;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EnhancedMarketSeries {
    pub symbol: String,
    pub indicators: Vec<Option<Indicator>>,
    pub data: Vec<EnhancedSeries>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Indicator {
    SMA(usize),
    RSI(usize),
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
    pub rsi: f32,
}

impl EnhancedMarketSeries {
    pub fn with_sma(&mut self, period: usize) -> () {
        self.indicators.push(Some(Indicator::SMA(period)));
    }
    pub fn with_rsi(&mut self, period: usize) -> () {
        self.indicators.push(Some(Indicator::RSI(period)));
    }
    //TODO consolidate within a single function
    //and iterate only one single time over struct to add calculated data
    fn sma(&mut self, period: usize) -> () {
        let averages = calculate_sma(&self.data, period);
        assert!(averages.len() == self.data.len());
        averages
            .iter()
            .enumerate()
            .for_each(|(index, &item)| self.data[index + period].sma = Some(item));

        // TODO validate the logic !!!!
    }
    fn rsi(&mut self, period: usize) -> () {
        let result_rsi = calculate_rsi(&self.data, period);
        assert!(result_rsi.len() == self.data.len());
    }
}
