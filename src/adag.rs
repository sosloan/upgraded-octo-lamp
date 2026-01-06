// A-DAG: Acyclic Directed Acyclic Graph
// OCTOTREÉ, Task DAG, Topological Sort, Critical Path

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub duration: u32,
    pub dependencies: Vec<String>,
}

#[derive(Debug)]
pub struct OctoTree {
    tasks: HashMap<String, Task>,
}

impl Default for OctoTree {
    fn default() -> Self {
        Self::new()
    }
}

impl OctoTree {
    pub fn new() -> Self {
        OctoTree {
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id.clone(), task);
    }

    pub fn topological_sort(&self) -> Result<Vec<String>, String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

        // Initialize
        for (id, task) in &self.tasks {
            in_degree.insert(id.clone(), task.dependencies.len());
            for dep in &task.dependencies {
                adj_list.entry(dep.clone()).or_default().push(id.clone());
            }
        }

        // Kahn's algorithm
        let mut queue: VecDeque<String> = VecDeque::new();
        for (id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(id.clone());
            }
        }

        let mut result = Vec::new();
        while let Some(task_id) = queue.pop_front() {
            result.push(task_id.clone());

            if let Some(neighbors) = adj_list.get(&task_id) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        if result.len() != self.tasks.len() {
            Err("Cycle detected in DAG".to_string())
        } else {
            Ok(result)
        }
    }

    pub fn critical_path(&self) -> Result<(Vec<String>, u32), String> {
        let topo_order = self.topological_sort()?;
        let mut earliest_start: HashMap<String, u32> = HashMap::new();

        // Calculate earliest start times
        for task_id in &topo_order {
            if let Some(task) = self.tasks.get(task_id) {
                let max_dep_finish = task.dependencies.iter()
                    .filter_map(|dep| earliest_start.get(dep))
                    .max()
                    .unwrap_or(&0);
                earliest_start.insert(task_id.clone(), *max_dep_finish);
            }
        }

        // Find critical path
        let max_time = *earliest_start.values().max().unwrap_or(&0);
        let critical_tasks: Vec<String> = topo_order.iter()
            .filter(|id| earliest_start.get(*id).unwrap_or(&0) == &max_time)
            .cloned()
            .collect();

        Ok((critical_tasks, max_time))
    }

    pub fn display(&self) -> String {
        format!("OCTOTREÉ: {} tasks", self.tasks.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() {
        let mut tree = OctoTree::new();
        tree.add_task(Task {
            id: "A".to_string(),
            name: "Task A".to_string(),
            duration: 5,
            dependencies: vec![],
        });
        tree.add_task(Task {
            id: "B".to_string(),
            name: "Task B".to_string(),
            duration: 3,
            dependencies: vec!["A".to_string()],
        });

        let result = tree.topological_sort().unwrap();
        assert_eq!(result, vec!["A", "B"]);
    }
}
