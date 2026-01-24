// Trading Signals
// Buy/Sell signal generation

#[derive(Debug, Clone, PartialEq)]
pub enum SignalType {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone)]
pub struct TradingSignal {
    pub signal_type: SignalType,
    pub symbol: String,
    pub strength: f64,
    pub reason: String,
}

impl TradingSignal {
    pub fn new(signal_type: SignalType, symbol: &str, strength: f64, reason: &str) -> Self {
        TradingSignal {
            signal_type,
            symbol: symbol.to_string(),
            strength,
            reason: reason.to_string(),
        }
    }

    pub fn display(&self) -> String {
        format!(
            "{:?} {} (strength: {:.2}) - {}",
            self.signal_type, self.symbol, self.strength, self.reason
        )
    }
}

pub fn generate_signals(rsi: f64, macd: f64) -> SignalType {
    if rsi < 30.0 && macd > 0.0 {
        SignalType::Buy
    } else if rsi > 70.0 && macd < 0.0 {
        SignalType::Sell
    } else {
        SignalType::Hold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trading_signal_new() {
        let signal = TradingSignal::new(SignalType::Buy, "TEST", 0.8, "Oversold");
        assert_eq!(signal.signal_type, SignalType::Buy);
        assert_eq!(signal.symbol, "TEST");
        assert_eq!(signal.strength, 0.8);
        assert_eq!(signal.reason, "Oversold");
    }

    #[test]
    fn test_trading_signal_display() {
        let signal = TradingSignal::new(SignalType::Sell, "TEST", 0.9, "Overbought");
        let display = signal.display();
        assert!(display.contains("Sell"));
        assert!(display.contains("TEST"));
        assert!(display.contains("0.90"));
        assert!(display.contains("Overbought"));
    }

    #[test]
    fn test_generate_signals_buy() {
        let signal = generate_signals(25.0, 1.0);
        assert_eq!(signal, SignalType::Buy);
    }

    #[test]
    fn test_generate_signals_sell() {
        let signal = generate_signals(75.0, -1.0);
        assert_eq!(signal, SignalType::Sell);
    }

    #[test]
    fn test_generate_signals_hold_neutral() {
        let signal = generate_signals(50.0, 0.5);
        assert_eq!(signal, SignalType::Hold);
    }

    #[test]
    fn test_generate_signals_hold_rsi_low_macd_negative() {
        let signal = generate_signals(25.0, -1.0);
        assert_eq!(signal, SignalType::Hold);
    }

    #[test]
    fn test_generate_signals_hold_rsi_high_macd_positive() {
        let signal = generate_signals(75.0, 1.0);
        assert_eq!(signal, SignalType::Hold);
    }
}
