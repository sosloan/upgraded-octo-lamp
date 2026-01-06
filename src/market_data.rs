// Market Data
// Real-time and historical market data structures

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub volume: u64,
    pub timestamp: u64,
}

impl Quote {
    pub fn spread(&self) -> f64 {
        self.ask - self.bid
    }

    pub fn mid_price(&self) -> f64 {
        (self.bid + self.ask) / 2.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLCV {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub timestamp: u64,
}

pub struct MarketDataFeed {
    quotes: Vec<Quote>,
}

impl MarketDataFeed {
    pub fn new() -> Self {
        MarketDataFeed { quotes: Vec::new() }
    }

    pub fn add_quote(&mut self, quote: Quote) {
        self.quotes.push(quote);
    }

    pub fn latest_quote(&self, symbol: &str) -> Option<&Quote> {
        self.quotes.iter().rev().find(|q| q.symbol == symbol)
    }

    pub fn get_all_quotes(&self) -> &[Quote] {
        &self.quotes
    }
}
