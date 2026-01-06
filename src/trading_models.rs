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
