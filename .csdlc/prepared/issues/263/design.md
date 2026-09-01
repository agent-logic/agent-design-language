# Issue 263 Design — Podcast directory submission runbooks and operator preflight

## Goal

Prepare current provider-specific directory submission runbooks and one redacted operator preflight without mutating provider accounts.

## Required Outcome

Apple, Spotify, Amazon, and YouTube runbooks identify every account-side and irreversible step, consume the exact production feed, and hand a safe ledger schema to #264.

## Ownership

- `docs/milestones/v0.92.1/evidence/podcast/51-c`
- `docs/podcast/directory-runbooks`

## Dependencies

- Terminal #261
- Terminal #262
- Sprint 8 umbrella #536

## Safety Boundary

- This issue owns only the listed result and paths.
- All external mutations and private material remain governed by the operator constraints.
- Validation and exact-head review precede publication.

## Non-Goals

- Provider submission
- Provider account creation or mutation
- Hosting implementation
- Public launch
