// Capital Flow Analysis
// Money flow and volume analysis

#[derive(Debug, Clone)]
pub struct CapitalFlow {
    pub symbol: String,
    pub inflow: f64,
    pub outflow: f64,
    pub net_flow: f64,
}

impl CapitalFlow {
    pub fn new(symbol: &str, inflow: f64, outflow: f64) -> Self {
        CapitalFlow {
            symbol: symbol.to_string(),
            inflow,
            outflow,
            net_flow: inflow - outflow,
        }
    }

    pub fn flow_ratio(&self) -> f64 {
        if self.outflow == 0.0 {
            return f64::INFINITY;
        }
        self.inflow / self.outflow
    }

    pub fn is_bullish(&self) -> bool {
        self.net_flow > 0.0
    }
}

pub fn calculate_money_flow(prices: &[f64], volumes: &[u64]) -> f64 {
    if prices.len() < 2 || volumes.is_empty() {
        return 0.0;
    }

    let mut positive_flow = 0.0;
    let mut negative_flow = 0.0;

    for i in 1..prices.len().min(volumes.len()) {
        let typical_price = prices[i];
        let volume = volumes[i] as f64;
        let money_flow = typical_price * volume;

        if prices[i] > prices[i - 1] {
            positive_flow += money_flow;
        } else {
            negative_flow += money_flow;
        }
    }

    if negative_flow == 0.0 {
        return 100.0;
    }

    let money_ratio = positive_flow / negative_flow;
    100.0 - (100.0 / (1.0 + money_ratio))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capital_flow_new() {
        let flow = CapitalFlow::new("TEST", 1000.0, 500.0);
        assert_eq!(flow.symbol, "TEST");
        assert_eq!(flow.inflow, 1000.0);
        assert_eq!(flow.outflow, 500.0);
        assert_eq!(flow.net_flow, 500.0);
    }

    #[test]
    fn test_capital_flow_flow_ratio() {
        let flow = CapitalFlow::new("TEST", 1000.0, 500.0);
        assert_eq!(flow.flow_ratio(), 2.0);
    }

    #[test]
    fn test_capital_flow_flow_ratio_zero_outflow() {
        let flow = CapitalFlow::new("TEST", 1000.0, 0.0);
        assert_eq!(flow.flow_ratio(), f64::INFINITY);
    }

    #[test]
    fn test_capital_flow_is_bullish_true() {
        let flow = CapitalFlow::new("TEST", 1000.0, 500.0);
        assert!(flow.is_bullish());
    }

    #[test]
    fn test_capital_flow_is_bullish_false() {
        let flow = CapitalFlow::new("TEST", 500.0, 1000.0);
        assert!(!flow.is_bullish());
    }

    #[test]
    fn test_calculate_money_flow_insufficient_data() {
        let prices = vec![100.0];
        let volumes = vec![];
        let flow = calculate_money_flow(&prices, &volumes);
        assert_eq!(flow, 0.0);
    }

    #[test]
    fn test_calculate_money_flow_positive() {
        let prices = vec![100.0, 101.0, 102.0, 103.0];
        let volumes = vec![1000, 1000, 1000, 1000];
        let flow = calculate_money_flow(&prices, &volumes);
        assert_eq!(flow, 100.0);
    }

    #[test]
    fn test_calculate_money_flow_negative() {
        let prices = vec![103.0, 102.0, 101.0, 100.0];
        let volumes = vec![1000, 1000, 1000, 1000];
        let flow = calculate_money_flow(&prices, &volumes);
        assert!(flow < 50.0);
    }

    #[test]
    fn test_calculate_money_flow_mixed() {
        let prices = vec![100.0, 101.0, 100.5, 101.5, 100.8];
        let volumes = vec![1000, 1000, 1000, 1000, 1000];
        let flow = calculate_money_flow(&prices, &volumes);
        assert!(flow > 0.0 && flow < 100.0);
    }
}
