# Qwen/Gemma vLLM Speculative-Decoding Proof Publication - Issue #5075

Issue: `#5075 [v0.91.7][models][dspark] Publish Qwen and Gemma vLLM speculative-decoding proof`

Source issue: `#4653`

Source PR: `#5051`

## Purpose

This follow-up publishes the Qwen/Gemma vLLM evidence measured after #4653 had
already merged and closed. The retained measurement artifacts keep their
original `4653` filenames and `issue_number` fields for provenance; this note
is the #5075 publication surface that explains how to read them.

## Retained Evidence

- `DSPARK_SPECULATIVE_DECODING_EVALUATION_4653.md`
- `DSPARK_SPECULATIVE_DECODING_EVALUATION_4653.json`
- `VLLM_QWEN_TARGET_ONLY_BENCHMARK_4653.json`
- `VLLM_QWEN_SPECULATIVE_BENCHMARK_4653.json`
- `VLLM_GEMMA_SPECULATIVE_SMOKE_4653.json`
- `NATIVE_QWEN_ASSISTED_GENERATION_BENCHMARK_4653.json`
- `NATIVE_QWEN_ASSISTED_GENERATION_BENCHMARK_OFFLINE_4653.json`
- `GEMMA_QWEN_TOKENIZER_PROBE_4653.json`
- `VLLM_NATIVE_WINDOWS_AVAILABILITY_PROBE_4653.json`

## Result

Qwen vLLM draft-model speculative decoding worked as an execution path and
exposed accepted-token counters, but it was slower than target-only generation
for the measured `Qwen/Qwen2.5-1.5B-Instruct` target plus
`Qwen/Qwen2.5-0.5B-Instruct` draft pair:

- target-only median: `0.483s`
- speculative median: `1.317s`
- accepted tokens: `378`
- draft acceptance rate: `61.765%`

Gemma vLLM same-family speculative decoding did not reach generation. The
authenticated run loaded and compiled `google/gemma-3-1b-it` plus
`google/gemma-3-270m-it`, then failed during KV-cache initialization with:

```text
AssertionError: All drafting layers should belong to the same kv cache group
```

## Non-Claims

- This issue does not claim Qwen acceleration for the measured pair.
- This issue does not claim Gemma speculative decoding works in vLLM.
- This issue does not claim DSpark confidence-scheduled backend behavior.
- This issue does not claim DeepSeek-V4 DSpark live proof; that remains outside
  this publication issue.
- No provider secret value is printed or retained by the proof artifacts.
