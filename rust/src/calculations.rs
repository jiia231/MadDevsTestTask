pub mod rsi {
    pub struct RsiCalculator {
        period: usize,
        close_prices: Vec<f64>,
    }

    impl RsiCalculator {
        pub fn new(period: usize) -> Self {
            RsiCalculator {
                period,
                close_prices: Vec::new(),
            }
        }

        pub fn update(&mut self, close_price: f64) -> Option<f64> {
            self.close_prices.push(close_price);

            if self.close_prices.len() >= self.period {
                let gain_loss_values = self.calculate_gain_loss_values();
                let average_gain = self.calculate_average(&gain_loss_values.gains);
                let average_loss = self.calculate_average(&gain_loss_values.losses);

                if average_loss != 0.0 {
                    let relative_strength = average_gain / average_loss;
                    let rsi = 100.0 - (100.0 / (1.0 + relative_strength));
                    Some(rsi)
                } else {
                    Some(100.0)
                }
            } else {
                None
            }
        }

        fn calculate_gain_loss_values(&self) -> GainLossValues {
            let mut gains = Vec::new();
            let mut losses = Vec::new();

            for i in 1..self.close_prices.len() {
                let price_diff = self.close_prices[i] - self.close_prices[i - 1];
                if price_diff >= 0.0 {
                    gains.push(price_diff);
                    losses.push(0.0);
                } else {
                    gains.push(0.0);
                    losses.push(price_diff.abs());
                }
            }

            GainLossValues { gains, losses }
        }

        fn calculate_average(&self, values: &[f64]) -> f64 {
            let sum: f64 = values.iter().sum();
            sum / values.len() as f64
        }
    }

    struct GainLossValues {
        gains: Vec<f64>,
        losses: Vec<f64>,
    }
}

pub mod vwap {
    pub struct VwapCalculator {
        total_volume: f64,
        cumulative_price_volume: f64,
    }

    impl VwapCalculator {
        pub fn new() -> Self {
            VwapCalculator {
                total_volume: 0.0,
                cumulative_price_volume: 0.0,
            }
        }

        pub fn update(&mut self, price: f64, volume: f64) -> f64 {
            self.cumulative_price_volume += price * volume;
            self.total_volume += volume;

            if self.total_volume > 0.0 {
                self.cumulative_price_volume / self.total_volume
            } else {
                0.0 // Return 0 if there's no volume
            }
        }
    }
}
