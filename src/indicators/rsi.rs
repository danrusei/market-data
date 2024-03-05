use std::collections::VecDeque;

use crate::EnhancedSeries;

// calculation based on:
// https://www.investopedia.com/terms/r/rsi.asp
pub(crate) fn calculate_rsi(series: &[EnhancedSeries], period: usize) -> VecDeque<f32> {
    let mut rsi_values: VecDeque<f32> = VecDeque::with_capacity(series.len());

    for (_, window) in series.windows(period + 1).enumerate() {
        let mut gains = 0.0;
        let mut losses = 0.0;

        for j in 1..window.len() {
            let price_change = window[j].close - window[j - 1].close;
            if price_change > 0.0 {
                gains += price_change;
            } else {
                losses += price_change.abs();
            }
        }

        let avg_gain = gains / period as f32;
        let avg_loss = losses / period as f32;

        let rsi_value = if avg_loss != 0.0 {
            100.0 - (100.0 / (1.0 + avg_gain / avg_loss))
        } else {
            100.0
        };

        rsi_values.push_back(rsi_value);
    }

    for _ in 1..period + 1 {
        rsi_values.push_front(0.0);
    }

    assert!(rsi_values.len() == series.len());

    rsi_values
}
