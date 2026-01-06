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
                .or_insert_with(Vec::new)
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
        format!(
            "Storm Topologies:\n  • Word Count\n  • Sum\n  • Edison ⚡\n  • Polymath 🌐\n  • Key Bounce\n  • Randomize Keys 🎹"
        )
    }
}
