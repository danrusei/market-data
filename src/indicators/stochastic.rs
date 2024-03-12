use crate::Series;
use std::collections::VecDeque;

// calculation based on:
// https://www.investopedia.com/terms/s/stochasticoscillator.asp

pub(crate) fn calculate_stochastic(series: &[Series], period: usize) -> VecDeque<f32> {
    if series.len() <= period {
        return VecDeque::new();
    }

    let mut stochastic_values = VecDeque::new();

    // This loop calculates %K for each data point.
    for i in period..series.len() {
        let closing_price = series[i].close;
        let mut lowest_low = f32::INFINITY;
        let mut highest_high = f32::NEG_INFINITY;

        //Find Lowest Low and Highest High:
        for j in (i - period + 1)..=i {
            let current_low = series[j].low;
            let current_high = series[j].high;

            if current_low < lowest_low {
                lowest_low = current_low;
            }

            if current_high > highest_high {
                highest_high = current_high;
            }
        }

        // The %K value is calculated using the formula when the highest high is not equal to the lowest low.
        let percent_k = if highest_high != lowest_low {
            ((closing_price - lowest_low) / (highest_high - lowest_low)) * 100.0
        } else {
            0.0
        };

        stochastic_values.push_back(percent_k);
    }

    for _ in 1..period + 1 {
        stochastic_values.push_front(0.0);
    }

    assert!(stochastic_values.len() == series.len());

    stochastic_values
}
