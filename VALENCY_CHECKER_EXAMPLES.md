# Valency Checker Examples

This file provides practical examples of using the Valency Checker for edge semantics and ambiguity elimination.

## Getting Started

```elixir
# Compile the module
c("valency_checker.ex")

# Check a single word
{:ok, result} = ValencyChecker.check("eating")
IO.inspect(result)
```

## Basic Examples

### Example 1: Simple Word Analysis

```elixir
# Analyze the word "running"
{:ok, analysis} = ValencyChecker.check("running")

IO.puts """
Word: #{analysis.word}
Root: #{analysis.stem}
Valency: #{analysis.valency}
Requires: #{Enum.join(Enum.map(analysis.required_roles, &to_string/1), ", ")}
"""

# Output:
# Word: running
# Root: run
# Valency: 1
# Requires: agent
```

### Example 2: Comparing Verb Valencies

```elixir
# Compare different verb types
verbs = ["sleep", "eat", "give"]

Enum.each(verbs, fn verb ->
  {:ok, analysis} = ValencyChecker.check(verb)
  IO.puts "#{verb}: valency #{analysis.valency}"
end)

# Output:
# sleep: valency 1 (monovalent)
# eat: valency 2 (divalent)
# give: valency 3 (trivalent)
```

### Example 3: Visualizing Analysis

```elixir
# Get detailed visualization
{:ok, analysis} = ValencyChecker.check("processing")
visualization = ValencyChecker.visualize(analysis)
IO.puts(visualization)

# Output:
# Valency Analysis
# ================
# Word: processing
# Stem: process
# Valency: 2
# Required Roles: agent, patient
# Optional Roles: manner, time
# Ambiguity Score: 0.3000 (lower is better)
# 
# Interpretation:
# The verb "process" has valency 2.
# Required semantic roles: agent, patient
# Optional semantic roles: manner, time
```

## Advanced Examples

### Example 4: Batch Processing

```elixir
# Process multiple words concurrently
words = ["detecting", "analyzing", "processing", "classifying", "extracting"]

results = ValencyChecker.check_batch(words)

# Display summary
Enum.each(results, fn 
  {:ok, analysis} ->
    IO.puts "#{analysis.word} -> #{analysis.stem} (valency: #{analysis.valency})"
  {:error, reason} ->
    IO.puts "Error: #{reason}"
end)

# Output:
# detecting -> detect (valency: 2)
# analyzing -> analyze (valency: 2)
# processing -> process (valency: 2)
# classifying -> classify (valency: 2)
# extracting -> extract (valency: 2)
```

### Example 5: Ambiguity Elimination

```elixir
# Analyze sentences to eliminate ambiguity
sentences = [
  "The system processes the data",
  "The detector finds the pattern",
  "The algorithm analyzes the results"
]

Enum.each(sentences, fn sentence ->
  IO.puts "\nSentence: #{sentence}"
  
  case ValencyChecker.eliminate_ambiguity(sentence) do
    {:ok, interpretations} ->
      Enum.each(interpretations, fn interp ->
        IO.puts "  Verb: #{interp.verb} (valency: #{interp.valency}, score: #{interp.score})"
      end)
    
    {:error, reason} ->
      IO.puts "  Error: #{reason}"
  end
end)

# Output:
# Sentence: The system processes the data
#   Verb: process (valency: 2, score: 0.3)
#
# Sentence: The detector finds the pattern
#   Verb: find (valency: 2, score: 0.25)
#
# Sentence: The algorithm analyzes the results
#   Verb: analyze (valency: 2, score: 0.3)
```

### Example 6: Semantic Role Extraction

```elixir
# Extract semantic roles from sentences
test_cases = [
  "The robot moves the box",
  "The system detects anomalies",
  "The processor analyzes data"
]

Enum.each(test_cases, fn sentence ->
  IO.puts "\nAnalyzing: #{sentence}"
  
  case ValencyChecker.analyze_roles(sentence) do
    {:ok, roles} ->
      IO.puts "  Verb: #{roles.verb}"
      if roles[:agent], do: IO.puts "  Agent: #{roles.agent}"
      if roles[:patient], do: IO.puts "  Patient: #{roles.patient}"
    
    {:error, reason} ->
      IO.puts "  Error: #{reason}"
  end
end)
```

### Example 7: Memory Footprint Analysis

```elixir
# Demonstrate lightweight nature
footprint = ValencyChecker.memory_footprint()

IO.puts """
Edge Semantics Memory Footprint
================================
Lexicon: #{footprint.lexicon_kb} KB
Stemma:  #{footprint.stemma_kb} KB
Total:   #{footprint.total_kb} KB

This is #{Float.round(1000 / footprint.total_kb, 1)}x more efficient than a 5MB model!
Perfect for edge devices with limited memory.
"""

# Output:
# Edge Semantics Memory Footprint
# ================================
# Lexicon: 3.38 KB
# Stemma:  1.85 KB
# Total:   5.22 KB
#
# This is 191.6x more efficient than a 5MB model!
# Perfect for edge devices with limited memory.
```

## Integration Examples

### Example 8: Integration with Vision Detection

```elixir
# Combine semantic and visual analysis
defmodule EdgeProcessor do
  def analyze_scene(image_data, description) do
    # Process in parallel
    vision_task = Task.async(fn ->
      # Assuming VisionDetector is available
      # VisionDetector.detect(image_data)
      {:ok, %{objects: [], confidence: 0.9}}
    end)
    
    semantic_task = Task.async(fn ->
      ValencyChecker.eliminate_ambiguity(description)
    end)
    
    # Wait for both
    vision_result = Task.await(vision_task)
    semantic_result = Task.await(semantic_task)
    
    %{
      visual: vision_result,
      semantic: semantic_result,
      processed_at: DateTime.utc_now(),
      edge_processing: true
    }
  end
end

# Use it
result = EdgeProcessor.analyze_scene(
  <<1, 2, 3>>,
  "The camera detects the object"
)

IO.inspect(result)
```

### Example 9: Real-Time Stream Processing

```elixir
# Process a stream of commands
defmodule CommandProcessor do
  def process_stream(commands) do
    commands
    |> Stream.map(&String.trim/1)
    |> Stream.filter(&(&1 != ""))
    |> Stream.map(fn command ->
      case ValencyChecker.eliminate_ambiguity(command) do
        {:ok, interpretations} ->
          {command, List.first(interpretations)}
        {:error, _} ->
          {command, nil}
      end
    end)
    |> Enum.to_list()
  end
end

# Example usage
commands = [
  "The robot moves forward",
  "The system processes input",
  "The detector finds targets"
]

results = CommandProcessor.process_stream(commands)
IO.inspect(results, label: "Processed Commands")
```

### Example 10: Caching for Performance

```elixir
# Cache frequently analyzed words
defmodule CachedValencyChecker do
  use Agent
  
  def start_link(_opts) do
    Agent.start_link(fn -> %{} end, name: __MODULE__)
  end
  
  def check_cached(word) do
    cache = Agent.get(__MODULE__, & &1)
    
    case Map.get(cache, word) do
      nil ->
        # Cache miss - perform analysis
        result = ValencyChecker.check(word)
        Agent.update(__MODULE__, &Map.put(&1, word, result))
        result
      
      cached_result ->
        # Cache hit
        cached_result
    end
  end
  
  def cache_stats do
    cache = Agent.get(__MODULE__, & &1)
    %{
      entries: map_size(cache),
      words: Map.keys(cache)
    }
  end
end

# Usage
{:ok, _} = CachedValencyChecker.start_link([])

# First call - cache miss
CachedValencyChecker.check_cached("running")

# Second call - cache hit (faster)
CachedValencyChecker.check_cached("running")

stats = CachedValencyChecker.cache_stats()
IO.inspect(stats)
```

## Domain-Specific Examples

### Example 11: IoT Command Processing

```elixir
# Process IoT device commands
defmodule IoTCommandProcessor do
  def parse_command(command) do
    case ValencyChecker.analyze_roles(command) do
      {:ok, %{verb: verb, agent: agent, patient: patient}} ->
        %{
          action: verb,
          device: agent,
          target: patient,
          executable: true
        }
      
      {:error, _} ->
        %{executable: false, error: "Could not parse command"}
    end
  end
end

# Test IoT commands
iot_commands = [
  "The thermostat adjusts temperature",
  "The sensor detects motion",
  "The camera captures images"
]

Enum.each(iot_commands, fn cmd ->
  result = IoTCommandProcessor.parse_command(cmd)
  IO.inspect(result, label: cmd)
end)
```

### Example 12: Natural Language Query Processing

```elixir
# Process natural language queries
defmodule QueryProcessor do
  def process_query(query) do
    {:ok, stem} = query
    |> String.downcase()
    |> String.split()
    |> Enum.find_value(fn word ->
      case ValencyChecker.check(word) do
        {:ok, analysis} -> {:ok, analysis}
        _ -> nil
      end
    end)
    |> case do
      {:ok, analysis} -> {:ok, analysis.stem}
      _ -> {:error, :no_verb}
    end
    
    case stem do
      {:ok, "find"} -> {:search_query, query}
      {:ok, "show"} -> {:display_query, query}
      {:ok, "analyze"} -> {:analysis_query, query}
      _ -> {:unknown_query, query}
    end
  end
end

# Test queries
queries = [
  "Find all documents containing the keyword",
  "Show me the latest results",
  "Analyze the performance data"
]

Enum.each(queries, fn query ->
  {type, _} = QueryProcessor.process_query(query)
  IO.puts "#{query} -> #{type}"
end)
```

## Performance Benchmarking

### Example 13: Speed Comparison

```elixir
# Benchmark different operations
defmodule ValencyBenchmark do
  def run do
    words = ["running", "eating", "processing", "detecting"]
    iterations = 10_000
    
    # Single word check
    {time_single, _} = :timer.tc(fn ->
      Enum.each(1..iterations, fn _ ->
        ValencyChecker.check("running")
      end)
    end)
    
    # Batch processing
    {time_batch, _} = :timer.tc(fn ->
      Enum.each(1..div(iterations, 4), fn _ ->
        ValencyChecker.check_batch(words)
      end)
    end)
    
    IO.puts """
    Performance Benchmark (#{iterations} iterations)
    ================================================
    Single word check: #{Float.round(time_single / iterations, 2)} μs/check
    Batch processing:  #{Float.round(time_batch / iterations, 2)} μs/check
    
    Batch is #{Float.round(time_single / time_batch, 2)}x faster due to parallelism!
    """
  end
end

ValencyBenchmark.run()
```

## Error Handling Examples

### Example 14: Robust Error Handling

```elixir
# Handle various error cases
defmodule RobustProcessor do
  def safe_check(word) do
    case ValencyChecker.check(word) do
      {:ok, analysis} ->
        IO.puts "✓ #{word}: valency #{analysis.valency}"
      
      {:error, :not_in_lexicon} ->
        IO.puts "✗ #{word}: not in lexicon"
        # Could fall back to heuristic analysis
        {:ok, stem} = ValencyChecker.get_stem(word)
        IO.puts "  (stem: #{stem})"
      
      {:error, reason} ->
        IO.puts "✗ #{word}: error - #{reason}"
    end
  end
end

# Test with mixed valid/invalid words
words = ["running", "eating", "unknown_verb", "processing"]
Enum.each(words, &RobustProcessor.safe_check/1)
```

## Testing Examples

### Example 15: Property-Based Testing Concept

```elixir
# Demonstrate properties that should always hold
defmodule ValencyProperties do
  def verify_properties do
    # Property 1: Valency should always be positive
    test_words = ["run", "eat", "give", "sleep"]
    
    Enum.each(test_words, fn word ->
      {:ok, analysis} = ValencyChecker.check(word)
      assert analysis.valency > 0, "Valency must be positive"
      IO.puts "✓ #{word}: valency #{analysis.valency} > 0"
    end)
    
    # Property 2: Stemmed words should have same valency as root
    {:ok, stem1} = ValencyChecker.check("run")
    {:ok, stem2} = ValencyChecker.check("running")
    assert stem1.valency == stem2.valency
    IO.puts "✓ run and running have same valency"
    
    # Property 3: Memory footprint should be small
    footprint = ValencyChecker.memory_footprint()
    assert footprint.total_kb < 10, "Total size should be under 10KB"
    IO.puts "✓ Memory footprint: #{footprint.total_kb} KB < 10 KB"
    
    IO.puts "\n✓ All properties verified!"
  end
end

ValencyProperties.verify_properties()
```

---

These examples demonstrate the power and flexibility of the Valency Checker for edge semantics. The system provides sophisticated natural language processing capabilities while maintaining an extremely small memory footprint and zero cloud dependency.
