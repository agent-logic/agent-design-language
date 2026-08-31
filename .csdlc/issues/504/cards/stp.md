# Structured Task Prompt

Template: 1.0.0

Issue: 504

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #504 V3-E only; prepare and implement the construction-only remote delivery workflow after #503 is terminal and ancestral. Do not cut over authority, merge/finish/clean through v3, or absorb V3-F/#505.

## Deliverables

- Initialized #504 C-SDLC card bundle for V3-E remote-delivery workflow preparation.
- .csdlc/prepared/issues/504/validate-remote-workflow.rb
- Design and diagram packet under .csdlc/prepared/issues/504 preserving the #503 terminal dependency and V3-F/#505 authority boundary.
- Post-#503 handoff plan naming the future csdlc-v3 remote command, review, publication, finish, cleanup, and remote-command test surfaces.

## Acceptance

1. AC-1: Review binds exact immutable scope
2. AC-2: Publication modes are explicit
3. AC-3: Finish derives terminal truth
4. AC-4: Requirements #174 through #178 have positive and refusal proof

## Dependencies

- V3-D #503 terminal, reconciled, and ancestral before implementation

## Inputs

- agent-logic/agent-design-language#504
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#V3-E
- docs/milestones/v0.92.1/features/CSDLC_V3_v0.92.1.md
- docs/csdlc-v3/predecessor-coverage.json
- csdlc-v3
- PR #581 / issue #503 terminal output when available

## Non Goals

- Authority cutover
- Broad repository cleanup
- Replacing C-SDLC v2 as live authority
- Remote provider execution
- Merging, finishing, or cleaning through v3
