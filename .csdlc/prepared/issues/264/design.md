# Issue 264 Design — Operator-authorized podcast directory submissions

## Goal

Execute only separately authorized provider submissions and retain truthful redacted IDs, status, correction, monitoring, and rollback evidence.

## Required Outcome

Each authorized submission has an exact provider identity and truthful status while unauthorized providers remain untouched.

## Ownership

- `docs/milestones/v0.92.1/evidence/podcast/51-d`
- `docs/podcast/submission-ledger`

## Dependencies

- Terminal #263
- Explicit future provider-specific operator authorization
- Sprint 8 umbrella #536

## Safety Boundary

- This issue owns only the listed result and paths.
- All external mutations and private material remain governed by the operator constraints.
- Validation and exact-head review precede publication.

## Non-Goals

- Automatic submission
- Action before explicit authorization
- Credential retention
- Hosting redesign
- Advertising or monetization
