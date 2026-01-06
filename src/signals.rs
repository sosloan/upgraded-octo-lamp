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
