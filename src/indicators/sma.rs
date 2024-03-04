use crate::EnhancedSeries;

pub(crate) fn calculate_sma(series: &Vec<EnhancedSeries>, period: usize) -> Vec<f32> {
    let moving_averages: Vec<f32> = series
        .windows(period)
        .map(|window| window.iter().map(|item| item.close).sum::<f32>() / period as f32)
        .collect();

    moving_averages
}
