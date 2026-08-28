# Structured Task Prompt

Template: 1.0.0

Issue: 500

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Define and review only the V3-A contract and construction-decision packet, including exact retained-requirement mapping and rollback posture.

## Deliverables

- docs/csdlc-v3/CONTRACT.md with explicit v2 sole-authority, compatibility, construction, and rollback rules
- docs/csdlc-v3/predecessor-coverage.json with exactly one nonempty disposition for each of #161, #162, and #163
- docs/csdlc-v3/proportional-lifecycle.json with the complete lifecycle-surface denominator, exactly one retained/collapsed/derived/removed disposition per surface, named hazards for retained gates, and fixed default-path cardinality
- csdlc-v3/Cargo.toml
- csdlc-v3/src/lib.rs minimal non-authoritative contract boundary with named contract_schema, predecessor_coverage, architecture_boundary, and proportional_lifecycle tests
- .csdlc/evidence/500/** focused validation and exact-head review evidence

## Acceptance

1. AC-1: The v3 authority boundary and compatibility posture are explicit.
2. AC-2: Requirements 161 through 163 are mapped exactly.
3. AC-3: Construction and rollback decisions are reviewable, and V3 removes, collapses, or derives checkpoints, projections, reviews, and transitions that do not mitigate a named risk; its default path uses one meaningful design gate, focused validation, one independent implementation review, and truthful closeout, making a routine three-issue sprint mechanically ready in minutes rather than hours.

## Dependencies

- None; #161, #162, and #163 are retained predecessor inputs, not open execution blockers.

## Inputs

- agent-logic/agent-design-language#500
- agent-logic/agent-design-language#161
- agent-logic/agent-design-language#162
- agent-logic/agent-design-language#163
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#V3-A
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md

## Non Goals

- Authority cutover
- C-SDLC v2 retirement
- Implementation of V3-B or later issues
- Repository-wide lifecycle refactoring
