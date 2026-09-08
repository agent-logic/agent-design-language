# Structured Task Prompt

Template: 1.0.0

Issue: 517

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly one quality-gate decision; individual proving lanes are evidence inputs.

## Deliverables

- docs/milestones/v0.92.1/QUALITY_GATE_v0.92.1.md
- docs/milestones/v0.92.1/evidence/release/tail-01
- .csdlc/prepared/issues/517/validate-quality-gate.rb

## Acceptance

1. AC-1: Every required proving lane passes
2. AC-2: Skipped, absent, zero-test, stale, and non-proving results fail closed
3. AC-3: The exact candidate revision and complete denominator are recorded
4. AC-4: Every exception has an explicit owner and no unresolved exception remains

## Dependencies

- INT-01/#516 reviewed merge before execution

## Inputs

- agent-logic/agent-design-language#517
- agent-logic/agent-design-language#516
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#TAIL-01
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md

## Non Goals

- Documentation repair
- Product implementation
- Publication
- Release ceremony
