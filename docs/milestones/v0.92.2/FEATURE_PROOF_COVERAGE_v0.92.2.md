# v0.92.2 Feature and Proof Coverage

Status: planned ownership map.

| Exit-bar surface | Owner | Required proof |
|---|---|---|
| Shell, setup, onboarding, controls | CF-SHELL | Operator journey and failure-state tests |
| Local/GitHub/CI ingestion | CF-ADAPTER | Portable fixture conformance |
| Stable evidence, provenance, redaction, retention | CF-EVIDENCE | Determinism, tamper, redaction, retention suites |
| Dependencies, boundaries, coupling, connascence | CF-COG | Grounded architecture fixtures |
| Drift, blast radius, quanta, ADR/rationale | CF-COG | Explanation traceability and reviewer calibration |
| Fitness functions and CI | CF-GOV | Passing/failing deterministic policy fixtures |
| Correctness perspective | CF-REVIEW | Attributed review fixture |
| Security perspective | CF-REVIEW | Attributed security fixture |
| Adversarial perspective | CF-REVIEW | Attributed misuse/failure fixture |
| Constitutional perspective | CF-REVIEW | Attributed policy-value fixture |
| Synthesis, remediation, test planning | CF-REVIEW | Deduplication, severity, plan-schema proof |
| Longitudinal second run | CF-MEMORY | Two-run and compatibility fixtures |
| Human publication controls and governance metadata | CF-UX | Approval-negative and manifest validation |
| Markdown, HTML, PDF | CF-UX | Renderer-parity proof |
| Docs, examples, fixtures | CF-PROOF | Fresh-operator walkthrough |
| ADL self-review | CF-PROOF | Retained review packet |
| External OSS proof | CF-PROOF | Licensed, bounded retained packet |
| Complete Beta 1 | CF-INTEGRATE | Exit-bar reconciliation and end-to-end failure matrix |
| Config-driven providers | PLAT-PROVIDER | Schema negatives and provider parity after merged #622 |
| MLX/Metal adapter | PLAT-MLX | Bounded platform smoke and unsupported-platform failure |
| UTS productization | PLAT-UTS | Schema conformance and supported-consumer fixture |
| Recurring Rust reduction | PLAT-RUST | Behavior parity and before/after measurement |
| AWS inventory maintenance | OPS-AWS | #484 baseline comparison, business-account readbacks, and redaction validation |
| Medium article preparation | PUB-MEDIUM | Source/citation traceability and non-publication check |
| C-SDLC paper preparation | PUB-CSDLC | Source/citation traceability and non-submission check |
| Memory Palace production slice | PLAT-MEMORY | Production caller, deterministic retrieval, redaction negatives |
| Speculative-decoding decision | SPEC-RETEST | Current benchmark, equivalence, and fallback proof |

No row may be marked proven from a planned demo, a zero-test invocation, or green CI that does not cover the stated behavior.
