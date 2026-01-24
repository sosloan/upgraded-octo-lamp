// Storm Topologies
// Distributed stream processing topologies

use std::collections::HashMap;

// Bolt: Processing unit in Storm topology
pub trait Bolt {
    fn execute(&mut self, input: &str) -> Vec<String>;
}

// Word Count Topology
pub struct WordCountBolt {
    counts: HashMap<String, usize>,
}

impl Default for WordCountBolt {
    fn default() -> Self {
        Self::new()
    }
}

impl WordCountBolt {
    pub fn new() -> Self {
        WordCountBolt {
            counts: HashMap::new(),
        }
    }

    pub fn get_counts(&self) -> &HashMap<String, usize> {
        &self.counts
    }
}

impl Bolt for WordCountBolt {
    fn execute(&mut self, input: &str) -> Vec<String> {
        for word in input.split_whitespace() {
            *self.counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }
        vec![format!("Processed: {}", input)]
    }
}

// Sum Topology
pub struct SumBolt {
    total: f64,
}

impl Default for SumBolt {
    fn default() -> Self {
        Self::new()
    }
}

impl SumBolt {
    pub fn new() -> Self {
        SumBolt { total: 0.0 }
    }

    pub fn get_total(&self) -> f64 {
        self.total
    }
}

impl Bolt for SumBolt {
    fn execute(&mut self, input: &str) -> Vec<String> {
        if let Ok(num) = input.trim().parse::<f64>() {
            self.total += num;
            vec![format!("Sum: {}", self.total)]
        } else {
            vec!["Invalid number".to_string()]
        }
    }
}

// Edison ⚡ Topology: Electric/Energy processing
pub struct EdisonBolt {
    voltage: f64,
    current: f64,
}

impl Default for EdisonBolt {
    fn default() -> Self {
        Self::new()
    }
}

impl EdisonBolt {
    pub fn new() -> Self {
        EdisonBolt {
            voltage: 0.0,
            current: 0.0,
        }
    }

    pub fn power(&self) -> f64 {
        self.voltage * self.current
    }
}

impl Bolt for EdisonBolt {
    fn execute(&mut self, input: &str) -> Vec<String> {
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(v), Ok(i)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                self.voltage = v;
                self.current = i;
                return vec![format!("⚡ Power: {:.2}W", self.power())];
            }
        }
        vec!["Invalid input".to_string()]
    }
}

// Polymath 🌐 Topology: Multi-domain processing
pub struct PolymathBolt {
    domains: HashMap<String, Vec<String>>,
}

impl Default for PolymathBolt {
    fn default() -> Self {
        Self::new()
    }
}

impl PolymathBolt {
    pub fn new() -> Self {
        PolymathBolt {
            domains: HashMap::new(),
        }
    }

    pub fn add_domain(&mut self, domain: &str, items: Vec<String>) {
        self.domains.insert(domain.to_string(), items);
    }
}

impl Bolt for PolymathBolt {
    fn execute(&mut self, input: &str) -> Vec<String> {
        let parts: Vec<&str> = input.split(':').collect();
        if parts.len() == 2 {
            let domain = parts[0].trim();
            let item = parts[1].trim();
            self.domains
                .entry(domain.to_string())
                .or_default()
                .push(item.to_string());
            vec![format!("🌐 Added {} to {}", item, domain)]
        } else {
            vec!["Invalid format".to_string()]
        }
    }
}

// Key Bounce Topology: Debounce key events
pub struct KeyBounceBolt {
    last_key: Option<String>,
    bounce_count: usize,
}

impl Default for KeyBounceBolt {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyBounceBolt {
    pub fn new() -> Self {
        KeyBounceBolt {
            last_key: None,
            bounce_count: 0,
        }
    }
}

impl Bolt for KeyBounceBolt {
    fn execute(&mut self, input: &str) -> Vec<String> {
        if let Some(ref last) = self.last_key {
            if last == input {
                self.bounce_count += 1;
                return vec![format!("Bounce filtered: {}", input)];
            }
        }
        self.last_key = Some(input.to_string());
        self.bounce_count = 0;
        vec![format!("Key accepted: {}", input)]
    }
}

// Randomize Keys 🎹 Topology: Random key generation/processing
pub struct RandomizeKeysBolt {
    seed: u64,
}

impl RandomizeKeysBolt {
    pub fn new(seed: u64) -> Self {
        RandomizeKeysBolt { seed }
    }

    fn simple_random(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed
    }
}

impl Bolt for RandomizeKeysBolt {
    fn execute(&mut self, input: &str) -> Vec<String> {
        let rand = self.simple_random();
        vec![format!("🎹 {} -> Random: {}", input, rand % 88)] // 88 keys on piano
    }
}

// Storm Topology Manager
pub struct StormTopology {
    pub word_count: WordCountBolt,
    pub sum: SumBolt,
    pub edison: EdisonBolt,
    pub polymath: PolymathBolt,
    pub key_bounce: KeyBounceBolt,
    pub randomize_keys: RandomizeKeysBolt,
}

impl Default for StormTopology {
    fn default() -> Self {
        Self::new()
    }
}

impl StormTopology {
    pub fn new() -> Self {
        StormTopology {
            word_count: WordCountBolt::new(),
            sum: SumBolt::new(),
            edison: EdisonBolt::new(),
            polymath: PolymathBolt::new(),
            key_bounce: KeyBounceBolt::new(),
            randomize_keys: RandomizeKeysBolt::new(42),
        }
    }

    pub fn display(&self) -> String {
        "Storm Topologies:\n  • Word Count\n  • Sum\n  • Edison ⚡\n  • Polymath 🌐\n  • Key Bounce\n  • Randomize Keys 🎹".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count_bolt_new() {
        let bolt = WordCountBolt::new();
        assert_eq!(bolt.get_counts().len(), 0);
    }

    #[test]
    fn test_word_count_bolt_execute() {
        let mut bolt = WordCountBolt::new();
        bolt.execute("hello world");
        assert_eq!(*bolt.get_counts().get("hello").unwrap(), 1);
        assert_eq!(*bolt.get_counts().get("world").unwrap(), 1);
    }

    #[test]
    fn test_word_count_bolt_multiple_executes() {
        let mut bolt = WordCountBolt::new();
        bolt.execute("hello world");
        bolt.execute("hello again");
        assert_eq!(*bolt.get_counts().get("hello").unwrap(), 2);
        assert_eq!(*bolt.get_counts().get("world").unwrap(), 1);
        assert_eq!(*bolt.get_counts().get("again").unwrap(), 1);
    }

    #[test]
    fn test_sum_bolt_new() {
        let bolt = SumBolt::new();
        assert_eq!(bolt.get_total(), 0.0);
    }

    #[test]
    fn test_sum_bolt_execute() {
        let mut bolt = SumBolt::new();
        bolt.execute("10.5");
        assert_eq!(bolt.get_total(), 10.5);
    }

    #[test]
    fn test_sum_bolt_multiple_executes() {
        let mut bolt = SumBolt::new();
        bolt.execute("10.5");
        bolt.execute("20.3");
        bolt.execute("5.2");
        assert!((bolt.get_total() - 36.0).abs() < 0.01);
    }

    #[test]
    fn test_sum_bolt_invalid_input() {
        let mut bolt = SumBolt::new();
        let result = bolt.execute("not a number");
        assert_eq!(result[0], "Invalid number");
    }

    #[test]
    fn test_edison_bolt_new() {
        let bolt = EdisonBolt::new();
        assert_eq!(bolt.power(), 0.0);
    }

    #[test]
    fn test_edison_bolt_execute() {
        let mut bolt = EdisonBolt::new();
        bolt.execute("120,10");
        assert_eq!(bolt.power(), 1200.0);
    }

    #[test]
    fn test_edison_bolt_invalid_input() {
        let mut bolt = EdisonBolt::new();
        let result = bolt.execute("invalid");
        assert_eq!(result[0], "Invalid input");
    }

    #[test]
    fn test_polymath_bolt_new() {
        let bolt = PolymathBolt::new();
        assert_eq!(bolt.domains.len(), 0);
    }

    #[test]
    fn test_polymath_bolt_execute() {
        let mut bolt = PolymathBolt::new();
        bolt.execute("science: physics");
        assert!(bolt.domains.contains_key("science"));
    }

    #[test]
    fn test_polymath_bolt_multiple_domains() {
        let mut bolt = PolymathBolt::new();
        bolt.execute("science: physics");
        bolt.execute("art: painting");
        bolt.execute("science: chemistry");
        assert_eq!(bolt.domains.len(), 2);
        assert_eq!(bolt.domains.get("science").unwrap().len(), 2);
    }

    #[test]
    fn test_key_bounce_bolt_new() {
        let bolt = KeyBounceBolt::new();
        assert_eq!(bolt.bounce_count, 0);
        assert!(bolt.last_key.is_none());
    }

    #[test]
    fn test_key_bounce_bolt_first_key() {
        let mut bolt = KeyBounceBolt::new();
        let result = bolt.execute("a");
        assert!(result[0].contains("accepted"));
    }

    #[test]
    fn test_key_bounce_bolt_bounce_filter() {
        let mut bolt = KeyBounceBolt::new();
        bolt.execute("a");
        let result = bolt.execute("a");
        assert!(result[0].contains("Bounce filtered"));
    }

    #[test]
    fn test_key_bounce_bolt_different_key() {
        let mut bolt = KeyBounceBolt::new();
        bolt.execute("a");
        let result = bolt.execute("b");
        assert!(result[0].contains("accepted"));
    }

    #[test]
    fn test_randomize_keys_bolt_execute() {
        let mut bolt = RandomizeKeysBolt::new(42);
        let result = bolt.execute("test");
        assert!(result[0].contains("Random"));
    }

    #[test]
    fn test_randomize_keys_bolt_deterministic() {
        let mut bolt1 = RandomizeKeysBolt::new(42);
        let mut bolt2 = RandomizeKeysBolt::new(42);
        let result1 = bolt1.execute("test");
        let result2 = bolt2.execute("test");
        assert_eq!(result1[0], result2[0]);
    }

    #[test]
    fn test_storm_topology_new() {
        let topology = StormTopology::new();
        assert_eq!(topology.word_count.get_counts().len(), 0);
        assert_eq!(topology.sum.get_total(), 0.0);
    }

    #[test]
    fn test_storm_topology_display() {
        let topology = StormTopology::new();
        let display = topology.display();
        assert!(display.contains("Word Count"));
        assert!(display.contains("Edison"));
        assert!(display.contains("Polymath"));
    }
}
