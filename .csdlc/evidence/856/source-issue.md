## Summary

The v0.92.1 ceremony cannot pass its existing preflight: `adl/Cargo.toml` is 0.92.0, and the missing milestone ceremony gate sends the wrapper into retired v2 authority. Discovered while preparing #526 at aef581ecd6.

## Goal

Make non-mutating v0.92.1 ceremony preflight executable under current native v3 authority, with explicit release-artifact version reconciliation.

## Required Outcome

A reviewed repair that allows an otherwise eligible exact candidate to pass preflight and rejects an ineligible candidate without tag or release mutations.

## Deliverables

- Explicit release-artifact/version inventory, with coordinated manifest and lockfile changes where required; independently versioned helpers justified separately.
- Current v3 ceremony preflight replacing the automatic v2 fallback.
- Focused regression evidence and operator command documentation.

## Acceptance Criteria

- `v0.92.1` agrees with versions of the declared release artifacts and their lockfile identities.
- A missing or stale v3 gate fails closed without invoking v2 or bypass flags.
- Preflight is bound to the exact proposed commit and notes; changed identities invalidate eligibility.
- Non-mutating positive and negative fixtures prove version mismatch, missing/stale gate and eligible candidate behavior; no actual release publication is needed for tests.
- Operational owner binaries use stable installed locations, not a disposable worktree target directory.
- PVF classification and stdout/stderr behavior are documented alongside focused proof.
- #526 receives the repair revision, exact command, validation results and any remaining prerequisite gates.

## Repo Inputs

- `adl/tools/release_ceremony.sh`: `check_cargo_version`, `check_typed_closeout_gate`, `run_csdlc_doctor`, release owner lookup.
- `adl/Cargo.toml` and associated workspace manifests/lockfiles.
- `docs/csdlc-v3/CURRENT_AUTHORITY.md`.
- `docs/milestones/v0.92.1/evidence/release/tail-02/CARGO_MANIFEST_REVIEW.md`: records 0.92.0 and assigns version reconciliation to #519.
- `docs/milestones/v0.92.1/evidence/release/tail-03/README.md`: records no package version changes.

## Dependencies

Required before #526 final candidate approval. Coordinate with #522 for remediation accounting. Do not modify #833, its worktree, or its review evidence.

## Demo Expectations

An offline, non-mutating preflight demonstration with explicit passing and failing candidates.

## Non-goals

Tagging, publishing a release, merging, bypassing release gates, rewriting historical review evidence, or repairing unrelated Runtime proof.

## Issue-Graph Notes

Separate tooling repair discovered by #526. This owns the lost version-reconciliation handoff from #518/#519; closure of those issues did not establish version parity.

## Notes

Safe reproduction is source-only: compare manifest 0.92.0 with the wrapper's required 0.92.1, confirm the milestone gate is absent, and follow the fallback into the v2 doctor. Do not execute retired commands to reproduce it.

## Tooling Notes

Use native v3 issue binding, review and publication in a dedicated FastWork worktree. Keep primary main clean.


<!-- csdlc-v3-operation:eefe00516cf2b95e58392b775dadf4d950255b9e50bfbab34b6ccc6ddd2b684a -->
