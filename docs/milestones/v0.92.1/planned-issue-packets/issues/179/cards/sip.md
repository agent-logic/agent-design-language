# Structured Intent Prompt

Template: 1.0.0

Issue: 179

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prove complete safety parity, execute bounded v3-only canaries, migrate authority without dual writes, and perform the separately approved selector cutover.

## Required Outcome

shadow corpus live canaries migration rehearsal authority scan rollback and independent review is produced at an exact revision and independently reproducible.

## Scope

- Representative v2 corpus, normalized parity runner, unsupported-field register, read-only shadow, opt-in v3 issue canaries, performance/effect measurement, migration tooling, operator runbook, rollback window, installation, one operator skill, selector switch, and post-cutover audit.

## Authority

- Issue V3-16 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
