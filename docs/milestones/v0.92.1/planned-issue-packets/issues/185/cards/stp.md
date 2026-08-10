# Structured Task Prompt

Template: 1.0.0

Issue: 185

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only DRT-05 within its exact owned paths and authority boundary.

## Deliverables

- Producer-derived security and failure matrix with exact envelope and authority inputs.
- Independent verifier for every positive and negative outcome without hard-coded counts.

## Acceptance

1. Voting, agent, Shepherd, operator, and Observatory identities use separated keys and roles; Shepherd cannot vote.
2. Production TLS chains to an approved trust anchor and no self-signed certificate appears on a production path.
3. Forged, stale, wrong-domain, missing-capability, cross-polis, malformed, and pre-auth disclosure attempts are denied with typed receipts.
4. Provider timeout, stall, malformed output, and partial failure preserve state and authority invariants.

## Dependencies

- DRT-03: issue #183
- DRT-04: issue #184

## Inputs

- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-05
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Writing custom TLS primitives
- Treating transport encryption as authorization
- Generating outcome totals independently of producer results
