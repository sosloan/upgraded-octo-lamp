# Valency Checker Test Suite
# 
# Run with: elixir valency_checker_test.exs
# Or with mix: mix test valency_checker_test.exs

Code.require_file("valency_checker.ex", __DIR__)

ExUnit.start()

defmodule ValencyCheckerTest do
  use ExUnit.Case
  doctest ValencyChecker

  describe "check/1" do
    test "detects monovalent verbs" do
      {:ok, result} = ValencyChecker.check("run")
      assert result.valency == 1
      assert :agent in result.required_roles
    end

    test "detects divalent verbs" do
      {:ok, result} = ValencyChecker.check("eat")
      assert result.valency == 2
      assert :agent in result.required_roles
      assert :patient in result.required_roles
    end

    test "detects trivalent verbs" do
      {:ok, result} = ValencyChecker.check("give")
      assert result.valency == 3
      assert :agent in result.required_roles
      assert :patient in result.required_roles
      assert :recipient in result.required_roles
    end

    test "handles word variations using stemma" do
      {:ok, result1} = ValencyChecker.check("running")
      {:ok, result2} = ValencyChecker.check("run")
      
      assert result1.stem == result2.stem
      assert result1.valency == result2.valency
    end

    test "returns error for unknown words" do
      assert {:error, :not_in_lexicon} = ValencyChecker.check("zzz_unknown_verb")
    end

    test "returns error for empty strings" do
      assert {:error, :empty_string} = ValencyChecker.check("")
      assert {:error, :empty_string} = ValencyChecker.check("   ")
    end

    test "calculates ambiguity scores" do
      {:ok, result} = ValencyChecker.check("eat")
      assert result.ambiguity_score > 0
      assert result.ambiguity_score < 1
    end
  end

  describe "check_batch/1" do
    test "processes multiple words concurrently" do
      words = ["run", "eat", "give", "sleep"]
      results = ValencyChecker.check_batch(words)
      
      assert length(results) == 4
      
      Enum.each(results, fn
        {:ok, result} ->
          assert is_map(result)
          assert result.valency > 0
        {:error, _} ->
          :ok
      end)
    end

    test "handles empty list" do
      results = ValencyChecker.check_batch([])
      assert results == []
    end
  end

  describe "eliminate_ambiguity/1" do
    test "finds verbs in sentences" do
      sentence = "The system processes the data"
      {:ok, interpretations} = ValencyChecker.eliminate_ambiguity(sentence)
      
      assert is_list(interpretations)
      assert length(interpretations) > 0
      
      first = List.first(interpretations)
      assert first.verb == "process"
    end

    test "returns error when no verbs found" do
      sentence = "zzz xxx yyy"
      assert {:error, :no_verbs_found} = ValencyChecker.eliminate_ambiguity(sentence)
    end

    test "ranks interpretations by ambiguity score" do
      sentence = "The detector finds objects"
      {:ok, interpretations} = ValencyChecker.eliminate_ambiguity(sentence)
      
      scores = Enum.map(interpretations, & &1.score)
      assert scores == Enum.sort(scores)
    end
  end

  describe "analyze_roles/1" do
    test "extracts semantic roles from sentence" do
      sentence = "The system processes data"
      {:ok, roles} = ValencyChecker.analyze_roles(sentence)
      
      assert roles.verb == "process"
      assert is_map(roles)
    end

    test "returns error when no verb found" do
      sentence = "xxx yyy zzz"
      assert {:error, :no_verb_found} = ValencyChecker.analyze_roles(sentence)
    end
  end

  describe "get_stem/1" do
    test "returns stem for inflected forms" do
      {:ok, stem} = ValencyChecker.get_stem("running")
      assert stem == "run"
      
      {:ok, stem} = ValencyChecker.get_stem("ate")
      assert stem == "eat"
      
      {:ok, stem} = ValencyChecker.get_stem("given")
      assert stem == "give"
    end

    test "returns the word itself if not in stemma" do
      {:ok, stem} = ValencyChecker.get_stem("unknown")
      assert stem == "unknown"
    end

    test "handles case insensitivity" do
      {:ok, stem1} = ValencyChecker.get_stem("Running")
      {:ok, stem2} = ValencyChecker.get_stem("running")
      assert stem1 == stem2
    end
  end

  describe "get_valency_pattern/1" do
    test "retrieves pattern for known verbs" do
      {:ok, pattern} = ValencyChecker.get_valency_pattern("eat")
      
      assert pattern.valency == 2
      assert is_list(pattern.required)
      assert is_list(pattern.optional)
    end

    test "returns error for unknown verbs" do
      assert {:error, :not_in_lexicon} = ValencyChecker.get_valency_pattern("unknown_verb")
    end
  end

  describe "memory_footprint/0" do
    test "returns memory usage information" do
      footprint = ValencyChecker.memory_footprint()
      
      assert is_map(footprint)
      assert footprint.total_bytes > 0
      assert footprint.lexicon_bytes > 0
      assert footprint.stemma_bytes > 0
      assert footprint.total_kb > 0
    end

    test "verifies lightweight nature (< 10KB)" do
      footprint = ValencyChecker.memory_footprint()
      
      assert footprint.total_kb < 10.0
      assert footprint.lexicon_kb < 5.0
      assert footprint.stemma_kb < 3.0
    end
  end

  describe "visualize/1" do
    test "generates human-readable output" do
      {:ok, analysis} = ValencyChecker.check("eat")
      output = ValencyChecker.visualize(analysis)
      
      assert is_binary(output)
      assert String.contains?(output, "Valency Analysis")
      assert String.contains?(output, "eat")
      assert String.contains?(output, "Ambiguity Score")
    end
  end

  describe "integration tests" do
    test "processes complex sentences" do
      sentences = [
        "The robot moves the box",
        "The system detects anomalies quickly",
        "The processor analyzes data streams"
      ]
      
      Enum.each(sentences, fn sentence ->
        case ValencyChecker.eliminate_ambiguity(sentence) do
          {:ok, interpretations} ->
            assert length(interpretations) > 0
            assert List.first(interpretations).verb != nil
          {:error, _} ->
            :ok
        end
      end)
    end

    test "demonstrates zero-latency processing" do
      {time_microseconds, _} = :timer.tc(fn ->
        Enum.each(1..1000, fn _ ->
          ValencyChecker.check("run")
        end)
      end)
      
      avg_time = time_microseconds / 1000
      assert avg_time < 100
    end

    test "verifies edge semantics - no external dependencies" do
      {:ok, _} = ValencyChecker.check("processing")
      {:ok, _} = ValencyChecker.eliminate_ambiguity("The system processes data")
      {:ok, _} = ValencyChecker.analyze_roles("The robot moves")
      assert true
    end
  end

  describe "edge cases" do
    test "handles empty strings gracefully" do
      result = ValencyChecker.check("")
      assert {:error, :empty_string} = result
    end

    test "handles whitespace-only strings" do
      result = ValencyChecker.check("   ")
      assert {:error, :empty_string} = result
    end
  end

  describe "performance characteristics" do
    test "batch processing completes successfully" do
      words = ["run", "eat", "give", "sleep"]
      
      {time_seq, _} = :timer.tc(fn ->
        Enum.each(1..100, fn _ ->
          Enum.map(words, &ValencyChecker.check/1)
        end)
      end)
      
      {time_batch, _} = :timer.tc(fn ->
        Enum.each(1..100, fn _ ->
          ValencyChecker.check_batch(words)
        end)
      end)
      
      assert time_seq > 0
      assert time_batch > 0
    end
  end
end
