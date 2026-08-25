# Distributed Multi-Agent Runtime Qualification — v0.92.1

Qualify multiple distinct resident agents performing governed collaborative work. UTS supplies a concrete workload; it is not a separate Runtime architecture. Proof covers identity, task division, continuity, restore, model/provider truth, resource use, failure handling, and exact completion receipts. Runtime v4 changes require explicit rebaseline.

## Retained predecessor denominator

Closed planning issues `#181`-`#187` were retired before execution and are not reopened. WP-01 will create three serial packages, DRT-A through DRT-C, that retain their complete requirements:

1. **DRT-A (`#181`, `#182`)** — freeze the qualification contract and prove deterministic ACIP authority and replay conformance.
2. **DRT-B (`#183`, `#184`)** — run distinct-agent UTS production work and hybrid local/AWS continuity with truthful paid-proof boundaries.
3. **DRT-C (`#185`, `#186`, `#187`)** — qualify identity/TLS/provider failures, coherent Observatory evidence, soak, replay, resource accounting, cleanup, and synthesis.

Existing issue `#345` is promoted into this lane for AWS GPU Shepherd hardening. It may run in parallel with DRT-A and the Observatory prerequisites; DRT-B and DRT-C consume its reviewed merged evidence only for GPU-backed qualification claims. Closeout is asynchronous.
