# Structured Task Prompt

Template: 1.0.0

Issue: 656

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only atomic matched-generation installation and pre-mutation verification for CSM, Guardian, and kernel.

## Deliverables

- Atomic generation installer
- Shared receipt verifier
- Atomic current activation and rollback
- CSM pre-mutation preflight
- Focused tests

## Acceptance

1. AC-1: One command stages and validates all three binaries
2. AC-2: One receipt binds revision, platform, profile, Runtime-init schema, and three SHA-256 hashes
3. AC-3: Activation atomically changes one current reference
4. AC-4: Prior complete generation remains rollback-capable
5. AC-5: launchd and Runtime-init resolve the same generation
6. AC-6: start and reload reject invalid generations before service mutation
7. AC-7: Negative proof shows rejection leaves service untouched
8. AC-8: Matched activation and rollback tests pass
9. AC-9: Excluded Runtime and cloud behavior remains unchanged
10. AC-10: Focused validation, hygiene, and exact-head review pass

## Dependencies

- Merged Runtime v3 Shepherd baseline
- .adl/docs/TBD/resilience/RUNTIME_V3_LAUNCH_AND_OBSERVATORY_RECOVERY_PLAN.md

## Inputs

- agent-logic/agent-design-language#656
- adl/tools/install_owner_binaries.sh
- adl/src/cli/csm_runtime_v3_cmd.rs
- adl-runtime-guardian/Cargo.toml
- adl-runtime-kernel/Cargo.toml

## Non Goals

- Provider or model configuration
- Convergence deadline policy
- Configuration-generation handoff
- Readiness taxonomy or agent identity
- Observatory, Caddy, cloud, Runtime v2, or live Runtime changes
