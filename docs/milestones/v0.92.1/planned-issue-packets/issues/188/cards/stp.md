# Structured Task Prompt

Template: 1.0.0

Issue: 188

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only INT-01 within its exact owned paths and authority boundary.

## Deliverables

- Findings-first integrated review at exact lane revisions.
- Disposition and remediation ledger with a bounded release recommendation.

## Acceptance

1. CORP-08, V3-16, and DRT-07 are terminal and exact revisions are ancestral to the review revision.
2. Every required lane artifact and quality gate is independently recomputed or explicitly rejected.
3. All P1/P2 findings receive verified terminal dispositions before recommendation.
4. The review does not treat one lane's success as evidence for another lane.

## Dependencies

- CORP-08: issue #160
- V3-16: issue #179
- DRT-07: issue #187

## Inputs

- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#int-01
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Implementing undisclosed remediation
- Waiving blockers
- Publishing a release
