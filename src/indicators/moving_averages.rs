use crate::EnhancedSeries;

pub(crate) fn simple_moving_average(series: &Vec<EnhancedSeries>, window: usize) -> Vec<f32> {
    let moving_averages: Vec<f32> = series
        .windows(window)
        .map(|window| window.iter().map(|item| item.close).sum())
        .collect();

    moving_averages
}
