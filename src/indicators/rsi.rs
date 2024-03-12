use crate::{indicators::Ask, Series};
use std::collections::VecDeque;

// calculation based on:
// https://www.investopedia.com/terms/r/rsi.asp
pub(crate) fn calculate_rsi(series: &[Series], period: usize) -> (Ask, VecDeque<f32>) {
    let mut gain_sum = 0.0;
    let mut loss_sum = 0.0;

    // Calculate initial average gain and loss
    for i in 1..=period {
        let price_diff = series[i].close - series[i - 1].close;
        if price_diff > 0.0 {
            gain_sum += price_diff;
        } else {
            loss_sum += price_diff.abs();
        }
    }

    let mut rsi_values = VecDeque::new();

    // Calculate RSI for the remaining data
    for i in period..series.len() {
        let price_diff = series[i].close - series[i - 1].close;

        if price_diff > 0.0 {
            gain_sum += price_diff;
        } else {
            loss_sum += price_diff.abs();
        }

        // Calculate average gain and loss
        let avg_gain = gain_sum / period as f32;
        let avg_loss = loss_sum / period as f32;

        // Calculate relative strength (RS)
        let rs = if avg_loss != 0.0 {
            avg_gain / avg_loss
        } else {
            f32::INFINITY
        };

        // Calculate RSI
        let rsi = 100.0 - (100.0 / (1.0 + rs));

        rsi_values.push_back(rsi);

        // Update gain and loss sums for the next iteration
        gain_sum -= gain_sum / period as f32;
        loss_sum -= loss_sum / period as f32;
    }

    for _ in 1..period + 1 {
        rsi_values.push_front(0.0);
    }

    assert!(rsi_values.len() == series.len());

    (Ask::RSI(period), rsi_values)
}
