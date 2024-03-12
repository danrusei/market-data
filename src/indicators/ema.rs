use crate::Series;
use std::collections::VecDeque;

// calculated based on:
// https://www.investopedia.com/ask/answers/122314/what-exponential-moving-average-ema-formula-and-how-ema-calculated.asp
pub(crate) fn calculate_ema(series: &[Series], period: usize) -> VecDeque<f32> {
    let alpha = 2.0 / (period as f32 + 1.0);
    let mut ema_values: VecDeque<f32> = VecDeque::with_capacity(series.len());
    let mut ema_prev: Option<f32> = None;

    for item in series {
        let close_price = item.close;
        match ema_prev {
            Some(prev) => {
                let ema = alpha * close_price + (1.0 - alpha) * prev;
                ema_values.push_back(ema);
                ema_prev = Some(ema);
            }
            None => {
                ema_values.push_back(close_price);
                ema_prev = Some(close_price);
            }
        }
    }

    assert!(ema_values.len() == series.len());

    ema_values
}

pub(crate) fn calculate_ema_slice(series: &[f32], period: usize) -> VecDeque<f32> {
    let alpha = 2.0 / (period as f32 + 1.0);
    let mut ema_values: VecDeque<f32> = VecDeque::with_capacity(series.len());
    let mut ema_prev: Option<f32> = None;

    for item in series {
        match ema_prev {
            Some(prev) => {
                let ema = alpha * item + (1.0 - alpha) * prev;
                ema_values.push_back(ema);
                ema_prev = Some(ema);
            }
            None => {
                ema_values.push_back(*item);
                ema_prev = Some(*item);
            }
        }
    }

    assert!(ema_values.len() == series.len());

    ema_values
}
