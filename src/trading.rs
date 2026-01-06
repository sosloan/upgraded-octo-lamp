// Trading Execution
// Order management and execution

use crate::trading_models::Position;

#[derive(Debug, Clone)]
pub enum OrderType {
    Market,
    Limit(f64),
    Stop(f64),
}

#[derive(Debug, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub filled: bool,
}

impl Order {
    pub fn new(symbol: &str, side: OrderSide, order_type: OrderType, quantity: f64) -> Self {
        Order {
            symbol: symbol.to_string(),
            side,
            order_type,
            quantity,
            filled: false,
        }
    }

    pub fn execute(&mut self, price: f64) -> Option<Position> {
        if self.filled {
            return None;
        }

        match &self.order_type {
            OrderType::Market => {
                self.filled = true;
                Some(Position {
                    symbol: self.symbol.clone(),
                    quantity: match self.side {
                        OrderSide::Buy => self.quantity,
                        OrderSide::Sell => -self.quantity,
                    },
                    avg_price: price,
                    current_price: price,
                })
            }
            OrderType::Limit(limit_price) => {
                let can_execute = match self.side {
                    OrderSide::Buy => price <= *limit_price,
                    OrderSide::Sell => price >= *limit_price,
                };
                if can_execute {
                    self.filled = true;
                    Some(Position {
                        symbol: self.symbol.clone(),
                        quantity: match self.side {
                            OrderSide::Buy => self.quantity,
                            OrderSide::Sell => -self.quantity,
                        },
                        avg_price: price,
                        current_price: price,
                    })
                } else {
                    None
                }
            }
            OrderType::Stop(stop_price) => {
                let triggered = match self.side {
                    OrderSide::Buy => price >= *stop_price,
                    OrderSide::Sell => price <= *stop_price,
                };
                if triggered {
                    self.filled = true;
                    Some(Position {
                        symbol: self.symbol.clone(),
                        quantity: match self.side {
                            OrderSide::Buy => self.quantity,
                            OrderSide::Sell => -self.quantity,
                        },
                        avg_price: price,
                        current_price: price,
                    })
                } else {
                    None
                }
            }
        }
    }
}
