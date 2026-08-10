# Structured Task Prompt

Template: 1.0.0

Issue: 156

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only CORP-04 within its exact owned paths and authority boundary.

## Deliverables

- Redacted service custody matrix and recovery test record.
- Company-controlled administrative and billing readback with named role, not credential, ownership.

## Acceptance

1. Every critical service has a company-controlled administrator, billing owner, secure MFA, recovery route, and vault location.
2. Recovery is exercised without relying solely on a founder-owned phone, email, card, or device.
3. Break-glass access is bounded, audited, and distinct from routine credentials.
4. The repository records names and outcomes only; no credential material is retained.

## Dependencies

- CORP-01: issue #153

## Inputs

- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#corp-04
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Committing credentials
- Weakening MFA to simplify automation
- Treating personal billing linkage as corporate custody
