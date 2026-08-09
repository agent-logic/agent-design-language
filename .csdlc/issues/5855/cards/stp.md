# Structured Task Prompt

Template: 1.0.0

Issue: 5855

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare and operate only the runtime-observatory-polis-protocol sprint umbrella; do not implement its child issues.

## Deliverables

- Issue-specific six-card sprint record
- Sprint Execution Packet for exact terminal membership #5800, #5820, #5821, #5795, and #5832
- Explicit exclusion record for independent follow-ons #5837, #83, and #84
- Integrated sprint review and truthful umbrella closeout candidate

## Acceptance

1. AC-1: The Sprint Execution Packet records exact terminal membership #5800, #5820, #5821, #5795, and #5832, while excluding #5837, #83, and #84.
2. AC-2: Safe parallel lanes, actual merge order, and serial gates are explicit and do not overlap child ownership.
3. AC-3: The umbrella coordinates only; each child retains implementation, proof, review, publication, and closeout authority.
4. AC-4: Every child handoff requires issue-bound bind, readiness, and session-goal truth before implementation.
5. AC-5: The umbrella is only a closeout candidate before publication and becomes terminal only after qualified PR merge, live issue closure, and csdlc-finish.

## Dependencies

- #5800
- #5820
- #5795
- #5821
- #5832
- #5837

## Inputs

- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/milestones/v0.92/SPRINT_v0.92.md
- .csdlc/prepared/issues/5855/sprint-execution-packet.yaml
- #5800
- #5820
- #5795
- #5821
- #5832
- #5837

## Non Goals

- Implementing child issue code
- Replacing child C-SDLC records
- Collapsing child review or publication into the umbrella
- Claiming parallel execution beyond the declared packet
