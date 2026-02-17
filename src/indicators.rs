use crate::{Interval, Series};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fmt;

use self::{
    bollinger::calculate_bollinger_bands,
    ema::{calculate_ema, calculate_ema_slice},
    macd::calculate_macd,
    rsi::calculate_rsi,
    sma::calculate_sma,
    stochastic::calculate_stochastic,
};

pub(crate) mod bollinger;
pub(crate) mod ema;
pub(crate) mod macd;
pub(crate) mod rsi;
pub(crate) mod sma;
pub(crate) mod stochastic;

/// Type alias for indicators that return three values (e.g., MACD, Bollinger Bands)
pub type TripleIndicatorData = (VecDeque<f32>, VecDeque<f32>, VecDeque<f32>);

/// Holds the MarketSeries + the calculation for the supported indicators
#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedMarketSeries {
    /// holds symbol like: "GOOGL"
    pub symbol: String,
    /// inteval from intraday to monthly
    pub interval: Interval,
    /// the original series downloaded and parsed from publishers
    pub series: Vec<Series>,
    /// the request for technical indicators
    pub asks: Vec<Ask>,
    /// calculated indicators
    pub indicators: Indicators,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ask {
    Sma(usize),
    Ema(usize),
    Rsi(usize),
    Stochastic(usize),
    Macd(usize, usize, usize),
    Bb(usize, usize),
}

/// It is part of the EnhancedMarketSeries struct
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Indicators {
    /// Simple Moving Average
    pub sma: HashMap<String, VecDeque<f32>>,
    /// Exponential Moving Average
    pub ema: HashMap<String, VecDeque<f32>>,
    /// Relative Strength Index
    pub rsi: HashMap<String, VecDeque<f32>>,
    ///  Stochastic Oscillator
    pub stochastic: HashMap<String, VecDeque<f32>>,
    /// Moving average convergence/divergence (MACD)
    pub macd: HashMap<String, TripleIndicatorData>,
    /// Bollinger Band (BB)
    pub bb: HashMap<String, TripleIndicatorData>,
}

impl EnhancedMarketSeries {
    /// Simple Moving Average, a period must be provided over which it will be calculated
    pub fn with_sma(mut self, period: usize) -> Self {
        self.asks.push(Ask::Sma(period));
        self
    }

    /// Exponential Moving Average, a period must be provided over which it will be calculated
    pub fn with_ema(mut self, period: usize) -> Self {
        self.asks.push(Ask::Ema(period));
        self
    }

    /// Relative Strength Index, a period must be provided over which it will be calculated
    pub fn with_rsi(mut self, period: usize) -> Self {
        self.asks.push(Ask::Rsi(period));
        self
    }

    /// Stochastic Oscillator, a period must be provided over which it will be calculated
    pub fn with_stochastic(mut self, period: usize) -> Self {
        self.asks.push(Ask::Stochastic(period));
        self
    }

    /// Moving average convergence/divergence (MACD), a fast, slow & signal EMA values should be provided, default (12, 26, 9)
    pub fn with_macd(mut self, fast: usize, slow: usize, signal: usize) -> Self {
        self.asks.push(Ask::Macd(fast, slow, signal));
        self
    }

    /// Bollinger Bands (BB), the period & standard deviation values should be provided, like (20, 2)
    pub fn with_bb(mut self, period: usize, std_dev: usize) -> Self {
        self.asks.push(Ask::Bb(period, std_dev));
        self
    }

    /// Calculate the indicators and populate within the EnhancedMarketSeries struct
    pub fn calculate(mut self) -> Self {
        let series = self.series.clone();
        for ind in self.asks.iter() {
            match ind {
                Ask::Sma(period) => {
                    let calc_sma = calculate_sma(&series, *period);
                    self.indicators
                        .sma
                        .insert(format!("SMA {}", period), calc_sma);
                }

                Ask::Ema(period) => {
                    let calc_ema = calculate_ema(&series, *period);
                    self.indicators
                        .ema
                        .insert(format!("EMA {}", period), calc_ema);
                }

                Ask::Rsi(period) => {
                    let calc_rsi = calculate_rsi(&series, *period);
                    self.indicators
                        .rsi
                        .insert(format!("RSI {}", period), calc_rsi);
                }

                Ask::Stochastic(period) => {
                    let calc_stoch = calculate_stochastic(&series, *period);
                    self.indicators
                        .stochastic
                        .insert(format!("STO {}", period), calc_stoch);
                }

                Ask::Macd(fast, slow, signal) => {
                    let (calc_macd, calc_signal, calc_histogram) =
                        calculate_macd(&series, *fast, *slow, *signal);

                    self.indicators.macd.insert(
                        format!("MACD ({}, {}, {})", fast, slow, signal),
                        (calc_macd, calc_signal, calc_histogram),
                    );
                }
                Ask::Bb(period, std_dev) => {
                    let (upper_band, mid_band, lower_band) =
                        calculate_bollinger_bands(&series, *period, *std_dev);

                    self.indicators.bb.insert(
                        format!("BB ({}, {})", period, std_dev),
                        (upper_band, mid_band, lower_band),
                    );
                }
            }
        }

        self
    }
}

impl fmt::Display for Ask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ask::Sma(period) => write!(f, "SMA({})", period),
            Ask::Ema(period) => write!(f, "EMA({})", period),
            Ask::Rsi(period) => write!(f, "RSI({})", period),
            Ask::Macd(fast, slow, signal) => write!(f, "MACD({}, {}, {})", fast, slow, signal),
            Ask::Stochastic(period) => write!(f, "STO({})", period),
            Ask::Bb(period, std_dev) => write!(f, "BB({}, {})", period, std_dev),
        }
    }
}

impl fmt::Display for EnhancedMarketSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "EnhancedMarketSerie: Symbol = {}, Interval = {},  Requested Indicators: {:?}, Series: ",
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

            for (indicator_name, indicator_values) in &self.indicators.stochastic {
                if let Some(value) = indicator_values.get(i) {
                    write!(f, "{}: {:.2}, ", indicator_name, value)?;
                }
            }

            for (indicator_name, (macd, signal, histogram)) in &self.indicators.macd {
                if let Some(macd_value) = macd.get(i) {
                    if let Some(signal_value) = signal.get(i) {
                        if let Some(hist_value) = histogram.get(i) {
                            write!(
                                f,
                                "{}: {:.2}, {:.2}, {:.2}, ",
                                indicator_name, macd_value, signal_value, hist_value
                            )?;
                        }
                    }
                }
            }

            for (indicator_name, (upper_band, mid_band, lower_band)) in &self.indicators.bb {
                if let Some(upper_band) = upper_band.get(i) {
                    if let Some(mid_band) = mid_band.get(i) {
                        if let Some(lower_band) = lower_band.get(i) {
                            write!(
                                f,
                                "{}: {:.2}, {:.2}, {:.2}, ",
                                indicator_name, upper_band, mid_band, lower_band
                            )?;
                        }
                    }
                }
            }

            if i < self.series.len() - 1 {
                writeln!(f, ",")?;
            }
        }

        Ok(())
    }
}
