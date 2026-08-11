# v0.92.1 Quality Gate

| Gate | Pass condition | Fail-closed condition |
| --- | --- | --- |
| Corporate chain of title | Every critical scheduled asset has counsel-reviewed transfer and corporate acceptance evidence | Any critical in-scope asset deferred, ambiguous, or unsupported |
| Operational custody | Company identities control production billing, recovery, domains, cloud, deployment, monitoring, and rollback | Required personal-account dependency remains |
| v3 architecture fidelity | PR #77 contract and all eleven decisions preserved | Silent architecture change or missing hard gate |
| v3 state and recovery | One commit point, deterministic projections, interruption recovery, supported-platform proof | Dual authority, data loss, unsupported mutation |
| v3 remote lifecycle | Exact review, durable intent, idempotent readback, foreground cancellation, terminal truth | Stale review, duplicate mutation, hidden watcher |
| v3 cutover | Parity, canary, writer fence, authority scan, rollback rehearsal | v2 and v3 writable together; V3-R01 folded into cutover |
| Runtime topology | Exactly three voters, three governed agents, one shepherd, one leased Observatory | In-process substitute, shared state, synthetic projection |
| Runtime resilience | Exact `#142` merge ancestry; `3 -> 2 -> 1` behavior; old lease expiry; snapshot restore; AWS-only continuity; replay; per-phase cleanup | Stale leader writes, one-voter mutation, replay divergence, cleanup only at final exit |
| Runtime security | Key separation, non-voting shepherd identity, capability, TLS, stale lease/fence, cross-polis replay, provider, malformed-message, and pre-auth disclosure negatives pass | Plaintext, public endpoint, forged or stale authority accepted |
| Evidence | Producer-derived exact-revision receipts independently recompute | Hard-coded totals, skipped tests, screenshots-only evidence |
