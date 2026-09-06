# Issue 507 design

Status: ready for design review.

## Scope

#507 owns the DRT-B six-resident UTS qualification packet. The issue consumes
#506 DRT-A as the deterministic ACIP/replay contract and consumes #345 as the
closed AWS GPU Shepherd proof-runner authority, but it does not reimplement
either predecessor.

The smallest useful result is a deterministic, repo-native qualification lane
that can prove six distinct residents, exact UTS workload completion,
dehydrate/restore population preservation, replay/idempotency receipts,
resource-envelope accounting, and cleanup-zero evidence. Optional paid/GPU
execution remains explicitly gated and must not be claimed by local proof.

## Owned implementation boundary

- `adl-runtime/src/qualification/**`
- `adl-runtime/tests/distributed_contract/**`
- `adl-runtime-kernel/src/**` only if kernel support is strictly needed for
  the six-resident qualification denominator
- `adl/tools/run_issue268_**` only if adapting the retained runner surface is
  necessary for #507 evidence routing
- `docs/milestones/v0.92.1/evidence/runtime/drt-b/**`
- `.csdlc/prepared/issues/507/**` and `.csdlc/evidence/507/**`

## Non-goals

- DRT-C final distributed Runtime qualification (#508)
- DRT-D GCP portability qualification (#509)
- Observatory product redesign or authentic-observatory synthesis
- Unbounded soak
- Production cloud/provider cutover
- Credential discovery or retention
- Paid GPU launch without an explicit, bounded, issue-specific authorization

## Dependency truth

- #506 is terminal in typed cache at PR #616 merge
  `badcf9067da6eb46fc9f59e9da8b11a41e2f24f6`.
- #345 is closed in GitHub. No local derived-terminal cache was observed during
  #507 bootstrap. The prebind validator must therefore accept either a future
  local derived-terminal cache for #345 or live read-only GitHub state reporting
  #345 as `CLOSED`; if neither source is available, it must fail closed before
  bind.

## Design

The DRT-B qualification should extend the deterministic qualification contract
established by DRT-A with a six-resident denominator. The shell validator is a
fast issue-owned contract gate; the focused Rust tests remain the behavioral
proof for construction, duplicate/mutation rejection, and replay semantics.

1. Build a resident roster from actual resident identities, not labels or
   fixture names.
2. Assign one UTS workload item to each resident and emit a receipt for every
   resident/workload pair. Retained JSON evidence must expose a `residents`
   array with exactly six entries, six distinct `resident_id` values, six
   distinct `workload_receipt_id` values, and one workload receipt bound to each
   resident record.
3. Dehydrate the population into a retained snapshot that includes roster
   identity, workload receipts, lineage, replay cursor, and resource-envelope
   summary.
4. Restore from the snapshot and prove exact equality of roster identity,
   workload receipt identity, lineage, replay cursor, and cleanup selectors.
5. Execute negative checks for duplicate resident identity, missing workload
   receipt, mutated lineage, replay cursor regression, and cleanup selector
   mismatch. Retained JSON evidence must name each of those negative cases in a
   `negative_matrix` array with fail-closed decisions.
6. Emit one DRT-B evidence packet under
   `docs/milestones/v0.92.1/evidence/runtime/drt-b/` with redacted, deterministic
   evidence and no credential material.

The local proof should be deterministic and should not require live AWS, GCP,
Unity, model-provider, or browser credentials. If optional GPU evidence is
later required, it must be a separately named, bounded, cost/deadline-controlled
lane that consumes #345 and records cleanup truth without changing the local
qualification denominator.

## Validation plan

Prebind:

- `bash .csdlc/prepared/issues/507/validate-drt-b-six-resident.sh --lane=prebind`

Postbind:

- `bash .csdlc/prepared/issues/507/validate-drt-b-six-resident.sh --lane=six-resident-uts`
- `bash .csdlc/prepared/issues/507/validate-drt-b-six-resident.sh --lane=continuity-reclamation`
- focused Rust tests for the DRT-B qualification module/test surface
- `git diff --check origin/main...HEAD`

Review:

- Fresh no-context exact-head review after immutable implementation commit.

Publication:

- Typed `csdlc-publish` with `Closes #507`, standard-runner CI, then typed
  `csdlc-finish` only when required checks are green.
