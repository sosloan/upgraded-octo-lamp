// Momentum Indicators
// Technical analysis for trading signals

pub struct MomentumIndicator {
    pub name: String,
    pub value: f64,
}

impl MomentumIndicator {
    pub fn new(name: &str, value: f64) -> Self {
        MomentumIndicator {
            name: name.to_string(),
            value,
        }
    }
}

pub fn calculate_rsi(prices: &[f64], period: usize) -> f64 {
    if prices.len() < period + 1 {
        return 50.0;
    }

    let mut gains = 0.0;
    let mut losses = 0.0;

    for i in 1..=period {
        let change = prices[i] - prices[i - 1];
        if change > 0.0 {
            gains += change;
        } else {
            losses -= change;
        }
    }

    let avg_gain = gains / period as f64;
    let avg_loss = losses / period as f64;

    if avg_loss == 0.0 {
        return 100.0;
    }

    let rs = avg_gain / avg_loss;
    100.0 - (100.0 / (1.0 + rs))
}

pub fn calculate_macd(prices: &[f64]) -> (f64, f64, f64) {
    let ema12 = calculate_ema(prices, 12);
    let ema26 = calculate_ema(prices, 26);
    let macd_line = ema12 - ema26;
    let signal_line = macd_line * 0.9; // Simplified signal
    let histogram = macd_line - signal_line;
    (macd_line, signal_line, histogram)
}

fn calculate_ema(prices: &[f64], period: usize) -> f64 {
    if prices.is_empty() {
        return 0.0;
    }
    let alpha = 2.0 / (period as f64 + 1.0);
    let mut ema = prices[0];
    for &price in prices.iter().skip(1) {
        ema = alpha * price + (1.0 - alpha) * ema;
    }
    ema
}
