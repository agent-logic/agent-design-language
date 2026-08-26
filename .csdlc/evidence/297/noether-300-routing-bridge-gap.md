# #297 parent integration routing after #300 Noether r1

Date: 2026-08-13

## Source finding

Fresh #300 reviewer `fresh-session:review_300_fresh_r1:noether` reviewed
substantive #300 commit `381a6a7c56d920231d089fe2121211d8086833c8` and returned
FAIL with two P1 findings:

1. #300's cleanup proof used a self-authored terminal envelope, completed
   recovery receipt, and archive manifest instead of cleanup authority produced
   by production recovery.
2. #300's new integration target did not mechanically prove the full approved
   before/after recovery and cleanup failpoint/adversarial matrix; existing
   deep `gate5` proof may only be reused if #300 mechanically invokes or proves
   the same cases in the integrated lane.

## Ownership decision

The live issue graph makes the bridge parent-integration scope:

- #298 implements anchored classification and recovery, and explicitly excludes
  cleanup authorization.
- #299 implements exact-authority cleanup, and consumes a completed recovery
  receipt plus exact archive/canonical manifest authority.
- #300 is a test child and explicitly forbids mock receipts or self-authored
  authority.
- #297 coordinates the complete preserved failed-projection subsystem and owns
  final integrated reconciliation after child truth is terminal/ancestral.

Therefore the missing production recovery-to-cleanup authority bridge is not a
#300 test implementation detail. #300 remains review-failed and unpublished
until #297 (or a new child explicitly split from #297 if the operator chooses)
lands a production bridge that #300 can consume.

## Required bridge contract

The bridge must provide a production-generated authority artifact set that can
be passed to `ArchivedProjectionCleanupRequest` without the #300 harness
authoring authority:

- completed recovery receipt path and digest, schema
  `csdlc.completed_recovery_receipt.v1`;
- canonical archive manifest path and digest, schema
  `csdlc.canonical_archive_manifest.v1`;
- terminal issue/digest/merge binding and canonical/archive root authority;
- archived node list with exact no-follow identity, owner/mode, mount/device,
  type, link-count, size, and digest authority;
- deterministic relationship to the immutable recovery receipt head produced by
  `recover_preserved_projection`;
- idempotent same-operation replay and conflicting-operation rejection; and
- no fallback to constants, test-authored JSON, path strings, or digest-only
  ownership.

## #300 matrix routing

#300 may reuse existing deep #298/#299 tests only when the #300 integration lane
mechanically invokes and records them as part of its proof surface, or when it
enumerates the equivalent integration cases directly. Missing #300 integration
cases currently include:

- recovery before/after boundary enumeration tied to the production failpoint
  registry;
- cleanup before/after boundary enumeration tied to the production failpoint
  registry;
- conflicting operation rejection across recovery and cleanup;
- recovery-produced authority consumed by cleanup without test-authored
  receipts/manifests;
- adversarial topology/identity matrix under the integrated request path; and
- exact evidence that existing `gate5` cases are selected/proving rather than
  merely adjacent.

## Current lifecycle consequence

#300 is implemented but review-failed/unpublished with open P1 findings.
#297 parent card truth must not treat the old broad candidate as completed
integration, and #300 must not publish until the bridge and matrix findings are
fixed and freshly reviewed.
