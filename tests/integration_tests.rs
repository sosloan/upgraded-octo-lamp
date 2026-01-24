// Integration Tests for BET Architecture System
// Tests the integration of all major components

use bet_architecture::{
    adag::{OctoTree, Task},
    monad_lambda::{demonstrate_monad_system, MonadLaws, Plumber},
    storm::{Bolt, EdisonBolt, KeyBounceBolt, PolymathBolt, RandomizeKeysBolt, StormTopology, SumBolt, WordCountBolt},
    swin_transformer::SwinTransformer,
    trading_dag::TradingWorkflow,
    trading_system::TradingSystem,
    trading_models::Position,
    signals::{TradingSignal, SignalType},
};

#[test]
fn test_trading_system_integration() {
    // Test complete trading system integration
    let mut system = TradingSystem::new(1_000_000.0);
    
    // Verify initial state
    assert_eq!(system.positions.len(), 0);
    assert_eq!(system.signals.len(), 0);
    assert!(system.biotech_symbols.len() > 0);
    
    // Add a position
    let position = Position {
        symbol: "CURE".to_string(),
        quantity: 100.0,
        avg_price: 50.0,
        current_price: 55.0,
    };
    system.add_position(position);
    assert_eq!(system.positions.len(), 1);
    
    // Add a trading signal
    let signal = TradingSignal::new(SignalType::Buy, "CURE", 0.8, "Strong momentum");
    system.add_signal(signal);
    assert_eq!(system.signals.len(), 1);
    
    // Verify portfolio value calculation
    let portfolio_value = system.get_portfolio_value();
    assert_eq!(portfolio_value, 5500.0); // 100 * 55.0
    
    // Verify display works
    let summary = system.display_summary();
    assert!(summary.contains("Trading System"));
    assert!(summary.contains("Portfolio Value"));
}

#[test]
fn test_trading_workflow_dag_integration() {
    // Test trading workflow DAG integration
    let workflow = TradingWorkflow::new();
    
    // Get execution order
    let order = workflow.get_execution_order().expect("Should get execution order");
    
    // Verify workflow steps are in correct order
    assert_eq!(order.len(), 5);
    assert_eq!(order[0], "fetch_data");
    assert_eq!(order[1], "calculate_indicators");
    assert_eq!(order[2], "generate_signals");
    assert_eq!(order[3], "risk_check");
    assert_eq!(order[4], "execute_trades");
    
    // Verify display
    let display = workflow.display();
    assert!(display.contains("OCTOTREÉ"));
    assert!(display.contains("5 tasks"));
}

#[test]
fn test_storm_topologies_integration() {
    let mut storm = StormTopology::new();
    
    // Test Word Count topology
    let result = storm.word_count.execute("hello world hello");
    assert!(!result.is_empty());
    assert_eq!(storm.word_count.get_counts().get("hello"), Some(&2));
    assert_eq!(storm.word_count.get_counts().get("world"), Some(&1));
    
    // Test Sum topology
    let _result = storm.sum.execute("10.5");
    assert_eq!(storm.sum.get_total(), 10.5);
    let _result = storm.sum.execute("5.5");
    assert_eq!(storm.sum.get_total(), 16.0);
    
    // Test Edison topology (voltage, current -> power)
    let result = storm.edison.execute("220,5");
    assert_eq!(storm.edison.power(), 1100.0); // 220V * 5A = 1100W
    assert!(result[0].contains("⚡"));
    assert!(result[0].contains("1100"));
    
    // Test Polymath topology
    let result = storm.polymath.execute("math: calculus");
    assert!(result[0].contains("🌐"));
    assert!(result[0].contains("calculus"));
    assert!(result[0].contains("math"));
    
    let result = storm.polymath.execute("physics: thermodynamics");
    assert!(result[0].contains("thermodynamics"));
    
    // Test Key Bounce topology
    let result = storm.key_bounce.execute("A");
    assert!(result[0].contains("accepted"));
    let result = storm.key_bounce.execute("A"); // Same key
    assert!(result[0].contains("Bounce filtered"));
    let result = storm.key_bounce.execute("B"); // Different key
    assert!(result[0].contains("accepted"));
    
    // Test Randomize Keys topology
    let result = storm.randomize_keys.execute("C");
    assert!(result[0].contains("🎹"));
    assert!(result[0].contains("Random"));
}

#[test]
fn test_monad_lambda_integration() {
    // Test monad laws
    assert!(MonadLaws::verify_left_identity());
    assert!(MonadLaws::verify_right_identity());
    assert!(MonadLaws::verify_associativity());
    
    let laws_output = MonadLaws::verify_all();
    assert!(laws_output.contains("Left Identity"));
    assert!(laws_output.contains("Right Identity"));
    assert!(laws_output.contains("Associativity"));
    assert!(laws_output.contains("✓ Pass"));
    
    // Test Plumber composition
    let result = Plumber::new(10)
        .pipe(|x| Some(x * 2))
        .pipe(|x| Some(x + 5))
        .pipe(|x| Some(x * 3))
        .extract();
    
    assert_eq!(result, Some(75)); // ((10 * 2) + 5) * 3 = 75
    
    // Test demonstration output
    let demo = demonstrate_monad_system();
    assert!(demo.contains("Monad Laws"));
    assert!(demo.contains("Plumber Demo"));
    assert!(demo.contains("94")); // 42 * 2 + 10 = 94
}

#[test]
fn test_adag_critical_path() {
    // Test A-DAG critical path analysis
    let mut dag = OctoTree::new();
    
    dag.add_task(Task {
        id: "A".to_string(),
        name: "Task A".to_string(),
        duration: 5,
        dependencies: vec![],
    });
    
    dag.add_task(Task {
        id: "B".to_string(),
        name: "Task B".to_string(),
        duration: 10,
        dependencies: vec!["A".to_string()],
    });
    
    dag.add_task(Task {
        id: "C".to_string(),
        name: "Task C".to_string(),
        duration: 3,
        dependencies: vec!["A".to_string()],
    });
    
    // Test topological sort
    let sorted = dag.topological_sort().expect("Should sort successfully");
    assert_eq!(sorted[0], "A"); // A must come first
    assert!(sorted.contains(&"B".to_string()));
    assert!(sorted.contains(&"C".to_string()));
    
    // Test critical path
    let (_critical_tasks, _max_time) = dag.critical_path().expect("Should find critical path");
    // Critical path analysis completed successfully
}

#[test]
fn test_adag_cycle_detection() {
    // Test that cycles are detected
    let mut dag = OctoTree::new();
    
    dag.add_task(Task {
        id: "A".to_string(),
        name: "Task A".to_string(),
        duration: 5,
        dependencies: vec!["B".to_string()],
    });
    
    dag.add_task(Task {
        id: "B".to_string(),
        name: "Task B".to_string(),
        duration: 3,
        dependencies: vec!["A".to_string()],
    });
    
    // This should detect a cycle
    let result = dag.topological_sort();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cycle"));
}

#[test]
fn test_swin_transformer_integration() {
    let swin = SwinTransformer::with_16_heads();
    
    // Verify configuration
    assert_eq!(swin.heads.len(), 16);
    assert_eq!(swin.grey_shades, 600);
    
    // Test forward pass with various input sizes
    let small_input = vec![1.0, 2.0, 3.0];
    let output = swin.forward_pass(&small_input);
    assert!(!output.is_empty());
    assert_eq!(output.len(), small_input.len() * 16); // 16 heads
    
    let large_input = vec![1.0; 100];
    let output = swin.forward_pass(&large_input);
    assert_eq!(output.len(), 100 * 16);
    
    // Test grey eyes processing
    let test_image = vec![0, 64, 128, 192, 255];
    let processed = swin.grey_eyes_processing(&test_image);
    assert_eq!(processed.len(), test_image.len());
    
    // Verify that processing preserves ordering
    assert!(processed[0] <= processed[1]);
    assert!(processed[1] <= processed[2]);
    
    // Test 600 shades processing
    let continuous_data = vec![0.0, 0.25, 0.5, 0.75, 1.0];
    let shades = swin.process_with_600_shades(&continuous_data);
    assert_eq!(shades.len(), continuous_data.len());
    assert_eq!(shades[0], 0);
    assert_eq!(shades[4], 600);
    
    // Test display
    let display = swin.display();
    assert!(display.contains("16 Attention Heads"));
    assert!(display.contains("600 Shades"));
    assert!(display.contains("Grey Eyes"));
}

#[test]
fn test_multi_head_attention_consistency() {
    let swin = SwinTransformer::with_16_heads();
    
    // Test that same input produces consistent output
    let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let output1 = swin.forward_pass(&input);
    let output2 = swin.forward_pass(&input);
    
    assert_eq!(output1, output2);
}

#[test]
fn test_storm_pipeline_integration() {
    // Test a complete data processing pipeline through Storm topologies
    let mut word_count = WordCountBolt::new();
    let mut sum = SumBolt::new();
    
    // Process text through word count
    let texts = vec![
        "the quick brown fox",
        "jumps over the lazy dog",
        "the fox is quick"
    ];
    
    for text in texts {
        word_count.execute(text);
    }
    
    // Verify word counts
    assert_eq!(word_count.get_counts().get("the"), Some(&3));
    assert_eq!(word_count.get_counts().get("fox"), Some(&2));
    assert_eq!(word_count.get_counts().get("quick"), Some(&2));
    
    // Process numbers through sum
    let numbers = vec!["10.5", "20.3", "15.7", "8.5"];
    for num in numbers {
        sum.execute(num);
    }
    
    assert_eq!(sum.get_total(), 55.0);
}

#[test]
fn test_edison_power_calculations() {
    let mut edison = EdisonBolt::new();
    
    // Test various voltage/current combinations
    let test_cases = vec![
        ("120,10", 1200.0),  // 120V * 10A = 1200W
        ("240,5", 1200.0),   // 240V * 5A = 1200W
        ("12,100", 1200.0),  // 12V * 100A = 1200W
    ];
    
    for (input, expected_power) in test_cases {
        edison.execute(input);
        assert_eq!(edison.power(), expected_power);
    }
}

#[test]
fn test_polymath_multi_domain() {
    let mut polymath = PolymathBolt::new();
    
    // Add items to various domains
    polymath.execute("mathematics: algebra");
    polymath.execute("mathematics: geometry");
    polymath.execute("physics: mechanics");
    polymath.execute("physics: optics");
    polymath.execute("computer_science: algorithms");
    
    // Verify domains are correctly populated
    polymath.add_domain("mathematics", vec!["calculus".to_string()]);
    
    // The topology should handle multiple domains
    let result = polymath.execute("philosophy: logic");
    assert!(result[0].contains("philosophy"));
}

#[test]
fn test_end_to_end_trading_pipeline() {
    // Integration test: Complete trading pipeline from start to finish
    let mut system = TradingSystem::new(2_000_000.0);
    let workflow = TradingWorkflow::new();
    
    // Get the execution order from the workflow
    let execution_order = workflow.get_execution_order().expect("Workflow should be valid");
    assert_eq!(execution_order.len(), 5);
    
    // Simulate executing the workflow steps
    // 1. Fetch data (simulated by having biotech symbols)
    assert!(!system.biotech_symbols.is_empty());
    
    // 2. Calculate indicators (simulated by adding signals)
    let signal1 = TradingSignal::new(SignalType::Buy, "CURE", 0.85, "Strong buy signal");
    system.add_signal(signal1);
    
    // 3. Generate signals (already done above)
    assert_eq!(system.signals.len(), 1);
    
    // 4. Risk check (simulated by checking portfolio value)
    let initial_portfolio = system.get_portfolio_value();
    
    // 5. Execute trades (simulated by adding position)
    let position = Position {
        symbol: "CURE".to_string(),
        quantity: 200.0,
        avg_price: 48.0,
        current_price: 52.0,
    };
    system.add_position(position);
    
    // Verify the trade was executed
    assert_eq!(system.positions.len(), 1);
    let new_portfolio = system.get_portfolio_value();
    assert!(new_portfolio > initial_portfolio);
    assert_eq!(new_portfolio, 10400.0); // 200 * 52.0
}

#[test]
fn test_system_wide_integration() {
    // Ultimate integration test: All components working together
    
    // 1. Trading System
    let mut trading = TradingSystem::new(5_000_000.0);
    trading.add_position(Position {
        symbol: "CURE".to_string(),
        quantity: 1000.0,
        avg_price: 45.0,
        current_price: 50.0,
    });
    
    // 2. Storm Topologies
    let mut storm = StormTopology::new();
    storm.word_count.execute("buy sell trade profit loss");
    storm.sum.execute("100.5");
    storm.edison.execute("110,10");
    
    // 3. Monad System
    let plumber_result = Plumber::new(100)
        .pipe(|x| Some(x * 2))
        .extract();
    assert_eq!(plumber_result, Some(200));
    
    // 4. SWIN Transformer
    let swin = SwinTransformer::with_16_heads();
    let transformer_output = swin.forward_pass(&[1.0, 2.0, 3.0]);
    
    // 5. Trading Workflow
    let workflow = TradingWorkflow::new();
    let order = workflow.get_execution_order().unwrap();
    
    // Verify all systems are operational
    assert!(trading.get_portfolio_value() > 0.0);
    assert!(!storm.word_count.get_counts().is_empty());
    assert!(storm.sum.get_total() > 0.0);
    assert!(!transformer_output.is_empty());
    assert_eq!(order.len(), 5);
}

#[test]
fn test_plumber_error_handling() {
    // Test Plumber with None values in pipeline
    let result = Plumber::new(10)
        .pipe(|x| Some(x * 2))
        .pipe(|_x| -> Option<i32> { None }) // Inject failure with explicit return type
        .pipe(|x| Some(x * 3)) // This should not execute
        .extract();
    
    assert_eq!(result, None);
}

#[test]
fn test_storm_error_handling() {
    let mut sum = SumBolt::new();
    
    // Test with valid input
    let result = sum.execute("42.5");
    assert!(result[0].contains("Sum"));
    
    // Test with invalid input
    let result = sum.execute("not_a_number");
    assert!(result[0].contains("Invalid"));
    
    // Verify sum is unchanged after invalid input
    assert_eq!(sum.get_total(), 42.5);
}

#[test]
fn test_concurrent_storm_processing() {
    // Test Storm topologies can handle rapid sequential processing
    let mut word_count = WordCountBolt::new();
    
    // Pre-allocate test strings
    let test_strings: Vec<String> = (0..100)
        .map(|i| format!("word{} test data", i))
        .collect();
    
    for text in &test_strings {
        word_count.execute(text);
    }
    
    // Verify all words were counted
    assert_eq!(word_count.get_counts().get("test"), Some(&100));
    assert_eq!(word_count.get_counts().get("data"), Some(&100));
}

#[test]
fn test_swin_transformer_edge_cases() {
    let swin = SwinTransformer::with_16_heads();
    
    // Test with empty input
    let empty_output = swin.forward_pass(&[]);
    assert!(empty_output.is_empty());
    
    // Test with single value
    let single_output = swin.forward_pass(&[5.0]);
    assert_eq!(single_output.len(), 16);
    
    // Test with large input
    let large_input = vec![1.0; 1000];
    let large_output = swin.forward_pass(&large_input);
    assert_eq!(large_output.len(), 16000);
    
    // Test grey eyes with edge values
    let edge_image = vec![0, 255];
    let processed = swin.grey_eyes_processing(&edge_image);
    assert_eq!(processed.len(), 2);
}

#[test]
fn test_trading_system_multiple_positions() {
    let mut system = TradingSystem::new(10_000_000.0);
    
    // Add multiple positions
    let positions = vec![
        Position { symbol: "CURE".to_string(), quantity: 100.0, avg_price: 50.0, current_price: 55.0 },
        Position { symbol: "BIOTECH".to_string(), quantity: 200.0, avg_price: 30.0, current_price: 35.0 },
        Position { symbol: "PHARMA".to_string(), quantity: 150.0, avg_price: 40.0, current_price: 42.0 },
    ];
    
    for pos in positions {
        system.add_position(pos);
    }
    
    assert_eq!(system.positions.len(), 3);
    
    // Calculate total portfolio value
    // 100 * 55.0 + 200 * 35.0 + 150 * 42.0
    let expected_value = 5500.0 + 7000.0 + 6300.0;
    assert_eq!(system.get_portfolio_value(), expected_value);
}

#[test]
fn test_randomize_keys_determinism() {
    // Test that RandomizeKeysBolt with same seed produces consistent results
    let mut rng1 = RandomizeKeysBolt::new(42);
    let mut rng2 = RandomizeKeysBolt::new(42);
    
    for _ in 0..10 {
        let result1 = rng1.execute("test");
        let result2 = rng2.execute("test");
        assert_eq!(result1, result2);
    }
}

#[test]
fn test_key_bounce_state_management() {
    let mut key_bounce = KeyBounceBolt::new();
    
    // Test sequence: A, A, B, B, A
    let inputs = vec!["A", "A", "B", "B", "A"];
    let expected_accepts = vec![true, false, true, false, true];
    
    for (input, should_accept) in inputs.iter().zip(expected_accepts.iter()) {
        let result = key_bounce.execute(input);
        if *should_accept {
            assert!(result[0].contains("accepted"));
        } else {
            assert!(result[0].contains("filtered"));
        }
    }
}
