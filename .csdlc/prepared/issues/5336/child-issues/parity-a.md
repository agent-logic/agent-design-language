## Goal

Make the canonical Runtime v3 kernel accept and execute real domain work
instead of only fixed bootstrap fixtures.

## Owned Capability Groups

- kernel lifecycle;
- topology and backpressure;
- service contracts and configuration;
- continuity, replay, and recovery;
- canonical secure typed domain ingress.

## Required Outcome

A guardian-launched configured kernel accepts representative work through one
canonical ingress, executes production components through typed bounded
channels, emits retained evidence, checkpoints and resumes deterministically,
and shuts down gracefully under normal and resource-pressure paths.

## Deliverables

- Canonical ingress contract and negative authority tests.
- Live initialized-process proof for all owned groups.
- Consolidation of overlapping `adl-runtime` core implementations.
- Exact-revision LoC/test budget report.

## Parent And Dependencies

- Parent acceptance umbrella: #5361.
- Architecture and budgets: #5336.
- Milestone: v0.91.8.

## Definition Of Done

- Production code is exercised through `adl-runtime-kernel` and canonical
  domain ingress; fixture/library/degraded evidence is insufficient.
- Deterministic positive and negative evidence is retained at an exact
  revision, including graceful shutdown/recovery.
- Duplicate or placeholder code is deleted to preserve the #5336 budget.
- Maintained third-party crates are used where practical.
- No AWS use, hard-coded IPs, HTTP-only access, default switch, Runtime v2
  deletion, or new product scope.
