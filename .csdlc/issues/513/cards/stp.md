# Structured Task Prompt

Template: 1.0.0

Issue: 513

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Exactly one accepted Runtime v2/v3 authority topology with executable migration and rollback checks; no Runtime v4 work and no sibling Sprint 1 scope.

## Deliverables

- Runtime v2/v3 authority topology document.
- Machine-readable ownership and disposition manifest.
- Executable validator covering source denominator, reverse-reference census, compatibility, rollback, migration, and Runtime v4 exclusion.
- Fresh independent exact-head review and green PR closing #513.

## Acceptance

1. AC-1: Every declared Runtime v2 and Runtime v3 source root exists and has one owner and disposition.
2. AC-2: Current reverse references to Runtime v2 and Runtime v3 are inventoried with one owner/disposition.
3. AC-3: Supported behavior has an executable compatibility proof.
4. AC-4: Rollback and migration are executable dry-run contracts.
5. AC-5: Runtime v4 remains excluded from source ownership, migration, and rollback authority.

## Dependencies

- WP-01 #480
- v0.92.1 planning digest f00977324d7bfbfcb17a04d1798d14eca9c99c6d6299a0ae21977f564b518251

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/513
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#DEC-01
- docs/milestones/v0.92.1/features/RUNTIME_V2_V3_DECOUPLING_v0.92.1.md
- adl/src/runtime_v2
- adl-runtime
- adl-runtime-kernel

## Non Goals

- Runtime v4 implementation
- Runtime v2 deletion
- Runtime v3 default cutover
- Sibling Sprint 1 work
- Behavior deletion without proof
