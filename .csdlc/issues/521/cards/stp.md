# Structured Task Prompt

Template: 1.0.0

Issue: 521

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly one independent external-review report; findings are rows within that report.

## Deliverables

- Independent reviewer identity and conflict declaration
- Exact candidate and #520 packet manifest
- Findings-first external report
- Limitations and coverage statement
- Validated report manifest

## Acceptance

1. AC-1: Reviewer independence and conflicts are recorded
2. AC-2: Scope and limitations bind the unchanged exact candidate and #520 packet
3. AC-3: Every finding has severity, exact evidence, impact, and disposition route
4. AC-4: Missing or inaccessible evidence is a limitation or finding, never an implied pass

## Dependencies

- TAIL-04/#520 reviewed green merge is live and candidate identity is unchanged

## Inputs

- agent-logic/agent-design-language#521
- agent-logic/agent-design-language#520
- docs/milestones/v0.92.1/evidence/release/tail-04/**
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#TAIL-05

## Non Goals

- Finding remediation
- Release approval
- Candidate mutation
- Merge or deployment
