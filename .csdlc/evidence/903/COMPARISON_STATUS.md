# Operator-requested MLX comparison status

The operator requires a successful real review from both matched providers before comparing performance. Faster invalid reviews are not accepted evidence.

## Completed exploratory work

- A shared FP16 export from cached Llama-3.2-3B-Instruct four-bit weights was loaded by MLX-LM and Ollama. All 254 learned tensors were independently checked for equal numeric values. GGUF normalization vectors use F32 exact promotion; engine arithmetic differs.
- Short matched prompts produced equal output strings, but two prompts failed instruction following or reached the deliberately short output limit. These measurements do not establish successful-review acceleration.
- A real 212-line adapter review with 2206 input tokens and512 output-token ceiling completed on both engines. All three asserted findings were rejected by independent review. Increasing the ceiling to2048 did not establish valid reviews.
- General Qwen3 8B and14B qualification and Qwen3-Coder30B qualification also produced no accepted review result. The 8B thinking trial exhausted4096 generated tokens without delivering a review. These are failed qualification attempts, not evidence of a successful matched benchmark.
- Historical #3501 and #3415/#3503 packets distinguish general Qwen models from local qwen3-coder:30b. Their useful bounded worker/checklist result is not broad review-quality acceptance. Retain that distinction when reusing their settings.

## Required remaining work

Qualify a real review against independent ground truth, then compare identical model weights, rendered inputs, effective generation controls, and review budgets across MLX and non-MLX. Preserve failed trials, complete responses, input hashes, timing, resource differences, and cold/warm status. Run order and output lengths must be reported. No acceleration or successful-review claim is currently established.

The observational comparison artifacts are private under .adl/runs/903/comparison. This supplemental experiment does not substitute for the separately passing canonical production-workflow Metal test in actual-hardware.json. Direct endpoint probes do not prove the registered ADL adapter or resident-kernel integration.
