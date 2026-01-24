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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_new() {
        let order = Order::new("TEST", OrderSide::Buy, OrderType::Market, 100.0);
        assert_eq!(order.symbol, "TEST");
        assert_eq!(order.quantity, 100.0);
        assert!(!order.filled);
    }

    #[test]
    fn test_order_execute_market_buy() {
        let mut order = Order::new("TEST", OrderSide::Buy, OrderType::Market, 100.0);
        let position = order.execute(50.0);
        assert!(position.is_some());
        assert!(order.filled);
        let pos = position.unwrap();
        assert_eq!(pos.quantity, 100.0);
        assert_eq!(pos.avg_price, 50.0);
    }

    #[test]
    fn test_order_execute_market_sell() {
        let mut order = Order::new("TEST", OrderSide::Sell, OrderType::Market, 100.0);
        let position = order.execute(50.0);
        assert!(position.is_some());
        let pos = position.unwrap();
        assert_eq!(pos.quantity, -100.0);
    }

    #[test]
    fn test_order_execute_limit_buy_fills() {
        let mut order = Order::new("TEST", OrderSide::Buy, OrderType::Limit(51.0), 100.0);
        let position = order.execute(50.0);
        assert!(position.is_some());
        assert!(order.filled);
    }

    #[test]
    fn test_order_execute_limit_buy_no_fill() {
        let mut order = Order::new("TEST", OrderSide::Buy, OrderType::Limit(49.0), 100.0);
        let position = order.execute(50.0);
        assert!(position.is_none());
        assert!(!order.filled);
    }

    #[test]
    fn test_order_execute_limit_sell_fills() {
        let mut order = Order::new("TEST", OrderSide::Sell, OrderType::Limit(49.0), 100.0);
        let position = order.execute(50.0);
        assert!(position.is_some());
        assert!(order.filled);
    }

    #[test]
    fn test_order_execute_limit_sell_no_fill() {
        let mut order = Order::new("TEST", OrderSide::Sell, OrderType::Limit(51.0), 100.0);
        let position = order.execute(50.0);
        assert!(position.is_none());
        assert!(!order.filled);
    }

    #[test]
    fn test_order_execute_stop_buy_triggers() {
        let mut order = Order::new("TEST", OrderSide::Buy, OrderType::Stop(49.0), 100.0);
        let position = order.execute(50.0);
        assert!(position.is_some());
        assert!(order.filled);
    }

    #[test]
    fn test_order_execute_stop_buy_no_trigger() {
        let mut order = Order::new("TEST", OrderSide::Buy, OrderType::Stop(51.0), 100.0);
        let position = order.execute(50.0);
        assert!(position.is_none());
        assert!(!order.filled);
    }

    #[test]
    fn test_order_execute_stop_sell_triggers() {
        let mut order = Order::new("TEST", OrderSide::Sell, OrderType::Stop(51.0), 100.0);
        let position = order.execute(50.0);
        assert!(position.is_some());
        assert!(order.filled);
    }

    #[test]
    fn test_order_execute_stop_sell_no_trigger() {
        let mut order = Order::new("TEST", OrderSide::Sell, OrderType::Stop(49.0), 100.0);
        let position = order.execute(50.0);
        assert!(position.is_none());
        assert!(!order.filled);
    }

    #[test]
    fn test_order_already_filled() {
        let mut order = Order::new("TEST", OrderSide::Buy, OrderType::Market, 100.0);
        let _ = order.execute(50.0);
        let position = order.execute(50.0);
        assert!(position.is_none());
    }
}
