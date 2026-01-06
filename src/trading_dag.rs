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
