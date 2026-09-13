# #904 PAIR implementation status

The required two-node experiment is complete in the bound worktree. The reviewed disposition is **REPAIR**: retain PAIR as a promising multi-node request-routing substrate for ADL, then address operational controls before production use.

The run used NVIDIA PAIR v0.1.1 on an Apple M4 Pro node and an RTX 3090 node, with identical resident Phi-4 Mini model bytes. It executed a complete 72-request matrix across direct Ollama baseline, raw PAIR, and the current production Runtime route; healthy and actual node-loss states; concurrency one and two; three repetitions; and two exact-output prompts. All 72 requests returned the expected output. The Runtime route consumed the canonical provider reload sidecar through the existing `ollama` provider kind and PAIR loopback endpoint. No PAIR provider type or production provider behavior changed.

Broker attribution proves the actual topology. Healthy Phi-4 traffic selected the faster RTX node. After its PAIR process stopped and its required ports became unreachable, the next request through the unchanged endpoint completed on the Mac in 274 ms. All node-loss raw and Runtime work then ran on the Mac. After restart, the unchanged endpoint selected the RTX node again and returned the expected output in 312 ms.

The result is workload sensitive. A 24-request DeepSeek-R1 8B raw comparison measured the heterogeneous PAIR cluster at 1.92x the Mac-only baseline throughput at concurrency one and 3.07x at concurrency two, with work distributed over both healthy nodes. The heterogeneous Phi-4 cluster measured 4.72x the Mac-only baseline for healthy raw concurrency two, while a 5.783-second startup outlier reduced its concurrency-one result to 0.41x. The production Runtime path through the heterogeneous cluster measured 1.48x and 1.72x the Mac-only end-to-end healthy throughput at concurrency one and two. After node loss, work remained correct but generally slower than direct baseline, including two roughly 10.3-second raw outliers on the shared Mac. These negative results are retained.

Tracked evidence:

- `PLAN.json`: exact candidate, PAIR/model/provider/corpus/settings/resource bounds.
- `RESOURCE_SAMPLES.json` and `OBSERVED_RESOURCE_SAMPLE.json`: redacted capacity, engine, residency, bounded post-run utilization, and cost facts.
- `MEASUREMENTS.json`: complete 72-record matrix with node-event bindings.
- `RESULTS.json`: deterministic completeness, arithmetic, failure, latency, throughput, and topology accounting.
- `SUPPLEMENTAL_RESULTS.json`: end-to-end batch metrics, larger-model results, failure observations, and hashes of ignored private inputs.
- `EXPERIMENT_REPORT.md`: execution details, PVF classification, and evidence boundaries.
- `DECISION.md`: reviewed keep/repair/retire decision, limitations, and AWS fleet mapping.

Focused validation passes 18 deterministic Python tests, one exact Rust workflow-shape test, strict Clippy for the live test target, JSON parsing, accounting replay, and `git diff --check`. The hardware-dependent Runtime receipts and broker traces were collected at candidate `6edeb3e728709b5d3d83ca142419efa8ef46f3d4`; subsequent source changes only correct the deterministic accounting oracle so a fastest-node scheduler plus observed failover proves a two-node topology. The production Runtime integration code is unchanged.

Remaining work is lifecycle review, publication, and CI. AWS scaling, authenticated private transport, node admission/draining, model distribution/residency, interruption handling, and autoscaling with cost ceilings are explicit follow-up scope. PAIR routes whole requests and does not pool VRAM or shard one model across nodes.
