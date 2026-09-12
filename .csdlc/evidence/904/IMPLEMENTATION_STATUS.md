# #904 PAIR implementation checkpoint

The accounting component and a real single-node preflight are implemented; the full experiment is **not complete**.

The verified NVIDIA v0.1.1 macOS arm64 headless archive started successfully with normal app-data access. All ten workers were healthy, including the cluster manager. Direct Ollama stayed on loopback port 11434 and PAIR safely selected loopback port 11435. After explicit equal prewarming with bounded 300-second model residency, a same-corpus comparison completed 24 of 24 raw requests correctly at concurrency one and two. At one node, PAIR throughput was lower in this small run: 0.9763x baseline at concurrency one and 0.7682x at concurrency two. This is a local transport/preflight result, not the required multi-node benefit result.

The new ignored `adl/tests/pair_provider.rs` live gate also ran two concurrent exact-output requests through the canonical provider reload owner, the production workflow executor, the existing `ollama` provider kind and PAIR's loopback proxy. Both passed at source `8d1609c68c7ecec4aee5a5ba146cd6d38d5f13b1`; the content-redacted receipt is `local-runtime-single-node.json`. `LOCAL_PREFLIGHT.json` binds artifact, platform, model, raw-comparison and Runtime receipt hashes without retaining the local host name, LAN address, prompts or response content.

`adl/tools/pair_experiment.py` validates a pinned complete baseline/raw-PAIR/Runtime × healthy/node-loss × concurrency × repetition × request matrix. It derives denominators, failure counts, latency and completed-request throughput, compares against same-scenario baseline, and retains null/negative benefits. It checks declared two-node serving and concurrency overlap, rejects plan/provider/corpus drift, incorrect outputs, malformed resource references and absent node-loss observations. Every output explicitly says accounting does not establish qualification; untrusted measurement hashes do not authenticate a run.

Focused proof: `python3 adl/tools/test_pair_experiment.py` passes sixteen deterministic tests, including 20 receipt mutations and context, two-node, concurrency, no-benefit/failure, matrix-allocation bound, CLI channel/redaction, local HTTP collector and bounded concurrent-batch cases. The batch collector preserves exact outputs for private correctness proof while marking route and node identity unverified; publishable projections retain hashes instead of content. PVF: required local deterministic harness gate; small CPU/filesystem, no external network, model, GPU or credentials. Existing provider code is unchanged. Compatibility log-file routing is not implemented or claimed. Output JSON is stdout; sanitized rejection diagnostics are stderr.

## Upstream identity and corrected environment boundary

- NVIDIA Personal AI Router v0.1.1, tag object `13b68115fa2c9c1d94f1ead1358f8d5a527cfecf`, resolved read-only with `git ls-remote`.
- [Official release](https://github.com/NVIDIA/Personal-AI-Router/releases/tag/v0.1.1).
- [Official getting started](https://docs.nvidia.com/local-ai/nvpair/getting-started/) documents local-only Ollama/OpenAI-compatible inference and macOS, Linux and Windows packages. PAIR routes independent requests; it does not shard a model or pool VRAM. The earlier setup restriction to NVIDIA-only nodes was too narrow; actual acceptance requires two approved compatible nodes.
- PAIR v0.1.1, Ollama 0.32.14, the Llama 3.2 3B blob, its community license and both loopback endpoints are pinned for the local preflight. One approved node is present. A second approved node, pairing state, final resource ceiling and controlled failure scope remain unverified. No model was downloaded and no paid or cloud resource was used.

## Current production integration boundary

Current Runtime kernel `assembly.rs` routes conversation work into `control::invoke_provider_conversation` / `invoke_provider_model`. The live model route presently supports Ollama; PAIR exposes an Ollama-compatible local proxy, which is a candidate integration path, not executed proof. `adl-provider-adapter` CLI alone is not the Runtime admission/conversation path and cannot satisfy that criterion. Canonical editable provider definitions from #876 must still be consumed through their actual validation/dispatch owner; accounting only binds their byte digest and does not replace that validation.

## Review correction and collector component

Independent reviewer sprint8_909 found an oversized plan could allocate millions of matrix keys before rejection. Plan admission now caps the matrix at 100,000 records before creating any Cartesian product. A regression replaces product allocation with a failing sentinel and verifies oversized input rejects first.

`collect_ollama` now makes one actual HTTP POST to an explicit numeric loopback Ollama-compatible endpoint, using explicit model/prompt/stream, plan sampling controls and bounded model residency. It does not parse or bypass canonical provider validation, identify a PAIR installation, invent a serving node or claim a Runtime route. An orchestration caller must obtain endpoint/model through the verified canonical configuration and capture route/node provenance. The function uses no HTTP proxy or redirects; a deadline interrupts stalled socket/header reads, and response bytes are capped at 1 MiB. Tests use actual loopback sockets with fake responses to prove argv-equivalent request bytes, output identity, rejection, deadline and resource behavior, not inference. Review also found that mirroring Runtime keep_alive=-1 and omitted sampling would violate bounded/comparable raw collection. The collector now requires temperature=0, seed and positive bounded max_tokens, transmits Ollama options, defaults residency to zero and permits only explicit 0..300-second residency. Current Runtime still omits these controls and sends keep_alive=-1; actual Runtime comparison therefore needs an approved equivalent effective-setting/resource strategy or a separately scoped repair. No Runtime equivalence is inferred.

The live Runtime probe covers an actual bounded concurrent batch through canonical definitions. Full hardware orchestration and authenticated per-request serving-node evidence are not implemented yet; the collector, Runtime probe and accounting remain building blocks until a second approved node is paired and the complete matrix runs.

## Remaining required work

1. Pin two approved compatible nodes, actual PAIR/engine/model/license and tokenizer revisions, same corpus/sampling/warmth, node-loss operation and resource ceiling.
2. Wire and execute real bounded baseline, raw PAIR and current Runtime collectors; preserve raw observed request timing/output, serving-node and resource evidence. Accounting inputs are not a substitute for collectors or authenticated observation.
3. Execute concurrent requests and controlled real node unavailability/recovery, record failures and exclusions. Independently verify resource sample and node-event evidence bytes referenced by accounting inputs.
4. Obtain independent exact-head source/proof review and required CI, then make an evidence-bound keep/repair/retire decision. Do not publish a closing PR or mark #904 delivered from this checkpoint alone.
