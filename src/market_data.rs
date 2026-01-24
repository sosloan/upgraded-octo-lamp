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

impl Default for MarketDataFeed {
    fn default() -> Self {
        Self::new()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_spread() {
        let quote = Quote {
            symbol: "TEST".to_string(),
            bid: 100.0,
            ask: 101.0,
            last: 100.5,
            volume: 1000,
            timestamp: 0,
        };
        assert_eq!(quote.spread(), 1.0);
    }

    #[test]
    fn test_quote_mid_price() {
        let quote = Quote {
            symbol: "TEST".to_string(),
            bid: 100.0,
            ask: 102.0,
            last: 101.0,
            volume: 1000,
            timestamp: 0,
        };
        assert_eq!(quote.mid_price(), 101.0);
    }

    #[test]
    fn test_market_data_feed_new() {
        let feed = MarketDataFeed::new();
        assert_eq!(feed.get_all_quotes().len(), 0);
    }

    #[test]
    fn test_market_data_feed_add_quote() {
        let mut feed = MarketDataFeed::new();
        let quote = Quote {
            symbol: "TEST".to_string(),
            bid: 100.0,
            ask: 101.0,
            last: 100.5,
            volume: 1000,
            timestamp: 1,
        };
        feed.add_quote(quote);
        assert_eq!(feed.get_all_quotes().len(), 1);
    }

    #[test]
    fn test_market_data_feed_latest_quote() {
        let mut feed = MarketDataFeed::new();
        let quote1 = Quote {
            symbol: "TEST".to_string(),
            bid: 100.0,
            ask: 101.0,
            last: 100.5,
            volume: 1000,
            timestamp: 1,
        };
        let quote2 = Quote {
            symbol: "TEST".to_string(),
            bid: 102.0,
            ask: 103.0,
            last: 102.5,
            volume: 2000,
            timestamp: 2,
        };
        feed.add_quote(quote1);
        feed.add_quote(quote2);
        
        let latest = feed.latest_quote("TEST");
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().bid, 102.0);
        assert_eq!(latest.unwrap().timestamp, 2);
    }

    #[test]
    fn test_market_data_feed_latest_quote_not_found() {
        let feed = MarketDataFeed::new();
        assert!(feed.latest_quote("NONEXISTENT").is_none());
    }
}
