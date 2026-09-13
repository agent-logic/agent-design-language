# #904 PAIR experiment decision

## Decision: REPAIR

PAIR works as a multi-node request router for ADL's existing Ollama provider path and is worth retaining for further development. It is not ready for production use without operational repairs around model residency, node lifecycle, security, observability, and reproducible provider settings.

The bounded experiment executed 72 correct requests across direct Ollama baseline, raw PAIR, and the production Runtime route; healthy and actual node-loss states; concurrency one and two; three repetitions; and two exact-output prompts. The canonical provider sidecar retained `type: ollama` and changed only its loopback `base_url` to PAIR. No PAIR provider type or production provider behavior was added.

While both nodes were healthy, PAIR selected the RTX 3090 for all Phi-4 requests. When that node was stopped and all PAIR ports became unreachable, the next request through the unchanged loopback endpoint completed on the Mac in 274 ms. All subsequent raw and Runtime requests completed correctly on the Mac. After the Windows node restarted, the unchanged endpoint selected it again and returned the expected output in 312 ms. This proves routing, failover, and recovery across both machines without implying pooled VRAM or model sharding.

## Performance evidence

| Workload and state | Concurrency | Mac baseline req/s | Heterogeneous PAIR-cluster req/s | Deployment ratio |
|---|---:|---:|---:|---:|
| DeepSeek-R1 8B, healthy, raw | 1 | 0.1226 | 0.2360 | 1.92x |
| DeepSeek-R1 8B, healthy, raw | 2 | 0.1226 | 0.3763 | 3.07x |
| Phi-4 Mini, healthy, raw | 1 | 2.4286 | 0.9863 | 0.41x |
| Phi-4 Mini, healthy, raw | 2 | 3.7596 | 17.7610 | 4.72x |
| Phi-4 Mini, healthy, Runtime | 1 | 2.4286 | 3.6058 | 1.48x |
| Phi-4 Mini, healthy, Runtime | 2 | 3.7596 | 6.4586 | 1.72x |
| Phi-4 Mini, node loss, raw | 1 | 2.5837 | 2.4382 | 0.94x |
| Phi-4 Mini, node loss, raw | 2 | 4.1287 | 0.2854 | 0.07x |
| Phi-4 Mini, node loss, Runtime | 1 | 2.5837 | 1.9589 | 0.76x |
| Phi-4 Mini, node loss, Runtime | 2 | 4.1287 | 3.2662 | 0.79x |

The negative rows are retained. Phi-4 healthy raw concurrency one contains a 5.783-second startup outlier. Node-loss raw concurrency two contains 10.280-second and 10.334-second outliers on the shared Mac. The heterogeneous PAIR cluster versus Mac-only baseline shows the clearest deployment gain on larger DeepSeek work; this does not isolate router overhead from the faster RTX hardware. Runtime end-to-end batch rates include workflow overhead; `RESULTS.json` separately reports request-window accounting from per-request broker timings.

## Limits and repairs

- Preload the exact model on every eligible node and maintain residency for the job window. Model loading dominates small batches.
- Make node admission, health, draining, recovery, and request attribution machine-readable before unattended use.
- Pin equivalent context and generation settings across engines. The experiment pinned identical model bytes and temperature zero, but Ollama versions and default context allocation differed; the Runtime sidecar currently exposes no seed or context-size field.
- Treat reasoning models separately. DeepSeek-R1 used the Runtime output budget for reasoning and returned no response text, which Runtime correctly rejected.
- Isolate inference nodes from unrelated workloads. Shared-host interference likely caused the node-loss outliers.
- Add authenticated/private transport and bounded ingress policy before using PAIR beyond a trusted LAN.

## AWS fleet mapping

PAIR can sit behind ADL's existing Ollama-compatible provider definition on a private AWS network. Each GPU instance remains an independent inference node; PAIR assigns whole requests across the fleet. A production experiment should use private subnets and security groups, immutable model images or a verified model cache, lifecycle hooks that preload and health-check a node before admission, draining before Spot interruption or scale-in, request-attribution logs, and concurrency-based autoscaling with explicit cost ceilings. This can increase review throughput and tolerate node loss, but it cannot combine GPU memory for a single oversized model.

No AWS resources or paid services were used by #904. This fleet design is follow-up scope, not a production-readiness claim.
