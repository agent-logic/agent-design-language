# Language And Compiler

## Metadata
- Feature Name: Clean-room ADL language and deterministic compiler
- Milestone Target: `v0.91.8`
- Status: planned
- Owner: WP-04 and WP-05
- Doc Role: primary
- Supporting Docs: `../DESIGN_v0.91.8.md`
- Feature Types: architecture, runtime, artifact
- Proof Modes: tests, schema, replay, review

## Template Rules

Final crate/module choice is deferred to WP-02 architecture approval.

## Purpose

Define ADL documents precisely and compile them into stable executable plans.

## Context

- Related milestone: `v0.91.8`
- Related issues: WP-04/WP-05 pending
- Dependencies: WP-03 corpus contract

## Coverage / Ownership

- Primary owner doc: this document.
- Covered surfaces: six primitives, parsing, schema, validation, resolution,
  composition, patterns, node identity, `ExecutionPlan`.
- Related docs: engine/contracts feature.

## Overview

The language layer is typed and side-effect free. The compiler resolves and
expands a valid document into a canonical plan without runtime authority.

## Design

### Core Concepts

- Six primitive document model.
- Canonical versioned execution plan.

### Architecture

- Inputs: ADL YAML/JSON and explicit include sources.
- Outputs: validated document, diagnostics, schema, `ExecutionPlan`.
- Interfaces: Rust API and CLI serialization contracts.
- Invariants: deterministic output; no network, clock, provider, or mutation.

### Data / Artifacts

- language schema and plan schema with stable diagnostic codes.

## Execution Flow

1. Parse and structurally validate.
2. Resolve references and semantic invariants.
3. Expand patterns and emit a canonical DAG.

## Determinism and Constraints

- Stable node IDs and lexicographic ready ordering.
- Unknown fields and unresolved references fail closed.

## Integration Points

| System / Surface | Integration Type | Description |
|---|---|---|
| Engine | write | Supplies immutable execution plan. |
| CLI | read | Exposes validate, schema, and plan commands. |

## Validation

- Schema fixtures, composition cases, cycle/unknown-reference negatives.
- Repeated canonical plan equality.

## Acceptance Criteria

- Characterization parity passes for language and plan cases.
- Combined implementation remains within the approved budget.

## Risks

- Compatibility aliases can widen the core; isolate them in the importer.

## Future Work

New language features require versioned schema and compiler contracts.

## Notes

Runtime-specific behavior is not language law unless explicitly contracted.
