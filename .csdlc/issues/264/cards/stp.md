# Structured Task Prompt

Template: 1.0.0

Issue: 264

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Deliver only issue #264: Operator-authorized podcast directory submissions.

## Deliverables

- Each authorized submission has an exact provider identity and truthful status while unauthorized providers remain untouched.
- Issue-specific retained validation evidence
- Exact-head review and truthful terminal record

## Acceptance

1. AC-1: No execution occurs before terminal #263 and explicit authorization naming each provider.
2. AC-2: Every submission uses the exact reviewed feed, identity, artwork, rights declaration, and company account.
3. AC-3: Canonical IDs, URLs, and status are retained without credentials, verification codes, or unsupported acceptance claims.
4. AC-4: Destination links activate only after live verification; corrections and rollback preserve history.
5. AC-5: Exact-head reviews pass before external action and after final reconciliation.

## Dependencies

- Terminal #263
- Explicit future provider-specific operator authorization
- Sprint 8 umbrella #536

## Inputs

- docs/milestones/v0.92.1/evidence/podcast/51-d
- docs/podcast/submission-ledger
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md
- .csdlc/prepared/issues/536/sprint-execution-packet.yaml

## Non Goals

- Automatic submission
- Action before explicit authorization
- Credential retention
- Hosting redesign
- Advertising or monetization
