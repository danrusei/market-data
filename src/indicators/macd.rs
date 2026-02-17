use crate::indicators::{calculate_ema, calculate_ema_slice};
use crate::Series;
use std::collections::VecDeque;

pub(crate) fn calculate_macd(
    series: &[Series],
    fast: usize,
    slow: usize,
    signal: usize,
) -> (VecDeque<f32>, VecDeque<f32>, VecDeque<f32>) {
    if series.len() <= slow || fast >= slow || signal >= slow {
        return (VecDeque::new(), VecDeque::new(), VecDeque::new());
    }

    // Calculate fast and slow EMA
    let fast_ema = calculate_ema(series, fast);
    let slow_ema = calculate_ema(series, slow);

    // Calculate MACD line
    let mut macd_line: VecDeque<f32> = fast_ema
        .iter()
        .zip(slow_ema.iter())
        .map(|(fast_val, slow_val)| fast_val - slow_val)
        .collect();

    // Calculate Signal line (EMA over MACD)
    let signal_line = calculate_ema_slice(macd_line.make_contiguous(), signal);

    // Calculate Histogram
    let histogram: VecDeque<f32> = macd_line
        .iter()
        .zip(signal_line.iter())
        .map(|(macd_val, signal_val)| macd_val - signal_val)
        .collect();

    assert!((series.len() == macd_line.len()) && (series.len() == signal_line.len()));

    (macd_line, signal_line, histogram)
}
