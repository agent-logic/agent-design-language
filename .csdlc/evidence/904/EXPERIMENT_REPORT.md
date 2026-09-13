# #904 two-node experiment report

## Pinned execution

- Candidate source: `6edeb3e728709b5d3d83ca142419efa8ef46f3d4`
- NVIDIA Personal AI Router: v0.1.1, source `13b68115fa2c9c1d94f1ead1358f8d5a527cfecf`, broker v0.40.2
- Mac archive SHA-256: `6080b89e2c8e83e842e5a163f021f4cae5747bc9c9f371af481d95550a175d8f`
- Windows archive SHA-256: `8248b7c0c5175075c565369f9af67f7614f1fa951ab31374483707eeaf8dac16`
- Model: `phi4-mini:latest`, identical manifest digest `78fad5d182a7c33065e153a5f8ba210754207ba9d91973f57dffa7f487363753` and blob digest `3c168af1dea0a414299c7d9077e100ac763370e5a98b3c53801a958a47f0a5db` on both nodes; embedded MIT license verified with `ollama show --license`
- Nodes: Apple M4 Pro/64 GiB and RTX 3090 24 GiB/Core i9/128 GiB; publishable details are in `RESOURCE_SAMPLES.json`
- Transport: trusted LAN PAIR cluster; Mac loopback proxy `127.0.0.1:11435`; narrow Windows LocalSubnet firewall allowance for PAIR
- Canonical Runtime configuration: existing `ollama` provider type, PAIR loopback base URL, temperature 0, one Runtime attempt, 64 output tokens, 120-second timeout
- Matrix: two exact-output prompts, three repetitions, concurrency 1 and 2, warm models, 120-second request bound, 500-second run bound, zero cloud cost

## Executed routes and outcomes

The raw collector used the same model, prompts, temperature, token ceiling, warmth policy, repetition count, and concurrency for direct Ollama and PAIR. The ignored Rust live gate loaded the pinned canonical provider definition through `ProviderReloadOwner` and ran the production concurrent workflow executor. Four content-redacted Runtime receipts cover healthy and node-loss states at both concurrency levels. All 72 qualification records were correct and within bounds.

PAIR's supported broker events supplied the actual node for every PAIR and Runtime request. Healthy Phi-4 traffic selected the RTX 3090. After controlled shutdown of the remote PAIR process, all required remote ports were unreachable and the unchanged local PAIR endpoint routed to the Mac. Restart restored routing to the RTX node. The broker trace, raw responses, prompts, addresses, and host identifiers remain in ignored private evidence; tracked files retain aliases and SHA-256 bindings.

A supplementary DeepSeek-R1 8B raw run completed 24 of 24 requests and exercised both healthy nodes. The heterogeneous PAIR cluster delivered 1.92x and 3.07x the Mac-only baseline throughput at concurrency one and two. This measures the deployment, not PAIR software alone; no same-RTX direct control was run. Its Runtime run failed truthfully because the model consumed the 256-token cap as reasoning and returned empty response text.

## Evidence boundaries

`MEASUREMENTS.json` contains normalized monotonic offsets so the deterministic accounting tool can verify ordering and concurrency without publishing wall-clock or host identity. Request durations come from the actual raw client observations or PAIR broker completions. `SUPPLEMENTAL_RESULTS.json` retains end-to-end batch throughput and hashes for every ignored private input. The deterministic accounting tool validates completeness and arithmetic; it labels its output as non-qualifying because the reviewed experiment packet, rather than self-asserted input, establishes qualification.

Hardware use was bounded through two approved already-owned nodes, declared memory/VRAM ceilings, concurrency at most two, preloaded resident models, and zero cloud spend. `OBSERVED_RESOURCE_SAMPLE.json` retains a bounded post-run snapshot: the RTX node held the tested model in 7,863 MiB of its 24,576 MiB VRAM and drew 29.39 W while idle; the Mac snapshot records VM page counts and shared-host activity. No per-request power or utilization time series was collected, so the experiment makes no efficiency claim.

## PVF classification

- Deterministic harness/accounting and negative tests: provider lane; correctness and fail-closed proof; deterministic; small CPU/filesystem; required release gate.
- Actual raw PAIR and Runtime matrix: provider lane; integration and performance characterization; hardware/network dependent; two approved local nodes; required experiment gate.
- Controlled node loss and recovery: provider lane; resilience proof; hardware/network dependent; bounded manual service transition; required experiment gate.
- DeepSeek supplemental run: provider lane; workload-sensitivity evidence; hardware/network dependent; informative, not a separate release gate.
