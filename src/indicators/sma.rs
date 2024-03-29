use crate::Series;
use std::collections::VecDeque;

// calculate based on:
// https://www.investopedia.com/terms/s/sma.asp
pub(crate) fn calculate_sma(series: &[Series], period: usize) -> VecDeque<f32> {
    let mut sma_values: VecDeque<f32> = series
        .windows(period)
        .map(|window| window.iter().map(|item| item.close).sum::<f32>() / period as f32)
        .collect();

    for _ in 1..period {
        sma_values.push_front(0.0);
    }

    assert!(sma_values.len() == series.len());

    sma_values
}
