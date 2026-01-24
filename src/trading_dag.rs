// Trading DAG
// DAG-based trading workflow orchestration

use crate::adag::{OctoTree, Task};

pub struct TradingWorkflow {
    dag: OctoTree,
}

impl Default for TradingWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

impl TradingWorkflow {
    pub fn new() -> Self {
        let mut dag = OctoTree::new();

        // Build trading workflow DAG
        dag.add_task(Task {
            id: "fetch_data".to_string(),
            name: "Fetch Market Data".to_string(),
            duration: 2,
            dependencies: vec![],
        });

        dag.add_task(Task {
            id: "calculate_indicators".to_string(),
            name: "Calculate Technical Indicators".to_string(),
            duration: 3,
            dependencies: vec!["fetch_data".to_string()],
        });

        dag.add_task(Task {
            id: "generate_signals".to_string(),
            name: "Generate Trading Signals".to_string(),
            duration: 2,
            dependencies: vec!["calculate_indicators".to_string()],
        });

        dag.add_task(Task {
            id: "risk_check".to_string(),
            name: "Risk Management Check".to_string(),
            duration: 1,
            dependencies: vec!["generate_signals".to_string()],
        });

        dag.add_task(Task {
            id: "execute_trades".to_string(),
            name: "Execute Trades".to_string(),
            duration: 2,
            dependencies: vec!["risk_check".to_string()],
        });

        TradingWorkflow { dag }
    }

    pub fn get_execution_order(&self) -> Result<Vec<String>, String> {
        self.dag.topological_sort()
    }

    pub fn display(&self) -> String {
        self.dag.display()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trading_workflow_new() {
        let workflow = TradingWorkflow::new();
        assert!(workflow.get_execution_order().is_ok());
    }

    #[test]
    fn test_trading_workflow_execution_order() {
        let workflow = TradingWorkflow::new();
        let order = workflow.get_execution_order().unwrap();
        
        // Verify we have the expected tasks
        assert!(order.len() > 0);
        assert!(order.contains(&"fetch_data".to_string()));
        assert!(order.contains(&"execute_trades".to_string()));
    }

    #[test]
    fn test_trading_workflow_correct_sequence() {
        let workflow = TradingWorkflow::new();
        let order = workflow.get_execution_order().unwrap();
        
        // Verify the execution order maintains dependency relationships
        let fetch_idx = order.iter().position(|x| x == "fetch_data");
        let calc_idx = order.iter().position(|x| x == "calculate_indicators");
        let signal_idx = order.iter().position(|x| x == "generate_signals");
        let risk_idx = order.iter().position(|x| x == "risk_check");
        let exec_idx = order.iter().position(|x| x == "execute_trades");
        
        // All tasks should exist
        assert!(fetch_idx.is_some());
        assert!(calc_idx.is_some());
        assert!(signal_idx.is_some());
        assert!(risk_idx.is_some());
        assert!(exec_idx.is_some());
        
        // Verify dependency order
        let fetch_idx = fetch_idx.unwrap();
        let calc_idx = calc_idx.unwrap();
        let signal_idx = signal_idx.unwrap();
        let risk_idx = risk_idx.unwrap();
        let exec_idx = exec_idx.unwrap();
        
        assert!(fetch_idx < calc_idx, "fetch_data must come before calculate_indicators");
        assert!(calc_idx < signal_idx, "calculate_indicators must come before generate_signals");
        assert!(signal_idx < risk_idx, "generate_signals must come before risk_check");
        assert!(risk_idx < exec_idx, "risk_check must come before execute_trades");
    }

    #[test]
    fn test_trading_workflow_display() {
        let workflow = TradingWorkflow::new();
        let display = workflow.display();
        assert!(display.contains("5 tasks"));
    }
}
