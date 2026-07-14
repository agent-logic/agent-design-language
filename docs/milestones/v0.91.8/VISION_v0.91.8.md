# v0.91.8 Vision

## Metadata
- Project: Agent Design Language
- Milestone: `v0.91.8`
- Version: `v0.91.8`
- Date: `2026-07-14`
- Owner: ADL maintainers
- Related issues: `#5335` and the v0.91.8 issue wave

## Purpose

Define the strategic outcome of making ADL itself as small, typed, and
reviewable as the successfully rearchitected runtime and control plane.

## How To Use

Use this document for direction and success boundaries. Implementation details
belong in the design, feature docs, and issue-local plans.

## Overview

ADL has accumulated language, runtime, control-plane, integration, demo, and
proof responsibilities in one approximately 356k-line Rust surface. v0.91.8
makes the language product legible again: documents compile deterministically
into plans, a small engine executes those plans through explicit ports, and
other products retain their own authority.

## Core Goals

1. Recover a small explicit language boundary.
2. Preserve behavior through characterization rather than source migration.
3. Make dependency and authority boundaries enforceable.
4. Prove reversible replacement before deletion.
5. Reduce the incumbent source surface by 80-90%.

## Language Clarity

The six primitives—provider, tool, agent, task, workflow, and run—remain the
language foundation. Extensions must compile through explicit contracts rather
than widening every layer of the product.

## Deterministic Compilation

Resolution, composition, pattern expansion, stable node identity, validation,
and plan serialization form a pure compiler boundary with no network, runtime,
wall-clock, or cloud authority.

## Portable Execution

The engine owns only portable execution semantics: bounded concurrency,
ordering, retry/failure, joins, resume state, results, and trace/artifact
contracts. Runtime v3 owns service supervision and long-lived operation.

## Product Independence

ADL core must build and validate without C-SDLC, AWS, Google Workspace,
GitHub, Observatory, or cognitive-runtime dependency graphs.

## Milestone Context

Runtime v3 proved that a component kernel can replace an accreted runtime.
C-SDLC v2 proved that a typed state engine and owner binaries can replace an
accreted control plane. v0.91.8 applies that method to the remaining ADL core.

## Long-Term Direction

Future language features should be cheap to understand, compile, validate, and
review. Product integrations should evolve independently through versioned
ports instead of growing the default ADL crate.

## Summary

Success is not a rearranged monolith. Success is a small default product,
measured parity, deleted legacy ownership, and a clear boundary for v0.92.

## Exit Criteria

- Direction, owner boundaries, budgets, and non-goals remain intact through execution.
- Any strategic change is recorded in the decision log and architecture review.
