# Structured Task Prompt

Template: 1.0.0

Issue: 503

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only the V3-D local-preparation workflow: typed local command contracts, topology bind model, active-registry card rendering path, and doctor/PVF planning proof for requirements #171 through #173.

## Deliverables

- Local command contracts under csdlc-v3/src/commands/local/**
- Focused local-command workflow tests under csdlc-v3/tests/local_commands/**
- Active prompt-template registry/card-rendering integration for the V3-D local preparation path
- Doctor/PVF planning proof that a typed issue input reaches a validated PVF plan
- Truthful validation and review evidence for #503

## Acceptance

1. AC-1: Commands consume typed contracts.
2. AC-2: Bind enforces registered topology.
3. AC-3: Cards render from the active registry.
4. AC-4: Requirements #171 through #173 have CLI proof.

## Dependencies

- V3-C: #502 is closed by merged PR #572 and typed terminal closeout before #503 starts.
- V3-E: #504 must wait for #503 delivery.

## Inputs

- agent-logic/agent-design-language#503
- agent-logic/agent-design-language#502
- agent-logic/agent-design-language#171
- agent-logic/agent-design-language#172
- agent-logic/agent-design-language#173
- docs/csdlc-v3/CONTRACT.md
- docs/csdlc-v3/predecessor-coverage.json
- docs/csdlc-v3/proportional-lifecycle.json
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#V3-D
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md
- docs/milestones/v0.92.1/planned-issue-packets/issues/171/cards/stp.md
- docs/milestones/v0.92.1/planned-issue-packets/issues/172/cards/stp.md
- docs/milestones/v0.92.1/planned-issue-packets/issues/173/cards/stp.md
- docs/templates/prompts/current.json

## Non Goals

- PVF execution
- GitHub writes from csdlc-v3
- Publication, finish, or cleanup implementation
- Authority cutover
- v2 migration or retirement
- V3-E remote-delivery workflow
- V3-F authority-transition decision
- Repository-wide template redesign
