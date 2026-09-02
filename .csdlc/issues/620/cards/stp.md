# Structured Task Prompt

Template: 1.0.0

Issue: 620

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Audit and reconcile the existing v0.92.2 planning documents and produce a complete disposition of relevant TBD inputs.

## Deliverables

- Refreshed canonical v0.92.2 planning package
- Findings-first TBD scheduling reconciliation
- Aligned machine-readable issue wave and execution specifications
- Focused package validator
- Review handoff with deferrals and operator decisions

## Acceptance

1. AC-1: Every canonical planning document exists and agrees on purpose, scope, dependencies, sequencing, and planned posture
2. AC-2: Every first-class work track has a feature document, work-package home, proof expectation, and readiness disposition
3. AC-3: Every planned issue is one bounded unit of work with a concrete deliverable and acceptance denominator
4. AC-4: The catalog, issue wave, execution specifications, readiness table, and sprint plan have exact work-package parity
5. AC-5: The ten-step release tail is consistent across every planning surface
6. AC-6: Every relevant active TBD source has a source path, status, proposed home, rationale, and unresolved-decision field
7. AC-7: Unplanned or unscheduled TBD material is explicit while backlog and later-milestone work remains deferred
8. AC-8: Existing issues and completed work are reconciled without duplicate or reopened scope
9. AC-9: Links resolve, YAML parses, machine-local paths and unresolved placeholders are rejected, and diff hygiene passes
10. AC-10: Bounded independent review has no unresolved actionable finding before publication

## Dependencies

- v0.92.1 remains the active milestone
- Operator authorization is required before v0.92.2 milestone opening or WP-01 creation

## Inputs

- agent-logic/agent-design-language#620
- docs/milestones/v0.92.2/**
- .adl/docs/TBD/TBD_DOC_STATUS_INVENTORY.md
- .adl/docs/TBD/LOCAL_BACKLOG.md
- .adl/docs/TBD/planning/NEW_FEATURE_MILESTONE_ASSIGNMENT_PLAN.md

## Non Goals

- Open the v0.92.2 milestone or create its version label
- Create WP-01 or child issues
- Implement planned features
- Silently schedule deferred or unresolved work
- Run broad Rust or Runtime validation
