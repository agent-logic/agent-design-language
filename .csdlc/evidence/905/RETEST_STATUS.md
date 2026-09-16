# #905 speculative-decoding requalification

Status: **complete; recommendation: retire speculative decoding from the current Runtime qualification path**. This is a qualification disposition, not service decommissioning or a production Runtime change.

## Current Runtime route and pinned environment

Candidate `1872cf04b6d2641f32ff63d58dd64385f69e5b5e` was built locally and exercised through Runtime v3's Observatory conversation path, provider registry, Ollama chat compatibility fallback, and `/api/generate`. The provider boundary forced `think=false` because the generic Runtime request does not yet carry Ollama's explicit thinking control and reasoning-only responses are invalid normal conversation output.

The run used Ollama `0.32.14` on an Apple M4 Pro with 14 CPU cores, 20 GPU cores and 64 GB unified memory. Both aliases referenced the same resident `Qwen3.5:9b` Q4_K_M model family and the same 15 embedded MTP tensors. Baseline set `draft_num_predict=0`; speculative set `draft_num_predict=4`. Both used temperature 0, seed 905, `num_predict=64`, the same two-prompt fixed-marker corpus and two repetitions. No model download, cloud call or paid resource occurred.

## Executed result

All four paired outputs were byte-equivalent and matched their declared correctness markers. Baseline completed in 30.865 seconds; speculative completed in 37.673 seconds. The measured ratio was `0.819x`, so speculative decoding was **18.07% slower** cold-inclusive. Excluding the first call in each arm, baseline took 23.066 seconds and speculative took 24.112 seconds, a `0.957x` ratio or **4.34% slower**. Engine decode throughput was 36.01 tokens/s baseline and 15.81 tokens/s speculative. Ollama did not expose accepted/proposed draft-token counters through this API, so none are claimed.

The controlled missing-draft model was rejected at admission. A subsequent ordinary baseline agent became communication-eligible and returned the exact expected marker, proving bounded healthy fallback. Failed harness attempts and the completed natural-language negative run are retained in `ATTEMPT_REGISTER.json`; the final machine-readable packet is `RUNTIME_RETEST.json`.

## Disposition

Retire speculative decoding from the current Runtime qualification path. The current MTP configuration preserves the fixed-marker outputs but produces no speed benefit and materially reduces decode throughput. Reconsideration requires a separately bounded issue with a newer engine/model pairing, accepted/proposed-token telemetry, and a fresh comparable Runtime run. No product integration, model deletion, or running-service change is performed by #905.

## Validation classification

- `python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py`: 13 deterministic accounting and negative-contract tests passed.
- `issue905_runtime_speculative_retest.py`: actual local hardware/Runtime lane, four paired comparisons, nonzero denominator, exact output checks, negative draft admission and healthy ordinary fallback.
- `git diff --check`: passed.
- Local evidence is hardware-dependent and release-supporting. Hosted CI remains integration proof for repository bytes; it cannot replace this local model execution.
