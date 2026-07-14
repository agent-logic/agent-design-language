# CLI And Adapters

## Metadata
- Feature Name: Thin ADL CLI and narrow runtime/provider/tool adapters
- Milestone Target: `v0.91.8`
- Status: planned
- Owner: WP-08 through WP-10
- Doc Role: primary
- Supporting Docs: `../DESIGN_v0.91.8.md`
- Feature Types: runtime, architecture, policy
- Proof Modes: demo, tests, review

## Template Rules

Default CLI builds must exclude unrelated integration dependency graphs.

## Purpose

Expose the small product and connect it to external owners without rebuilding
the monolith.

## Context

- Related milestone: `v0.91.8`
- Related issues: WP-08/WP-09/WP-10 pending
- Dependencies: WP-04 through WP-07

## Coverage / Ownership

- Primary owner doc: this document.
- Covered surfaces: owner commands, Runtime v3 adapter, provider/tool adapters,
  generation selector, binary/dependency budgets.
- Related docs: cutover feature.

## Overview

The CLI owns validate, schema, plan, run, inspect, sign, and verify. Runtime
administration and C-SDLC commands remain in their owner binaries.

## Design

### Core Concepts

- Thin owner command.
- Capability-scoped adapter.

### Architecture

- Inputs: typed CLI arguments, files, explicit adapter configuration.
- Outputs: stable JSON stdout, diagnostics/events stderr.
- Interfaces: engine API, Runtime v3, provider/tool ports, selector.
- Invariants: no shell evaluation; secrets never emitted; default graph stays small.

### Data / Artifacts

- selector, install receipt, adapter capability manifest.

## Execution Flow

1. Parse direct Clap arguments.
2. Select generation and adapter explicitly.
3. Invoke one typed operation and emit classified result.

## Determinism and Constraints

- Local commands are deterministic for identical files and selector.
- Network adapters declare nondeterminism and capture responses.

## Integration Points

| System / Surface | Integration Type | Description |
|---|---|---|
| Runtime v3 | trigger | Admit/supervise portable execution. |
| Providers/tools | trigger | Perform explicit external capability calls. |

## Validation

- CLI snapshots, JSON parsing, exit codes, dependency and binary-size checks.
- Secret redaction and authority-denial negatives.

## Acceptance Criteria

- Required commands pass parity.
- Default binary and installed set meet budgets.

## Risks

- Compatibility commands may become permanent; every one needs an expiry.

## Future Work

New adapters remain separate packages with their own proof and release cadence.

## Notes

An adapter may depend on ADL contracts; ADL core must not depend on the adapter.
