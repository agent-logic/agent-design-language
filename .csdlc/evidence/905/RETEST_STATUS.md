# #905 speculative-decoding requalification

Status: **complete; recommendation: keep speculative decoding qualified for the declared current Runtime/model/hardware configuration**. This is a bounded qualification result, not a claim for every model, engine, prompt corpus, or machine.

## Current Runtime route and pinned environment

Runtime candidate `1872cf04b6d2641f32ff63d58dd64385f69e5b5e` was built locally and exercised through Runtime v3's Observatory conversation path, provider registry, Ollama chat compatibility fallback, and `/api/generate`. The provider boundary forced `think=false` because the generic Runtime request does not yet carry Ollama's explicit thinking control and reasoning-only responses are invalid normal conversation output.

The final run used Ollama `0.32.14` on an Apple M4 Pro with 14 CPU cores, 20 GPU cores and 64 GB unified memory. The harness created both aliases from the same immutable GGUF blob (`dec52a44569a2a25341c4e4d3fee25846eed4f6f0b936278e3a3c900bb99d37c`), verified identical full model metadata and tokenizer metadata hashes, verified the same 15 embedded MTP tensors, and failed unless normalized configurations differed only in `draft_num_predict`. Baseline used `0`; speculative used `4`. Temperature 0, seed 905, `num_predict=64`, the two-prompt fixed-marker corpus, and the exact same Runtime agent identity were shared. No model download, cloud call, or paid resource occurred.

## Executed result

Four counterbalanced A/B and B/A blocks produced eight measured conversations per arm after a symmetric direct preload and excluded Runtime prewarm at every arm switch. All eight paired outputs were byte-equivalent and matched their declared correctness markers. Baseline completed in 59.395 seconds; speculative completed in 52.569 seconds. The measured ratio was `1.130x`, so speculative decoding was **12.98% faster end-to-end**. Engine decode throughput was 13.86 tokens/s baseline and 16.26 tokens/s speculative, a **17.29% increase**. Both arms decoded 60 output tokens. Runtime's dynamic envelope yielded 15,696 baseline and 15,694 speculative prompt tokens, a 2-token (`0.013%`) difference recorded in the packet rather than hidden. Ollama did not expose accepted/proposed draft-token counters through this API, so none are claimed.

An alias derived from the valid resident target with `draft_num_predict=invalid` was rejected during Ollama configuration before Runtime admission. The harness then admitted the ordinary baseline configuration through the same Runtime route and delivered the exact expected marker. This proves fail-closed invalid-draft handling and healthy **operator-selected** ordinary-generation recovery; it does not claim automatic Runtime fallback.

All failed and superseded attempts are retained in `ATTEMPT_REGISTER.json`; the final machine-readable packet is `RUNTIME_RETEST.json`.

## Disposition

Keep speculative decoding qualified for this current Runtime, resident `Qwen3.5:9b` MTP model, Ollama `0.32.14`, and Apple M4 Pro configuration. The final comparable run preserved exact outputs while improving measured end-to-end latency and decode throughput. Broader adoption should retain per-model/hardware qualification. A future engine that exposes accepted/proposed draft-token counters can strengthen mechanism-level attribution.

## Validation classification

- `python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py`: 13 deterministic accounting and negative-contract tests passed.
- `issue905_runtime_speculative_retest.py`: actual local hardware/Runtime lane; immutable model/tokenizer identity checks; eight paired comparisons; symmetric preload/prewarm; counterbalanced ordering; exact output checks; invalid draft rejection; healthy operator-selected ordinary recovery.
- `python3 -m py_compile adl/tools/issue905_runtime_speculative_retest.py`: passed.
- `git diff --check`: passed.
- Local evidence is hardware-dependent and release-supporting. Hosted CI remains integration proof for repository bytes; it cannot replace this local model execution.
