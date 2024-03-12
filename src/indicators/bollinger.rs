use crate::Series;
use std::collections::VecDeque;

use super::ema::calculate_ema;

// calculation based on:
// https://www.investopedia.com/terms/b/bollingerbands.asp

pub(crate) fn calculate_bollinger_bands(
    series: &[Series],
    period: usize,
    std_dev: usize,
) -> (VecDeque<f32>, VecDeque<f32>, VecDeque<f32>) {
    if series.len() <= period {
        return (VecDeque::new(), VecDeque::new(), VecDeque::new());
    }

    let middle_band_values = calculate_ema(&series, period);

    let mut upper_band_values = VecDeque::new();
    let mut lower_band_values = VecDeque::new();

    // Iterate over the series to calculate upper and lower bands
    for i in period..series.len() {
        // Calculate typical price (TP)
        let typical_price = (series[i].high + series[i].low + series[i].close) / 3.0;

        // Calculate standard deviation over last n periods of TP
        let sum_squares: f32 = series[i - period + 1..=i]
            .iter()
            .map(|s| ((s.high + s.low + s.close) / 3.0 - typical_price).powi(2))
            .sum();
        let std_deviation = (sum_squares / period as f32).sqrt();

        // Calculate upper and lower bands
        let upper_band = middle_band_values.back().unwrap_or(&0.0) + std_dev as f32 * std_deviation;
        let lower_band = middle_band_values.back().unwrap_or(&0.0) - std_dev as f32 * std_deviation;

        // Store upper and lower bands in the respective vectors
        upper_band_values.push_back(upper_band);
        lower_band_values.push_back(lower_band);
    }

    for _ in 1..period + 1 {
        upper_band_values.push_front(0.0);
        lower_band_values.push_front(0.0);
    }

    assert!((series.len() == upper_band_values.len()) && (series.len() == lower_band_values.len()));

    (upper_band_values, middle_band_values, lower_band_values)
}
