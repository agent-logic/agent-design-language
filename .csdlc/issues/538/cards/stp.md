# Structured Task Prompt

Template: 1.0.0

Issue: 538

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Align and validate Sprint 10 coordination and child readiness only.

## Deliverables

- Canonical Sprint 10 planning update
- Versioned sequential Sprint Execution Packet and sprint state
- Mechanical structured-prompt readiness report for #516 through #526
- Dependency-truth and first-child execution handoff

## Acceptance

1. AC-1: #538 membership is exactly #516 through #526 with one bounded result per child
2. AC-2: Canonical sprint planning and live membership version 7 agree
3. AC-3: The execution packet records the strict #516 through #526 sequential chain, watchers, issue-goal handoffs, and closeout bar
4. AC-4: Every child has issue-specific typed design-time prompt surfaces or an explicit fail-closed repair route
5. AC-5: #516 is handed off only when all declared admission prerequisites are closed; open prerequisites remain visible blockers
6. AC-6: No later child is executable before its immediate predecessor has a reviewed green merge on main
7. AC-7: Focused readiness, prompt, path, and diff validation pass

## Dependencies

- Live sprint umbrella #538
- Live child issues #516 through #526
- #516 admission prerequisites, including #505 and #512

## Inputs

- docs/milestones/v0.92.1/SPRINT_v0.92.1.md
- docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml
- docs/templates/sprints/current.json
- docs/templates/sprints/1.0.0/sprint_execution_packet.md
- adl/tools/skills/sprint-conductor/scripts/check_sprint_readiness.py

## Non Goals

- Implementing or merging child work
- Collapsing child results into the umbrella
- Weakening dependency or review gates
- Executing release qualification or release ceremony
