# #904 PAIR implementation checkpoint

The accounting component is implemented; the full experiment is **not complete**.

`adl/tools/pair_experiment.py` validates a pinned complete baseline/raw-PAIR/Runtime × healthy/node-loss × concurrency × repetition × request matrix. It derives denominators, failure counts, latency and completed-request throughput, compares against same-scenario baseline, and retains null/negative benefits. It checks declared two-node serving and concurrency overlap, rejects plan/provider/corpus drift, incorrect outputs, malformed resource references and absent node-loss observations. Every output explicitly says accounting does not establish qualification; untrusted measurement hashes do not authenticate a run.

Focused proof: `python3 adl/tools/test_pair_experiment.py` passes eight deterministic tests, including 20 receipt mutations and context, two-node, concurrency, no-benefit/failure and CLI channel/redaction cases. PVF: required local deterministic harness gate; small CPU/filesystem, no network, model, GPU or credentials. Existing provider code is unchanged. Compatibility log-file routing is not implemented or claimed. Output JSON is stdout; sanitized rejection diagnostics are stderr.

## Upstream identity and corrected environment boundary

- NVIDIA Personal AI Router v0.1.1, tag object `13b68115fa2c9c1d94f1ead1358f8d5a527cfecf`, resolved read-only with `git ls-remote`.
- [Official release](https://github.com/NVIDIA/Personal-AI-Router/releases/tag/v0.1.1).
- [Official getting started](https://docs.nvidia.com/local-ai/nvpair/getting-started/) documents local-only Ollama/OpenAI-compatible inference and macOS, Linux and Windows packages. PAIR routes independent requests; it does not shard a model or pool VRAM. The earlier setup restriction to NVIDIA-only nodes was too narrow; actual acceptance requires two approved compatible nodes.
- No PAIR installation, second approved node, selected model/license, endpoint, pairing state, resource ceiling or controlled failure scope has been verified. No network discovery, installation, model download or service mutation was performed.

## Current production integration boundary

Current Runtime kernel `assembly.rs` routes conversation work into `control::invoke_provider_conversation` / `invoke_provider_model`. The live model route presently supports Ollama; PAIR exposes an Ollama-compatible local proxy, which is a candidate integration path, not executed proof. `adl-provider-adapter` CLI alone is not the Runtime admission/conversation path and cannot satisfy that criterion. Canonical editable provider definitions from #876 must still be consumed through their actual validation/dispatch owner; accounting only binds their byte digest and does not replace that validation.

## Remaining required work

1. Pin two approved compatible nodes, actual PAIR/engine/model/license and tokenizer revisions, same corpus/sampling/warmth, node-loss operation and resource ceiling.
2. Wire and execute real bounded baseline, raw PAIR and current Runtime collectors; preserve raw observed request timing/output, serving-node and resource evidence. Accounting inputs are not a substitute for collectors or authenticated observation.
3. Execute concurrent requests and controlled real node unavailability/recovery, record failures and exclusions. Independently verify resource sample and node-event evidence bytes referenced by accounting inputs.
4. Obtain independent exact-head source/proof review and required CI, then make an evidence-bound keep/repair/retire decision. Do not publish a closing PR or mark #904 delivered from this checkpoint alone.
