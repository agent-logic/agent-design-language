# v0.91.8 Design

## Metadata
- Milestone: `v0.91.8`
- Version: `v0.91.8`
- Date: `2026-07-14`
- Owner: ADL maintainers
- Related issues: `#5335` and the v0.91.8 issue wave

## Purpose

Define the clean-room ADL product, its contracts, budgets, migration gates, and
deletion outcome.

## Problem Statement

The incumbent `adl` crate contains about 621 Rust files and 355,675 lines,
including roughly 120,597 lines under `cli/`, 80,232 under `runtime_v2/`, 32
binaries, and 3,325 test annotations. Its public module surface mixes language,
execution, runtime, C-SDLC, providers, cognitive features, demos, and proof
tools. This raises change cost and obscures authority.

## Goals

- Independently implement a small typed ADL product.
- Preserve declared behavior through normalized black-box parity.
- Target 90% incumbent deletion; fail below 80%.
- Keep runtime and lifecycle products independent.

## Non-Goals

- Port every legacy internal abstraction or test.
- Add cognitive, provider, or birthday features.
- Achieve reduction by moving unchanged code into new crates.
- Delete before rollback and exact-revision review succeed.

## Scope

### In Scope

- Language parsing, schema, validation, and canonicalization.
- Pure compilation to `ExecutionPlan`.
- Portable bounded execution semantics.
- Versioned artifact, trace, result, and error contracts.
- Thin CLI, ports, adapters, selector, parity, cutover, and deletion.
- Integrated acceptance and deployment contracts for ADL v2, Runtime v3, and C-SDLC v2.

### Out Of Scope

- Runtime supervision, long-lived cognition, continuity services, and runtime APIs.
- C-SDLC cards, worktrees, GitHub publication, and closeout.
- Milestone-specific demos inside the product dependency graph.

## Requirements

### Functional

- Parse and validate the six primitives with fail-closed unknown-field handling.
- Resolve references and compile deterministic plans with stable node IDs.
- Execute plans with bounded concurrency and explicit retry/failure/join/resume semantics.
- Expose provider and tool behavior through typed ports.
- Emit stable machine-readable diagnostics, traces, artifacts, and results.

### Non-Functional

- Deterministic behavior and reproducible outputs.
- Clear failure semantics and stdout/stderr separation.
- No more than four core crates and five installed owner binaries.
- Target 20k implementation LoC; 30k hard ceiling.
- Target 8k test LoC; 15k hard ceiling.
- Focused warm validation under two minutes; deterministic full validation under ten.

## Proposed Design

### Overview

```text
ADL YAML/JSON
  -> adl-language: typed document, schema, semantic validation
  -> adl-compiler: resolution, composition, deterministic ExecutionPlan
  -> adl-engine: bounded state machine and provider/tool ports
  -> thin adl CLI and independently owned adapters
```

### Interfaces And Contracts

- `AdlDocument`: versioned six-primitives source contract.
- `ExecutionPlan`: canonical compiler output with stable node identity.
- `ExecutionEvent`, `ExecutionResult`, `ArtifactRef`: portable engine records.
- `ProviderPort` and `ToolPort`: capability-scoped asynchronous interfaces.
- `GenerationSelector`: authoritative v1/v2 selection during migration.

### Ownership Boundary

| Capability | Owner |
|---|---|
| Language/schema/compiler | new ADL product |
| Portable execution semantics | new ADL engine |
| Process/component supervision | Runtime v3 |
| Cognitive and continuity services | Runtime v3 components |
| Issue/PR/card lifecycle | C-SDLC v2 |
| Provider/cloud mechanics | adapter products |
| Demos and milestone proof | demo/proof workspace |

### Execution Semantics

The compiler is pure. The engine receives a captured plan and explicit inputs,
selects ready nodes using deterministic ordering, enforces a concurrency bound,
and records every external result before it affects later decisions. Runtime v3
may supervise the engine but cannot change plan semantics implicitly.

### Migration State Machine

```text
baseline -> characterization -> construction -> shadow parity
  -> opt-in soak -> reversible default -> rollback window -> deletion
  -> three-product acceptance/deployment -> canonical closeout tail
```

Every transition records exact revision, normalized outcome, budget evidence,
review state, and residual risk.

### Acceptance And Deployment Boundary

WP-14 accepts the three owner products as one deployed platform without
collapsing their authority boundaries. It proves stable installation,
configuration, readiness, operations, recovery/rollback, evidence retention,
consumer integration, and the exact v0.92 handoff. Runtime v3 must run through
its approved service topology. C-SDLC v2 must prove the full init-to-closeout
lifecycle and installed skill/operator surface. ADL v2 must be the accepted
default after parity, rollback, and deletion.

## Risks And Mitigations

- Risk: parity corpus preserves accidental bugs.
  - Mitigation: classify mismatches as v1 defect, v2 defect, intentional change, formatting difference, or unsupported case.
- Risk: adapters recreate the monolith.
  - Mitigation: default builds exclude integration dependency graphs and enforce dependency budgets.
- Risk: early deletion removes uncharacterized behavior.
  - Mitigation: pinned module-to-capability closure and approval-gated deletion manifest.
- Risk: v0.92 pressure bypasses soak.
  - Mitigation: v0.92 consumption requires explicit v0.91.8 cutover evidence or a recorded blocker.

## Alternatives Considered

- In-place refactor: rejected because it preserves accidental coupling and makes the deletion denominator ambiguous.
- Many micro-crates: rejected because boilerplate and dependency churn can hide complexity.
- One replacement crate: deferred; allowed only if language/compiler/engine authority remains mechanically separated.

## Validation Plan

- Characterization: selected positive and negative examples, repeated canonical outcomes.
- Focused crates: fmt, check, clippy, tests, schema and fixture parity.
- Shadow: normalized plan, error, trace, artifact, resume, provider, and tool outcomes.
- Operational: binary size, dependency graph, warm/cold build, memory, latency, soak, rollback.
- Deletion: pinned file-list hash, deleted LoC, retained manifest, forbidden dependency/path checks.

## Exit Criteria

- All contracts and owner boundaries are reviewed.
- Parity and rollback gates pass at exact revisions.
- Default selection is reversible and then reviewed as stable.
- At least 80% of the pinned incumbent denominator is deleted.
