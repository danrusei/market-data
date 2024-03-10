use crate::indicators::{ema::calculate_ema, rsi::calculate_rsi, sma::calculate_sma};
use crate::MarketSeries;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};
use std::fmt;

pub(crate) mod ema;
pub(crate) mod rsi;
pub(crate) mod sma;

/// Holds the MarketSeries + the calculation for the supported indicators
#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedMarketSeries {
    pub series: MarketSeries,
    pub asks: Vec<Ask>,
    pub indicators: Indicators,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ask {
    SMA(usize),
    EMA(usize),
    RSI(usize),
}

/// It is part of the EnhancedMarketSeries struct
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Indicators {
    // Simple Moving Average
    pub sma: BTreeMap<String, f32>,
    // Exponential Moving Average
    pub ema: BTreeMap<String, f32>,
    // Relative Strength Index
    pub rsi: BTreeMap<String, f32>,
}

impl EnhancedMarketSeries {
    /// Simple Moving Average, a period must be provided over which it will be calculated
    pub fn with_sma(mut self, period: usize) -> Self {
        self.asks.push(Ask::SMA(period));
        self
    }

    /// Exponential Moving Average, a period must be provided over which it will be calculated
    pub fn with_ema(mut self, period: usize) -> Self {
        self.asks.push(Ask::EMA(period));
        self
    }

    /// Relative Strength Index, a period must be provided over which it will be calculated
    pub fn with_rsi(mut self, period: usize) -> Self {
        self.asks.push(Ask::RSI(period));
        self
    }

    /// Calculate the indicators and populate within the EnhancedMarketSeries struct
    pub fn calculate(mut self) -> Self {
        let result: Vec<VecDeque<f32>> = self
            .indicators
            .iter()
            .map(|ind| match ind {
                Ask::SMA(period) => calculate_sma(&self.data, period.clone()),
                Ask::EMA(period) => calculate_ema(&self.data, period.clone()),
                Ask::RSI(period) => calculate_rsi(&self.data, period.clone()),
            })
            .collect();

        // populate EnhancedMarketSeries struct
        for (i, series) in self.data.iter_mut().enumerate() {
            for (j, ind) in self.indicators.iter().enumerate() {
                match ind {
                    // Assuming the order in self.indicators matches the order in result
                    Ask::SMA(value) => {
                        series.sma.insert(format!("SMA {}", value), result[j][i]);
                    }
                    Ask::EMA(value) => {
                        series.ema.insert(format!("EMA {}", value), result[j][i]);
                    }
                    Ask::RSI(value) => {
                        series.rsi.insert(format!("RSI {}", value), result[j][i]);
                    }
                }
            }
        }
        self
    }
}

impl fmt::Display for EnhancedMarketSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Symbol: {}, Indicators: {:?}, Data: [\n{}\n]",
            self.symbol,
            self.indicators,
            self.data
                .iter()
                .map(|series| format!("{}", series))
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }
}

impl fmt::Display for Ask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ask::SMA(period) => write!(f, "SMA({})", period),
            Ask::EMA(period) => write!(f, "EMA({})", period),
            Ask::RSI(period) => write!(f, "RSI({})", period),
        }
    }
}

// impl fmt::Display for EnhancedSeries {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "Date: {}, Open: {:.2}, Close: {:.2}, High: {:.2}, Low: {:.2}, Volume: {:.2},",
//             self.date, self.open, self.close, self.high, self.low, self.volume
//         )?;

//         for (key, value) in &self.sma {
//             write!(f, " {}: {:.2},", key, value)?;
//         }

//         for (key, value) in &self.ema {
//             write!(f, " {}: {:.2},", key, value)?;
//         }

//         for (key, value) in &self.rsi {
//             write!(f, " {}: {:.2}", key, value)?;
//         }

//         Ok(())
//     }
// }
