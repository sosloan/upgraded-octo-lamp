// P&L (Profit and Loss) Calculation
// Portfolio performance tracking

use crate::trading_models::Position;

#[derive(Debug, Clone)]
pub struct PnLReport {
    pub realized_pnl: f64,
    pub unrealized_pnl: f64,
    pub total_pnl: f64,
    pub return_pct: f64,
}

impl PnLReport {
    pub fn display(&self) -> String {
        format!(
            "P&L Report:\n  Realized: ${:.2}\n  Unrealized: ${:.2}\n  Total: ${:.2}\n  Return: {:.2}%",
            self.realized_pnl, self.unrealized_pnl, self.total_pnl, self.return_pct
        )
    }
}

pub struct PnLCalculator {
    initial_capital: f64,
    realized_pnl: f64,
}

impl PnLCalculator {
    pub fn new(initial_capital: f64) -> Self {
        PnLCalculator {
            initial_capital,
            realized_pnl: 0.0,
        }
    }

    pub fn add_realized_pnl(&mut self, pnl: f64) {
        self.realized_pnl += pnl;
    }

    pub fn calculate_report(&self, positions: &[Position]) -> PnLReport {
        let unrealized_pnl: f64 = positions.iter().map(|p| p.unrealized_pnl()).sum();
        let total_pnl = self.realized_pnl + unrealized_pnl;
        let return_pct = (total_pnl / self.initial_capital) * 100.0;

        PnLReport {
            realized_pnl: self.realized_pnl,
            unrealized_pnl,
            total_pnl,
            return_pct,
        }
    }
}
