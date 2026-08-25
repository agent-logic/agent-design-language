# WP-27 Review Findings Remediation

## Status

- Canonical issue: `#315`
- Legacy C-SDLC evidence id: `5848`
- Parent intake issue: `#314` / WP-26
- Known child route: `#471` remains under WP-27 for Runtime v3 follow-on routing.
- Independent sibling: `#316` / WP-28 is not a remediation owner for this packet.

This packet accounts for every finding row from the retained internal and
external review sources:

- `docs/reviews/v0.92/internal-review-5846/findings.json`
- `docs/reviews/v0.92/external-review-5847/findings-index.json`

The external code-review PDF was received twice and retained twice. Both source
occurrences are preserved in the disposition register, while the duplicate code
findings share one implementation fix and one focused validation proof.

## Result

- Source finding rows accounted for: 30
- Unique external findings: 7
- Current #315 implementation fixes: 3 unique birthday-activation findings
- Duplicate external source rows fixed by those same code changes: 6
- Internal review rows resolved by existing merged authority: 20
- Open v0.92 release blockers introduced here: 0

## Implementation Fixes

The current #315 remediation branch fixes the production birthday activation
findings in `adl-runtime-kernel/src/production_birthday.rs`:

1. post-commit cleanup failure is now represented as
   `CommittedWithCleanupPending(ProductionBirthdayReceipt)`, so callers can
   distinguish a committed activation with cleanup residue from an uncommitted
   activation failure;
2. `receipt_from_input` now propagates canonical digest errors instead of
   defaulting `input_sha256` to an empty string; and
3. `ProductionBirthdayStore` documents its same-host local-filesystem and
   advisory-lock assumptions.

Focused proof is retained at:

- `.csdlc/evidence/5848/production-birthday-focused-test.json`

## Existing Authority Used

The internal review rows are not reopened here. They are resolved for v0.92
release-gate purposes by the merged #467 / PR #468 corrective hydration
authority, which records the superseding quality-gate result and leaves #311 as
historical provenance only.

The external documentation-review rows are handled as claim-boundary and send
gate corrections: the received report remains a blocked/non-approving intake
artifact, while the future exact-revision documentation review route is the
WP-23/#312 handoff.

## Non-claims

- This packet does not approve release, external publication, deployment, or
  production citizenship.
- This packet does not claim network-filesystem safety for the production
  birthday store.
- This packet does not make #316 a remediation owner.
- This packet does not claim #471 is complete; it only preserves the operator's
  direction that #471 is a WP-27 child route.
