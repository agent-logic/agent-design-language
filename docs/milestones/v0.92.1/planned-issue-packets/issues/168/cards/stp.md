# Structured Task Prompt

Template: 1.0.0

Issue: 168

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-07 within its exact owned paths and authority boundary.

## Deliverables

- Pure transition API, transition-and-correction table generated from the V3-01 capability matrix, recovery graph, invariant/property tests, negative transition corpus, idempotency model, and v2 behavior mapping.

## Acceptance

1. Every state/command pair has an explicit allowed or rejected outcome.
2. The compiler enforces exhaustive closed-state handling.
3. Branch/worktree topology is the only local ownership authority.
4. Review staleness, publication gates, terminal truth, and cleanup eligibility remain fail-closed.
5. Every accepted recovery transition preserves a reachable typed correction or typed terminal-disposition command; no supported state is a lifecycle dead end and no abstract operator-required sink satisfies reachability.
6. The generated transition table accepts `review recover` only from `reviewed`, `published`, or `merge_ready`, returns to `implemented`, rejects `merged` and `closed_out`, and proves the matrix-declared atomic invalidations.
7. Removing or changing any authorization predicate causes mutation/property tests to fail, including correction invalidation and stale-CAS predicates.
8. Cleanup eligibility requires committed `closed_out` state and a retained terminal receipt; remote merge observation alone is insufficient.

## Dependencies

- V3-06: issue #167

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-07
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- File writes, Git commands, GitHub calls, clock reads, prompting, process execution, or retry policy.
