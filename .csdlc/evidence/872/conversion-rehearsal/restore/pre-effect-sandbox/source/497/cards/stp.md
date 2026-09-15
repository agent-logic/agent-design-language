# Structured Task Prompt

Template: 1.0.0

Issue: 497

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare and execute CORP-C acceptance only. Do not absorb CORP-D diligence acceptance, Sprint 7 #345 AWS GPU execution, Sprint 8 #84 Unity work, or any adjacent corporate migration lane.

## Deliverables

- Corporate operational-control transfer acceptance packet
- Rollback/break-glass availability notes for any proposed external mutation
- Provider/account authority evidence that excludes secrets
- Typed SRP/SOR truth for actual validation and residual risks

## Acceptance

1. Each control plane has corporate owner and rollback.
2. AWS uses the approved business account.
3. Terraform and CI authority are company-controlled.
4. Availability and recovery readbacks pass.

## Dependencies

- CORP-A #482 closed by PR #545 and merged into main
- CORP-B #483 closed by PR #562 and merged into main
- AWS-G #496 closed by PR #599 and merged into main
- GCP-D #493 closed by PR #587 and merged into main
- Sprint 4 umbrella #532 remains open for corporate acceptance orchestration

## Inputs

- AGENTS.md
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md
- docs/milestones/v0.92.1/WP_EXECUTION_READINESS_v0.92.1.md
- GitHub issue #497
- GitHub issue #532

## Non Goals

- Do not execute Sprint 7 #345 AWS GPU work.
- Do not execute Sprint 8 #84 Unity work.
- Do not complete CORP-D diligence acceptance in this issue.
- Do not mutate external providers, billing, credentials, or legal/private diligence records without explicit operator authorization.
