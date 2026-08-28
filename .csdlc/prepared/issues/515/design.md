# Issue 515 Design — Local-model shadow execution

## Goal

Produce one bounded local-model shadow-execution and comparison path that cannot acquire authority.

## Required Outcome

Shadow execution is distinguishable, deterministic, redacted, and unable to mutate or replace the authoritative provider result.

## Ownership

- `adl/src/provider`
- `docs/milestones/v0.92.1/evidence/provider/prov-b`
- `.csdlc/prepared/issues/515/validate-provider-shadow.rb`

## Dependencies

- Terminal reviewed and ancestral PROV-A issue #514
- Sprint 9 umbrella #537

## Safety Boundary

- This issue owns only the listed result and paths.
- Missing, stale, skipped, non-proving, or ambiguous evidence fails closed.
- Validation and independent exact-head review precede publication.

## Non-Goals

- Provider benchmark marketing claims
- Production provider cutover
- Changing provider authority
