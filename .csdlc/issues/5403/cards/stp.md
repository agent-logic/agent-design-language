# Structured Task Prompt

Template: 1.0.0

Issue: 5403

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Review the ten declared closed sprint umbrellas and publish review evidence; route implementation fixes separately.

## Deliverables

- Ten separate findings-first sprint review packets
- Cross-sprint findings and remediation synthesis
- Updated v0.91.7 sprint review register
- Independent quality review of the completed packet set

## Acceptance

1. Every in-scope sprint has a separate retained review packet
2. Every ordered child is covered or explicitly excluded with rationale
3. Findings record severity evidence impact and disposition
4. Actionable fixes are routed to separate remediation issues
5. The canonical register agrees with live GitHub and retained evidence
6. Independent review finds no unresolved actionable defect in the review package

## Dependencies

- Live GitHub issue and pull request metadata
- Existing v0.91.7 review and closeout artifacts
- Sprint-review specialist workflow

## Inputs

- docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md
- docs/milestones/v0.91.7/review/V0917_CLOSED_SPRINT_REVIEW_4649.md
- .adl/v0.91.7/sprints
- docs/architecture/runtime_v3_closeout_truth_5385.v1.json

## Non Goals

- Implementing review findings
- Reviewing open v0.91.7 work packages
- Reviewing the active v0.91.8 implementation wave
- Changing product or runtime behavior
