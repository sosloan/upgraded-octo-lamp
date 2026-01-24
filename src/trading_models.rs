// Trading Models
// Core data structures for biotech trading

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiotechSymbol {
    pub ticker: String,
    pub company_name: String,
    pub sector: String,
    pub market_cap: f64,
}

impl BiotechSymbol {
    pub fn new(ticker: &str, company_name: &str, sector: &str, market_cap: f64) -> Self {
        BiotechSymbol {
            ticker: ticker.to_string(),
            company_name: company_name.to_string(),
            sector: sector.to_string(),
            market_cap,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub avg_price: f64,
    pub current_price: f64,
}

impl Position {
    pub fn unrealized_pnl(&self) -> f64 {
        (self.current_price - self.avg_price) * self.quantity
    }

    pub fn market_value(&self) -> f64 {
        self.current_price * self.quantity
    }
}

pub fn get_biotech_universe() -> Vec<BiotechSymbol> {
    vec![
        BiotechSymbol::new("BIIB", "Biogen Inc", "Biotechnology", 38_000_000_000.0),
        BiotechSymbol::new("GILD", "Gilead Sciences", "Biotechnology", 95_000_000_000.0),
        BiotechSymbol::new("VRTX", "Vertex Pharmaceuticals", "Biotechnology", 110_000_000_000.0),
        BiotechSymbol::new("REGN", "Regeneron Pharmaceuticals", "Biotechnology", 85_000_000_000.0),
        BiotechSymbol::new("AMGN", "Amgen Inc", "Biotechnology", 138_000_000_000.0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biotech_symbol_new() {
        let symbol = BiotechSymbol::new("TEST", "Test Company", "Biotech", 1_000_000.0);
        assert_eq!(symbol.ticker, "TEST");
        assert_eq!(symbol.company_name, "Test Company");
        assert_eq!(symbol.sector, "Biotech");
        assert_eq!(symbol.market_cap, 1_000_000.0);
    }

    #[test]
    fn test_position_unrealized_pnl() {
        let position = Position {
            symbol: "TEST".to_string(),
            quantity: 100.0,
            avg_price: 50.0,
            current_price: 60.0,
        };
        assert_eq!(position.unrealized_pnl(), 1000.0); // (60 - 50) * 100
    }

    #[test]
    fn test_position_unrealized_pnl_negative() {
        let position = Position {
            symbol: "TEST".to_string(),
            quantity: 100.0,
            avg_price: 60.0,
            current_price: 50.0,
        };
        assert_eq!(position.unrealized_pnl(), -1000.0); // (50 - 60) * 100
    }

    #[test]
    fn test_position_market_value() {
        let position = Position {
            symbol: "TEST".to_string(),
            quantity: 100.0,
            avg_price: 50.0,
            current_price: 60.0,
        };
        assert_eq!(position.market_value(), 6000.0); // 60 * 100
    }

    #[test]
    fn test_get_biotech_universe() {
        let universe = get_biotech_universe();
        assert_eq!(universe.len(), 5);
        assert_eq!(universe[0].ticker, "BIIB");
        assert_eq!(universe[4].ticker, "AMGN");
    }
}
