# Issue 27 Design: Native Receipt Validator Repair

## Problem

The original Runtime v3 native receipt validator canonicalized observed artifact
roles but compared them with a declaration whose order was not canonical. The
WP-03 branch contains a candidate sorted comparison, but it is not independently
proven and its Git revision exception remains vulnerable to path and ancestry
edge cases. A verifier repair after a completed native proof needs a narrow
Git-backed exception that cannot admit runtime or product changes.

## Design

1. Treat the required artifact roles as a set with an explicit uniqueness
   invariant. Compare canonical sorted copies of the expected and observed role
   lists.
2. Keep duplicate-role rejection separate from denominator equality so a
   duplicated role can never be hidden by canonicalization.
3. Centralize the post-proof path policy in one predicate. Permit only the
   validator, its focused test, and issue-local lifecycle finalization
   paths.
4. Resolve the proof revision and current verifier revision through Git. Require
   proof ancestry, enumerate every changed path with rename detection disabled,
   require a clean worktree, and reject the packet if any path falls outside the
   explicit allowlist.
5. Preserve all existing digest recomputation, platform coverage, provenance,
   and receipt-content validation.

## Proof

The focused validator self-test must prove order independence, duplicate
rejection, accepted verifier-only paths, and rejected runtime/product paths.
The native receipt packet remains the integration proof for digest and platform
denominators; no product soak is rerun because this issue changes verifier code
only.

## Non-Goals

- Changing Runtime v3, Guardian, kernel, TLS, or lifecycle behavior.
- Regenerating native product proof artifacts.
- Expanding the post-proof allowlist beyond issue-local finalization surfaces.
