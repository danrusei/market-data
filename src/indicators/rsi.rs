use crate::EnhancedSeries;

pub(crate) fn calculate_rsi(data: &Vec<EnhancedSeries>, period: usize) -> Vec<f32> {
    let mut result: Vec<f32> = Vec::new();

    for (i, window) in data.windows(period + 1).enumerate() {
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

        result.push(rsi_value);
    }

    result
}
