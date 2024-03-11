use crate::indicators::{ema::calculate_ema, rsi::calculate_rsi, sma::calculate_sma};
use crate::{Interval, Series};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fmt;

pub(crate) mod ema;
pub(crate) mod rsi;
pub(crate) mod sma;

/// Holds the MarketSeries + the calculation for the supported indicators
#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedMarketSeries {
    pub symbol: String,
    pub interval: Interval,
    pub series: Vec<Series>,
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
    pub sma: HashMap<String, VecDeque<f32>>,
    // Exponential Moving Average
    pub ema: HashMap<String, VecDeque<f32>>,
    // Relative Strength Index
    pub rsi: HashMap<String, VecDeque<f32>>,
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
        let result: Vec<(Ask, VecDeque<f32>)> = self
            .asks
            .iter()
            .map(|ind| match ind {
                Ask::SMA(period) => calculate_sma(&self.series, period.clone()),
                Ask::EMA(period) => calculate_ema(&self.series, period.clone()),
                Ask::RSI(period) => calculate_rsi(&self.series, period.clone()),
            })
            .collect();

        for (ask, ind) in result.into_iter() {
            match ask {
                // Assuming the order in self.indicators matches the order in result
                Ask::SMA(value) => {
                    self.indicators.sma.insert(format!("SMA {}", value), ind);
                }
                Ask::EMA(value) => {
                    self.indicators.ema.insert(format!("EMA {}", value), ind);
                }
                Ask::RSI(value) => {
                    self.indicators.rsi.insert(format!("RSI {}", value), ind);
                }
            }
        }

        self
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

// impl fmt::Display for EnhancedMarketSeries {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "Symbol: {}, Requested Indicators: {:?}, Data: [\n{}\n]",
//             self.symbol,
//             self.asks,
//             self.series
//                 .iter()
//                 .map(|series| format!("{}", series))
//                 .collect::<Vec<_>>()
//                 .join(",\n")
//         )
//     }
// }

impl fmt::Display for EnhancedMarketSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EnhancedMarketSerie: Symbol = {}, Interval = {},  Requested Indicators: {:?}, Series: \n",
            self.symbol, self.interval, self.asks
        )?;
        for (i, series) in self.series.iter().enumerate() {
            write!(
                f,
                "Date: {}, Open: {:.2}, Close: {:.2}, High: {:.2}, Low: {:.2}, Volume: {:.2}, ",
                series.date, series.open, series.close, series.high, series.low, series.volume
            )?;

            for (indicator_name, indicator_values) in &self.indicators.sma {
                if let Some(value) = indicator_values.get(i) {
                    write!(f, "{}: {:.2}, ", indicator_name, value)?;
                }
            }

            for (indicator_name, indicator_values) in &self.indicators.ema {
                if let Some(value) = indicator_values.get(i) {
                    write!(f, "{}: {:.2}, ", indicator_name, value)?;
                }
            }

            for (indicator_name, indicator_values) in &self.indicators.rsi {
                if let Some(value) = indicator_values.get(i) {
                    write!(f, "{}: {:.2}, ", indicator_name, value)?;
                }
            }

            // Remove trailing comma and space
            //let _ = f.write_str("\b\b");

            if i < self.series.len() - 1 {
                writeln!(f, ",")?;
            }
        }

        Ok(())
    }
}
