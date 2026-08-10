# Structured Task Prompt

Template: 1.0.0

Issue: 153

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only CORP-01 within its exact owned paths and authority boundary.

## Deliverables

- Redacted critical-asset register with current and target owner, custodian, recovery authority, transfer method, dependency, and disposition.
- Machine-checkable denominator and exclusion matrix with stable asset identifiers.

## Acceptance

1. Every asset class named by the promoted corporate source has at least one inventoried row or an explicit not-applicable disposition.
2. Each critical row identifies current control, target corporate control, transfer dependency, verification method, rollback posture, and evidence location.
3. The validator rejects duplicate identifiers, missing owners, missing recovery authority, unbounded secret fields, and unapproved exclusions.
4. No transfer or credential rotation occurs in this inventory issue.

## Dependencies

- No child dependency; setup issue #146 and umbrella readiness only

## Inputs

- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#corp-01
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Executing transfers
- Storing secrets or private legal instruments
- Inferring ownership from billing screenshots alone
