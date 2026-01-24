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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_momentum_indicator_new() {
        let indicator = MomentumIndicator::new("RSI", 65.5);
        assert_eq!(indicator.name, "RSI");
        assert_eq!(indicator.value, 65.5);
    }

    #[test]
    fn test_calculate_rsi_insufficient_data() {
        let prices = vec![100.0, 101.0];
        let rsi = calculate_rsi(&prices, 14);
        assert_eq!(rsi, 50.0);
    }

    #[test]
    fn test_calculate_rsi_all_gains() {
        let prices = vec![100.0, 101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0, 109.0, 110.0, 111.0, 112.0, 113.0, 114.0];
        let rsi = calculate_rsi(&prices, 14);
        assert_eq!(rsi, 100.0);
    }

    #[test]
    fn test_calculate_rsi_mixed() {
        let prices = vec![100.0, 101.0, 100.5, 101.5, 100.8, 102.0, 101.0, 102.5, 101.5, 103.0, 102.0, 103.5, 102.5, 104.0, 103.0];
        let rsi = calculate_rsi(&prices, 14);
        assert!(rsi > 0.0 && rsi < 100.0);
    }

    #[test]
    fn test_calculate_macd() {
        let prices = vec![100.0, 101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0, 109.0, 110.0, 111.0, 112.0];
        let (macd_line, signal_line, histogram) = calculate_macd(&prices);
        assert!(macd_line > 0.0);
        assert_eq!(signal_line, macd_line * 0.9);
        assert_eq!(histogram, macd_line - signal_line);
    }

    #[test]
    fn test_calculate_ema_empty() {
        let prices: Vec<f64> = vec![];
        let ema = calculate_ema(&prices, 12);
        assert_eq!(ema, 0.0);
    }

    #[test]
    fn test_calculate_ema_single_value() {
        let prices = vec![100.0];
        let ema = calculate_ema(&prices, 12);
        assert_eq!(ema, 100.0);
    }

    #[test]
    fn test_calculate_ema_multiple_values() {
        let prices = vec![100.0, 102.0, 104.0, 106.0];
        let ema = calculate_ema(&prices, 3);
        assert!(ema > 100.0 && ema <= 106.0);
    }
}
