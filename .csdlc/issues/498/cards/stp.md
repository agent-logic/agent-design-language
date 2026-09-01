# Structured Task Prompt

Template: 1.0.0

Issue: 498

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare CORP-D and execute only after CORP-C #497 is terminal. Do not reopen CORP-A/CORP-B, perform CORP-C control transfer, or absorb unrelated sprint lanes.

## Deliverables

- Corporate diligence acceptance packet
- Blocker and residual-risk disposition record
- Private-evidence boundary notes
- Typed SRP/SOR truth for actual validation and residual risks

## Acceptance

1. CORP-A #482, CORP-B #483, and CORP-C #497 are verified closed, merged, and ancestral before CORP-D execution claims readiness.
2. The diligence acceptance packet records all blockers and residual risks with dispositions.
3. Repository artifacts exclude private legal advice, private diligence material, credentials, tokens, and account secrets.
4. Validation evidence proves repository-local packet structure and states any deferred or non-public evidence boundaries truthfully.

## Dependencies

- CORP-A #482 closed by PR #545 and merged into main
- CORP-B #483 closed by PR #562 and merged into main
- CORP-C #497 must close, merge, and become ancestral before CORP-D execution
- Sprint 4 umbrella #532 remains open for corporate acceptance orchestration

## Inputs

- AGENTS.md
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md
- docs/milestones/v0.92.1/WP_EXECUTION_READINESS_v0.92.1.md
- GitHub issue #498
- GitHub issue #532

## Non Goals

- Do not execute CORP-D before CORP-C is terminal.
- Do not perform CORP-C operational-control transfer in this issue.
- Do not commit private legal advice, private diligence material, credentials, tokens, or account secrets.
- Do not execute unrelated Sprint 4 or later-sprint lanes.
