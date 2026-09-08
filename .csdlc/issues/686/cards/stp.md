# Structured Task Prompt

Template: 1.0.0

Issue: 686

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Close only the configuration-generation authority and crash-recovery gap remaining after #589 and #678.

## Deliverables

- Versioned immutable configuration-generation receipt
- Atomic active configuration reference
- Cross-component generation and digest validation/reporting
- Focused failpoint and prior-generation restoration proof

## Acceptance

1. AC-1: A committed receipt binds canonical content hash, schema version, redacted secret references, and compatible Runtime binary generation.
2. AC-2: One atomic active reference identifies the committed receipt and partial candidates never become authoritative.
3. AC-3: CSM, Guardian, kernel, status, and readiness expose and validate the same configuration generation and digest.
4. AC-4: Secret values never enter receipts, status, logs, or retained evidence.
5. AC-5: Invalid, missing, incompatible, rolled-back, cross-generation, or digest-mismatched receipts fail before service mutation.
6. AC-6: Isolated tests prove recovery before activation, after pointer replacement, after candidate readiness, and before commit cleanup.
7. AC-7: Candidate failure restores the prior committed generation and its receipt/reference authority.
8. AC-8: Focused tests, diff hygiene, and independent exact-head review pass before publication.

## Dependencies

- #589 / PR #598 is merged
- #678 / PR #682 is merged at d10de869f0c8a0ac660e12ad8561535d6a1878b6

## Inputs

- agent-logic/agent-design-language#686
- agent-logic/agent-design-language#589
- agent-logic/agent-design-language#678
- adl/src/cli/csm_runtime_v3_cmd.rs
- adl/tools/runtime_v3_generation.py
- adl-runtime-kernel/src
- .adl/docs/TBD/resilience/RUNTIME_V3_LAUNCH_AND_OBSERVATORY_RECOVERY_PLAN.md

## Non Goals

- Runtime binary installation redesign
- Convergence deadline changes
- Provider model agent or Observatory changes
- Live Runtime or cloud actions
- Broad configuration-format redesign
