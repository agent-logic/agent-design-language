# Portable Engine And Contracts

## Metadata
- Feature Name: Portable bounded execution and durable records
- Milestone Target: `v0.91.8`
- Status: planned
- Owner: WP-06 and WP-07
- Doc Role: primary
- Supporting Docs: `../DESIGN_v0.91.8.md`
- Feature Types: runtime, artifact, policy
- Proof Modes: tests, schema, replay, review

## Template Rules

The engine must remain smaller than a runtime supervisor.

## Purpose

Execute canonical plans portably while retaining deterministic state,
failures, results, traces, artifacts, and trust evidence.

## Context

- Related milestone: `v0.91.8`
- Related issues: WP-06/WP-07 pending
- Dependencies: language/compiler and characterization contracts

## Coverage / Ownership

- Primary owner doc: this document.
- Covered surfaces: readiness, concurrency, retry, failure, joins, resume,
  ports, errors, events, artifacts, signing, verification.
- Related docs: CLI/adapters feature.

## Overview

The engine is a bounded state machine. It does not supervise processes,
discover services, own cloud clients, or implement cognitive services.

## Design

### Core Concepts

- Captured external outcome.
- Versioned execution transition and durable record.

### Architecture

- Inputs: plan, run inputs, provider/tool port outcomes.
- Outputs: events, result, artifact references, resumable state.
- Interfaces: `ProviderPort`, `ToolPort`, execution and trust schemas.
- Invariants: bounded concurrency, stable ordering, no unaudited action.

### Data / Artifacts

- execution events/results, artifact manifest, trace envelope, signed digest.

## Execution Flow

1. Admit a validated plan and captured inputs.
2. Select and run ready nodes under bounds.
3. Persist ordered transitions and terminal result.

## Determinism and Constraints

- Deterministic core after external inputs are captured.
- Saturation produces explicit backpressure/failure, never silent loss.

## Integration Points

| System / Surface | Integration Type | Description |
|---|---|---|
| Runtime v3 | trigger/observe | Supervises engine instance and consumes events. |
| Adapters | trigger | Supply typed provider/tool outcomes. |

## Validation

- Sequential/fork/join/retry/failure/resume and tamper cases.
- Schema and stdout/stderr contract checks.

## Acceptance Criteria

- Engine and contract budgets pass.
- Runtime authority remains external.

## Risks

- Convenience features may recreate supervision; reject them at review.

## Future Work

Additional schedulers or stores integrate through explicit versioned ports.

## Notes

Observability is evidence output, not hidden control authority.
