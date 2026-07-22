# Structured Task Prompt

Template: 1.0.0

Issue: 5355

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare lifecycle packet only; closeout-plan authoring is future execution work.

## Deliverables

- generated six-card preparation packet
- concise design and diagram
- focused typed doctor validation

## Acceptance

1. AC-1: preparation packet is generated through typed C-SDLC v2
2. AC-2: future execution is blocked on #5362 live merge and ancestry
3. AC-3: receipts are recorded as audit evidence only
4. AC-4: no implementation, PR, AWS, raw gh, or root-main tracked write occurs

## Dependencies

- WP-21 #5362 live merged into origin/main
- #5362 observed merge SHA ancestral to exact #5355 execution base

## Inputs

- docs/milestones/v0.91.8/CANONICAL_DOC_INVENTORY_v0.91.8.md
- docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- issue #5355

## Non Goals

- closeout-plan authoring during preparation
- birthday implementation
- PR publication
- receipt-gated execution
