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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pnl_report_display() {
        let report = PnLReport {
            realized_pnl: 1000.0,
            unrealized_pnl: 500.0,
            total_pnl: 1500.0,
            return_pct: 15.0,
        };
        let display = report.display();
        assert!(display.contains("1000.00"));
        assert!(display.contains("500.00"));
        assert!(display.contains("1500.00"));
        assert!(display.contains("15.00"));
    }

    #[test]
    fn test_pnl_calculator_new() {
        let calc = PnLCalculator::new(10000.0);
        let report = calc.calculate_report(&[]);
        assert_eq!(report.realized_pnl, 0.0);
        assert_eq!(report.unrealized_pnl, 0.0);
        assert_eq!(report.total_pnl, 0.0);
    }

    #[test]
    fn test_pnl_calculator_add_realized_pnl() {
        let mut calc = PnLCalculator::new(10000.0);
        calc.add_realized_pnl(500.0);
        let report = calc.calculate_report(&[]);
        assert_eq!(report.realized_pnl, 500.0);
    }

    #[test]
    fn test_pnl_calculator_calculate_report() {
        let mut calc = PnLCalculator::new(10000.0);
        calc.add_realized_pnl(500.0);
        
        let positions = vec![
            Position {
                symbol: "TEST1".to_string(),
                quantity: 100.0,
                avg_price: 50.0,
                current_price: 55.0,
            },
            Position {
                symbol: "TEST2".to_string(),
                quantity: 50.0,
                avg_price: 100.0,
                current_price: 90.0,
            },
        ];
        
        let report = calc.calculate_report(&positions);
        assert_eq!(report.realized_pnl, 500.0);
        assert_eq!(report.unrealized_pnl, 0.0); // (55-50)*100 + (90-100)*50 = 500 - 500 = 0
        assert_eq!(report.total_pnl, 500.0);
        assert_eq!(report.return_pct, 5.0);
    }

    #[test]
    fn test_pnl_calculator_negative_return() {
        let mut calc = PnLCalculator::new(10000.0);
        calc.add_realized_pnl(-500.0);
        let report = calc.calculate_report(&[]);
        assert_eq!(report.return_pct, -5.0);
    }
}
