use crate::Series;
use std::collections::VecDeque;

// calculation based on:
// https://www.investopedia.com/terms/r/rsi.asp
pub(crate) fn calculate_rsi(series: &[Series], period: usize) -> VecDeque<f32> {
    if series.len() <= period {
        return VecDeque::new();
    }

    // Wilder RSI with TA-Lib seeding; warm-up bars are NaN, not 0.0
    let closes: Vec<f64> = series.iter().map(|s| s.close as f64).collect();
    fin_primitives::signals::batch_f64::rsi(&closes, period)
        .into_iter()
        .map(|v| v as f32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rsi_matches_wilder_reference() {
        // StockCharts 14-period RSI sample, Wilder smoothing computed exactly
        let closes = [
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.61, 46.28, 46.28, 46.00, 46.03, 46.41, 46.22, 45.64,
        ];
        let series: Vec<Series> = closes
            .iter()
            .map(|&close| Series {
                datetime: Default::default(),
                open: close,
                close,
                high: close,
                low: close,
                volume: 0.0,
            })
            .collect();

        let rsi = calculate_rsi(&series, 14);
        assert_eq!(rsi.len(), series.len());
        assert!(rsi.iter().take(14).all(|v| v.is_nan()));
        let expected = [70.46, 66.25, 66.48, 69.35, 66.29, 57.92];
        for (got, want) in rsi.iter().skip(14).zip(expected) {
            assert!((got - want).abs() < 0.01, "got {got}, want {want}");
        }
    }

    #[test]
    fn rsi_seed_counts_each_change_once() {
        // changes +1, -1, +2: avg gain 1, avg loss 1/3, so RS 3 and RSI 75
        let series: Vec<Series> = [10.0, 11.0, 10.0, 12.0, 11.0]
            .iter()
            .map(|&close| Series {
                datetime: Default::default(),
                open: close,
                close,
                high: close,
                low: close,
                volume: 0.0,
            })
            .collect();

        let rsi = calculate_rsi(&series, 3);
        assert!(rsi.iter().take(3).all(|v| v.is_nan()));
        assert!((rsi[3] - 75.0).abs() < 1e-4, "got {}", rsi[3]);
        assert!((rsi[4] - 54.545).abs() < 1e-2, "got {}", rsi[4]);
    }
}
