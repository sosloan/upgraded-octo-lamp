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
