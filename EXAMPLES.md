# Vision Detection Examples

This file provides practical examples of using the Erlang and Elixir vision detection modules.

## Erlang Examples

### Starting the Vision Detector

```erlang
%% Compile the module
c(vision_detector).

%% Start the gen_server
{ok, Pid} = vision_detector:start_link().
```

### Synchronous Detection

```erlang
%% Load image data (simulated)
ImageData = <<"binary image data here">>.

%% Perform detection
{ok, Result} = vision_detector:detect(ImageData).

%% Access results
#{status := Status, objects := Objects} = Result.
io:format("Detected ~p objects~n", [length(Objects)]).
```

### Asynchronous Detection

```erlang
%% Define callback to receive results
CallbackPid = self().

%% Start async detection
ok = vision_detector:detect_async(ImageData, CallbackPid).

%% Wait for results
receive
    {detection_result, Result} ->
        io:format("Async detection complete: ~p~n", [Result])
after 5000 ->
    io:format("Timeout waiting for detection~n")
end.
```

### Batch Processing with Erlang

```erlang
%% Process multiple images in parallel
Images = [Image1, Image2, Image3, Image4, Image5].

ProcessImage = fun(Image) ->
    spawn(fun() ->
        {ok, Result} = vision_detector:detect(Image),
        io:format("Processed image: ~p objects detected~n", 
                  [length(maps:get(objects, Result))])
    end)
end.

lists:foreach(ProcessImage, Images).
```

## Elixir Examples

### Starting the Vision Detector

```elixir
# Compile the module
c("vision_detector.ex")

# Start the GenServer
{:ok, pid} = VisionDetector.start_link()
```

### Synchronous Detection

```elixir
# Load image data
image_data = <<1, 2, 3, 4, 5, 6, 7, 8>>

# Perform detection
{:ok, result} = VisionDetector.detect(image_data)

# Access results using pattern matching
%{
  status: :detected,
  objects: objects,
  confidence: confidence,
  timestamp: timestamp
} = result

IO.puts("Detected #{length(objects)} objects with #{confidence * 100}% confidence")
```

### Asynchronous Detection with Task

```elixir
# Start async detection
task = VisionDetector.detect_async(image_data)

# Do other work...
IO.puts("Processing other tasks while detection runs...")

# Wait for result
result = Task.await(task)
IO.inspect(result, label: "Detection Result")
```

### Pipeline-Based Processing

```elixir
# Use the built-in pipeline
result = VisionDetector.pipeline(image_data)

# The pipeline handles:
# 1. Preprocessing
# 2. Feature extraction
# 3. Object classification
# 4. Post-processing
```

### Batch Processing with Stream

```elixir
# Load multiple images
images = [image1, image2, image3, image4, image5]

# Process batch with concurrent streams
results = VisionDetector.detect_batch(images, max_concurrency: 4)
|> Enum.map(fn {:ok, result} -> result end)

# Display summary
total_objects = results
|> Enum.map(fn r -> length(r.objects) end)
|> Enum.sum()

IO.puts("Total objects detected across all images: #{total_objects}")
```

### Statistics and Monitoring

```elixir
# Get detection statistics
{:ok, stats} = VisionDetector.stats()

IO.puts """
Detection Statistics:
  Total Detections: #{stats.detection_count}
  Total Objects: #{stats.total_objects}
  Avg Objects/Detection: #{stats.avg_objects_per_detection}
"""
```

### Visualization

```elixir
# Detect and visualize
{:ok, result} = VisionDetector.detect(image_data)
output = VisionDetector.visualize(result)
IO.puts(output)

# Output example:
# Vision Detection Results
# ========================
# Status: detected
# Objects Detected: 15
# Average Confidence: 0.9234
# Timestamp: 2026-01-06 05:19:00.000000Z
# Processor: elixir
#
# Objects:
#   1. polyglot_symbol (confidence: 0.9567)
#   2. luxury_emblem (confidence: 0.9234)
#   3. tech_icon (confidence: 0.9123)
#   ...
```

## Advanced Examples

### Distributed Vision Processing (Elixir)

```elixir
# Connect to multiple nodes
Node.connect(:"vision1@localhost")
Node.connect(:"vision2@localhost")

# Distribute work across nodes
images
|> Enum.chunk_every(10)
|> Enum.map(fn chunk ->
  node = Enum.random([node() | Node.list()])
  Task.Supervisor.async({VisionApp.TaskSupervisor, node}, fn ->
    VisionDetector.detect_batch(chunk)
  end)
end)
|> Enum.map(&Task.await(&1, 30_000))
|> List.flatten()
```

### Real-Time Video Stream Processing (Elixir)

```elixir
defmodule VideoProcessor do
  def process_stream(video_stream) do
    video_stream
    |> Stream.map(&decode_frame/1)
    |> Stream.chunk_every(30) # Process 30 frames at a time
    |> Task.async_stream(
      &VisionDetector.detect_batch/1,
      max_concurrency: 8
    )
    |> Stream.each(fn {:ok, results} ->
      handle_detection_results(results)
    end)
    |> Stream.run()
  end

  defp decode_frame(frame), do: # Decode video frame
  defp handle_detection_results(results), do: # Process results
end
```

### Caching Detection Results (Erlang)

```erlang
%% Setup cache table
setup_cache() ->
    ets:new(detection_cache, [named_table, public, set]).

%% Cached detection
cached_detect(ImageData) ->
    Hash = erlang:phash2(ImageData),
    case ets:lookup(detection_cache, Hash) of
        [{_, CachedResult}] ->
            io:format("Cache hit!~n"),
            {ok, CachedResult};
        [] ->
            io:format("Cache miss, detecting...~n"),
            {ok, Result} = vision_detector:detect(ImageData),
            ets:insert(detection_cache, {Hash, Result}),
            {ok, Result}
    end.
```

### Supervised Application (Elixir)

```elixir
defmodule VisionApp.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # Start the vision detector
      {VisionDetector, []},
      
      # Start a task supervisor for async work
      {Task.Supervisor, name: VisionApp.TaskSupervisor},
      
      # Start your other workers
      VisionApp.LoadBalancer,
      VisionApp.ResultStore
    ]

    opts = [strategy: :one_for_one, name: VisionApp.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
```

## Testing Examples

### Erlang Unit Tests (EUnit)

```erlang
-module(vision_detector_tests).
-include_lib("eunit/include/eunit.hrl").

detection_test() ->
    {ok, _Pid} = vision_detector:start_link(),
    ImageData = <<"test data">>,
    {ok, Result} = vision_detector:detect(ImageData),
    ?assertMatch(#{status := detected}, Result).
```

### Elixir Tests (ExUnit)

```elixir
defmodule VisionDetectorTest do
  use ExUnit.Case
  doctest VisionDetector

  setup do
    {:ok, pid} = VisionDetector.start_link()
    %{pid: pid}
  end

  test "detects objects in image data" do
    image_data = <<1, 2, 3, 4, 5>>
    {:ok, result} = VisionDetector.detect(image_data)

    assert result.status == :detected
    assert is_list(result.objects)
    assert result.confidence > 0.0
  end

  test "batch processing returns multiple results" do
    images = [<<1>>, <<2>>, <<3>>]
    results = VisionDetector.detect_batch(images)
    |> Enum.to_list()

    assert length(results) == 3
  end
end
```

## Performance Benchmarking

### Erlang Benchmarking

```erlang
benchmark() ->
    {ok, _} = vision_detector:start_link(),
    ImageData = crypto:strong_rand_bytes(1024),
    
    {Time, _} = timer:tc(fun() ->
        [vision_detector:detect(ImageData) || _ <- lists:seq(1, 1000)]
    end),
    
    io:format("Average detection time: ~p microseconds~n", [Time / 1000]).
```

### Elixir Benchmarking with Benchee

```elixir
Benchee.run(%{
  "sync detection" => fn ->
    VisionDetector.detect(image_data)
  end,
  "async detection" => fn ->
    VisionDetector.detect_async(image_data) |> Task.await()
  end,
  "batch detection (10)" => fn ->
    VisionDetector.detect_batch(images_10) |> Enum.to_list()
  end
})
```

---

These examples demonstrate the power and flexibility of Erlang/OTP and Elixir for building concurrent, fault-tolerant vision detection systems.
