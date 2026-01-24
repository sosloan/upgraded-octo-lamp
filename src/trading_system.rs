// Trading System
// Unified interface for all trading components

use crate::cure_foundation::CureFoundation;
use crate::market_data::MarketDataFeed;
use crate::pnl::PnLCalculator;
use crate::signals::TradingSignal;
use crate::trading_models::{BiotechSymbol, Position};

pub struct TradingSystem {
    pub biotech_symbols: Vec<BiotechSymbol>,
    pub market_feed: MarketDataFeed,
    pub pnl_calc: PnLCalculator,
    pub positions: Vec<Position>,
    pub signals: Vec<TradingSignal>,
    pub cure_foundation: CureFoundation,
}

impl TradingSystem {
    pub fn new(initial_capital: f64) -> Self {
        TradingSystem {
            biotech_symbols: crate::trading_models::get_biotech_universe(),
            market_feed: MarketDataFeed::new(),
            pnl_calc: PnLCalculator::new(initial_capital),
            positions: Vec::new(),
            signals: Vec::new(),
            cure_foundation: crate::cure_foundation::initialize_cure_foundation(),
        }
    }

    pub fn add_position(&mut self, position: Position) {
        self.positions.push(position);
    }

    pub fn add_signal(&mut self, signal: TradingSignal) {
        self.signals.push(signal);
    }

    pub fn get_portfolio_value(&self) -> f64 {
        self.positions.iter().map(|p| p.market_value()).sum()
    }

    pub fn display_summary(&self) -> String {
        format!(
            "Trading System:\n  Symbols: {}\n  Positions: {}\n  Signals: {}\n  Portfolio Value: ${:.2}\n  {}",
            self.biotech_symbols.len(),
            self.positions.len(),
            self.signals.len(),
            self.get_portfolio_value(),
            self.cure_foundation.display()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trading_system_new() {
        let system = TradingSystem::new(1_000_000.0);
        assert_eq!(system.biotech_symbols.len(), 5);
        assert_eq!(system.positions.len(), 0);
        assert_eq!(system.signals.len(), 0);
    }

    #[test]
    fn test_trading_system_add_position() {
        let mut system = TradingSystem::new(1_000_000.0);
        let position = Position {
            symbol: "TEST".to_string(),
            quantity: 100.0,
            avg_price: 50.0,
            current_price: 55.0,
        };
        system.add_position(position);
        assert_eq!(system.positions.len(), 1);
    }

    #[test]
    fn test_trading_system_add_signal() {
        let mut system = TradingSystem::new(1_000_000.0);
        let signal = TradingSignal::new(crate::signals::SignalType::Buy, "TEST", 0.8, "Test signal");
        system.add_signal(signal);
        assert_eq!(system.signals.len(), 1);
    }

    #[test]
    fn test_trading_system_get_portfolio_value() {
        let mut system = TradingSystem::new(1_000_000.0);
        let position1 = Position {
            symbol: "TEST1".to_string(),
            quantity: 100.0,
            avg_price: 50.0,
            current_price: 55.0,
        };
        let position2 = Position {
            symbol: "TEST2".to_string(),
            quantity: 50.0,
            avg_price: 100.0,
            current_price: 110.0,
        };
        system.add_position(position1);
        system.add_position(position2);
        
        assert_eq!(system.get_portfolio_value(), 11000.0); // 100*55 + 50*110
    }

    #[test]
    fn test_trading_system_display_summary() {
        let system = TradingSystem::new(1_000_000.0);
        let summary = system.display_summary();
        assert!(summary.contains("Trading System"));
        assert!(summary.contains("Symbols: 5"));
        assert!(summary.contains("CURE Foundation"));
    }
}
