# Structured Task Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare only the V3-F authority-transition decision: requirements #179 and #180 mapping, v2-v3 parity, migration canary, rollback, observation window, and explicit operator disposition.

## Deliverables

- Initialized #505 C-SDLC card bundle for V3-F authority-transition decision preparation.
- .csdlc/prepared/issues/505/validate-authority-transition-prep.rb
- Design and diagram packet under .csdlc/prepared/issues/505 preserving the #504 terminal dependency, v2-live authority boundary, no-silent-retirement rule, and operator approval gate.
- Post-#504 handoff plan naming future parity, migration-canary, rollback, observation-window, and approval evidence surfaces.

## Acceptance

1. AC-1: Requirements #179 and #180 are mapped.
2. AC-2: v2-v3 parity is measured.
3. AC-3: Canary rollback is exercised.
4. AC-4: Cutover and retirement require operator approval.

## Dependencies

- V3-E: #504 must be closed by merged PR and typed terminal closeout before #505 execution starts.

## Inputs

- agent-logic/agent-design-language#505
- agent-logic/agent-design-language#504
- agent-logic/agent-design-language#179
- agent-logic/agent-design-language#180
- docs/csdlc-v3/CONTRACT.md
- docs/csdlc-v3/predecessor-coverage.json
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#V3-F
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md

## Non Goals

- Silent v2 retirement
- Unsupported-platform claims
- Authority cutover without operator approval
- Executing publication, finish, or cleanup through v3 before approval
- Broad repository cleanup
