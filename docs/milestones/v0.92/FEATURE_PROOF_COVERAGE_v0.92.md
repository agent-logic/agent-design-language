# v0.92 Feature and Proof Coverage

## Metadata

- Milestone: `v0.92`
- WP owner: `WP-20`
- Current issue: `agent-logic/agent-design-language#308`
- Legacy predecessor: `danielbaustin/agent-design-language#5840`
- Purpose: reconcile proof ownership and release-gate posture without
  implementing feature work or promoting planned rows to passed evidence.

## Coverage Status Vocabulary

| Status | Meaning | Release-gate posture |
| --- | --- | --- |
| `accepted` | Exact-revision evidence, required negative proof, and review are linked in the artifact index. | May support downstream quality/release gates. |
| `blocked_with_evidence` | The owner and blocker are named; the row is explicitly not accepted release evidence. | Blocks release until resolved or scoped out. |
| `deferred_non_claim` | The row is outside the current release claim or owned by a later sprint. | Does not support a release claim. |
| `planned` | Scheduling exists but exact-revision proof is absent. | Non-accepted; downstream gates must fail if this is required product scope. |

## Feature Coverage Table

| Outcome | Owner | Proof route | Status | Artifact index row |
| --- | --- | --- | --- | --- |
| Canonical milestone and version truth | WP-01, WP-01B | Issue graph, six-card inventory, docs/version parity | blocked_with_evidence | AEE-001 |
| Agent Logic repository copies | WP-02 | Source-before/destination-after/source-after manifests, Git/LFS parity, destination configuration checks, and source-immutability verification | blocked_with_evidence | AEE-002 |
| Reliable CI and coverage | WP-02A | Lane-selection regressions, coverage aggregation, platform checks | blocked_with_evidence | AEE-003 |
| Evidence-based build acceleration | WP-02B | Same-SHA standard/16-core trials, proof parity, canary, cost decision, and fallback or cleanup | blocked_with_evidence | AEE-004 |
| Resilient local Runtime | WP-03 | Start, stop, recovery, configuration, clean-log, and failure injection proof | blocked_with_evidence | AEE-005 |
| Distributed Guardian/polis | WP-04 | Architecture/security review and distributed child proof receipts | blocked_with_evidence | AEE-006 |
| Faster C-SDLC and remote validation | WP-05, WP-06, WP-07 | Cycle-time comparison, portable runner proof, typed-card parity | blocked_with_evidence | AEE-007 |
| Birthday and identity | WP-08, WP-09, WP-10 | Birth negative cases, stable identity, bounded-cycle continuity, and #451 authenticated exactly-once production composition/restart proof | implemented_with_evidence | AEE-008 |
| Memory and capability | WP-11, WP-12 | Grounded/redacted memory and capability-envelope validation | blocked_with_evidence | AEE-009 |
| Memory Palace production authority | WP-11 | `adl-runtime-kernel::memory_palace` production authority, `adl-runtime::memory_palace` retained checkpoint/latest/journal service, `adl::memory_palace` compatibility adapter, issue `#450`, PR `#458`, C-SDLC generation 24 digest `c4c198d48a58cff340854f8269ac1644b1e0b09f901dfb60f815ecf782f14968`, and CI run `32456967817` | blocked_with_evidence | AEE-009 |
| Cognitive profile and adaptation queue | WP-13, WP-13A | Evidence-grounded profile fixtures and current Runtime loop qualification | blocked_with_evidence | AEE-010 |
| ACIP/A2A transport | WP-14 | Reconciled contracts, protobuf/JSON parity, authenticated full-duplex WSS | blocked_with_evidence | AEE-011 |
| Witness, receipt, and review packet | WP-15, WP-16 | Witness/receipt validation and integrated reviewer packet | blocked_with_evidence | AEE-012 |
| Cross-polis continuity | WP-17 | Migration semantics and explicit infrastructure non-goals | blocked_with_evidence | AEE-013 |
| Demonstrable birthday | WP-18 | Runnable positive and negative birthday proof | blocked_with_evidence | AEE-014 |
| Observatory and Unity consumers | WP-18A | Real versioned API/WSS interactions, compatibility matrix, and consumer failures | blocked_with_evidence | AEE-015 |
| Provider-neutral multi-agent execution | WP-18B | Real multi-provider runs, ACIP traces, negative cases, and no-substitution proof | blocked_with_evidence | AEE-016 |
| v0.93 governance handoff | WP-19 | Traceable downstream evidence map | blocked_with_evidence | AEE-017 |
| Demo matrix and proof coverage | WP-20 | This matrix, AEE artifact index, activation ledger, and fail-closed validator | blocked_with_evidence | AEE-018 |
| Reduction and refactoring | WP-21, WP-21A | Deletion eligibility, net reduction, behavior-preserving Rust checks | planned | AEE-019 |
| Quality, release, and publication | WP-22 through WP-30 | Review packets, remediation, release evidence, articles, podcasts, ceremony | planned | AEE-020 |

## Coverage Rule

An outcome becomes `accepted` only when the artifact index row links
exact-revision implementation, positive validation, required negative proof,
review state, and integration evidence. An open issue, initialized card bundle,
fixture, local transcript, or planned command proves scheduling only.

WP-22 must fail the quality gate if any required product-feature row remains
`planned`, lacks accepted exact-revision evidence, or relies on fixtures,
synthetic success, provider substitution, or unreviewed local-only claims.

## WP-20 Boundary

WP-20 may mark its own reconciliation and validator row accepted only after the
exact-head typed review and publication loop records terminal review truth. The
pre-publication working copy remains `blocked_with_evidence` so downstream gates
cannot consume a local branch as release evidence. WP-20 must not convert WP-18,
WP-18A, WP-18B, WP-19, WP-21, or release-tail rows to accepted unless their own
issue evidence is exact, reviewed, and linked.

## WP-22A Corrective Hydration

The #467 corrective quality gate grants accepted release credit only to rows with complete canonical hydration. Current accepted rows are feature:ADAPTIVE_LEARNING_DAG_v0.92, feature:FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92, critical:AEE-008; all other feature and critical-path rows remain non-credit blockers or planned/deferred non-claims as recorded in `docs/reviews/v0.92/quality-gate-467/feature-completion-matrix.json`.
