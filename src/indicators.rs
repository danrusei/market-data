use crate::indicators::{ema::calculate_ema, rsi::calculate_rsi, sma::calculate_sma};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};
use std::fmt;

pub(crate) mod ema;
pub(crate) mod rsi;
pub(crate) mod sma;

/// Holds the MarketSeries + the calculation for the supported indicators
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EnhancedMarketSeries {
    pub symbol: String,
    pub indicators: Vec<Indicator>,
    pub data: Vec<EnhancedSeries>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Indicator {
    SMA(usize),
    EMA(usize),
    RSI(usize),
}

/// It is part of the EnhancedMarketSeries struct
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EnhancedSeries {
    pub date: NaiveDate,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub volume: f32,
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
        self.indicators.push(Indicator::SMA(period));
        self
    }

    /// Exponential Moving Average, a period must be provided over which it will be calculated
    pub fn with_ema(mut self, period: usize) -> Self {
        self.indicators.push(Indicator::EMA(period));
        self
    }

    /// Relative Strength Index, a period must be provided over which it will be calculated
    pub fn with_rsi(mut self, period: usize) -> Self {
        self.indicators.push(Indicator::RSI(period));
        self
    }

    /// Calculate the indicators and populate within the EnhancedMarketSeries struct
    pub fn calculate(mut self) -> Self {
        let result: Vec<VecDeque<f32>> = self
            .indicators
            .iter()
            .map(|ind| match ind {
                Indicator::SMA(period) => calculate_sma(&self.data, period.clone()),
                Indicator::EMA(period) => calculate_ema(&self.data, period.clone()),
                Indicator::RSI(period) => calculate_rsi(&self.data, period.clone()),
            })
            .collect();

        // populate EnhancedMarketSeries struct
        for (i, series) in self.data.iter_mut().enumerate() {
            for (j, ind) in self.indicators.iter().enumerate() {
                match ind {
                    // Assuming the order in self.indicators matches the order in result
                    Indicator::SMA(value) => {
                        series.sma.insert(format!("SMA {}", value), result[j][i]);
                    }
                    Indicator::EMA(value) => {
                        series.ema.insert(format!("EMA {}", value), result[j][i]);
                    }
                    Indicator::RSI(value) => {
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

impl fmt::Display for EnhancedSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Date: {}, Open: {:.2}, Close: {:.2}, High: {:.2}, Low: {:.2}, Volume: {:.2},",
            self.date, self.open, self.close, self.high, self.low, self.volume
        )?;

        for (key, value) in &self.sma {
            write!(f, " {}: {:.2},", key, value)?;
        }

        for (key, value) in &self.ema {
            write!(f, " {}: {:.2},", key, value)?;
        }

        for (key, value) in &self.rsi {
            write!(f, " {}: {:.2}", key, value)?;
        }

        Ok(())
    }
}

impl fmt::Display for Indicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Indicator::SMA(period) => write!(f, "SMA({})", period),
            Indicator::EMA(period) => write!(f, "EMA({})", period),
            Indicator::RSI(period) => write!(f, "RSI({})", period),
        }
    }
}
