# Structured Task Prompt

Template: 1.0.0

Issue: 659

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Change only Runtime-v3 service convergence policy and its focused proof; exclude providers, model execution, Observatory, Caddy, cloud, and Runtime v2.

## Deliverables

- validated convergence-policy configuration
- stage-specific service-control convergence and diagnostics
- focused deterministic tests for slow success, expiry, invalid configuration, and recovery
- strict Clippy, formatting, diff hygiene, and exact-head review evidence

## Acceptance

1. AC-1: Stop, unload, listener, and readiness limits are named validated configuration values with generous defaults.
2. AC-2: Configuration supports 5-10 minute model-backed startup without tiny operational deadlines.
3. AC-3: Reload remains continuously service-manager owned with no direct competing process.
4. AC-4: Deadline expiry identifies the exact stage and preserves a recoverable last-known service state.
5. AC-5: Focused tests prove slow success, true expiry, invalid bounds, rollback or recovery, and absence of fixed 15-second waits.
6. AC-6: General API request timeout semantics remain unchanged.
7. AC-7: No live Runtime restart occurs during implementation validation.

## Dependencies

- Issue #656 / PR #658 atomic Runtime generation installation merged and ancestral to current main cea5219f6e74b34d930d0dc39b6a607bc6303acb

## Inputs

- agent-logic/agent-design-language#659
- adl/src/cli/csm_runtime_v3_cmd.rs
- adl-runtime-kernel/src/config.rs
- Issue #656 / PR #658 merged prerequisite

## Non Goals

- general API request timeout changes
- provider or model execution redesign
- configuration-generation handoff
- canonical identity
- Caddy, Observatory, cloud, or Runtime v2 changes
- live Runtime restart
