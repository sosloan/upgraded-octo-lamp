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
