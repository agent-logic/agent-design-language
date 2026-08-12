# Issue #296 design: implemented authored-design refresh

## Problem boundary

An implemented issue may discover during an exact-head review that its authored
design or diagram is stale. `recover_review` clears review and publication
authority, but ordinary SPP edits remain phase-guarded and the existing
pre-bind binding refresh is intentionally limited to initialized or ready
records. The operator must not hand-edit cards or lifecycle state.

Issue #296 adds one explicit typed operation for this recovery case. It does
not change #294 product behavior, rewrite historical audit events, reopen bound
topology, or discard implementation evidence.

## Proposed contract

Add a semantic operation named `refresh_authored_design_after_recovery`. The
request remains an ordinary `csdlc-edit apply` request and therefore carries
the issue, card, actor, reason, expected generation, and expected digest. The
operation is valid only for the SPP card and takes no caller-supplied digests or
paths. The canonical record's existing repository-relative `design_path` and
`diagram_path` are the sole artifact identities.

Authorization resolves the canonical registered worktree from the bound record
and Git-common topology, rejects execution from any other checkout or aliased
path, and acquires the Git-common issue/binding lock namespace used across all
linked worktrees. A root-local `.csdlc/locks` file is not sufficient authority.
Within that canonical transaction, authorization requires all of the following:

- phase is exactly `implemented`;
- the most recent review-authority audit event is `recover_review` at the
  current generation;
- that recovery corresponds to the latest transition back to implemented when
  such a transition is required by the recovered prior phase;
- review assignment, review, publication, readiness, and terminal truth are
  absent;
- branch and worktree topology remain unchanged and valid;
- actor and reason are non-empty;
- expected generation and digest match the current canonical record.

## Atomic state change

While holding the issue lock, the operation opens both authored artifacts
through the repository-owned no-follow path walk, verifies that each is a
regular single-link file, and retains both handles. It computes both digests
from those handles before changing any in-memory card. Immediately before the
canonical commit it revalidates both retained handle identities and bytes
against fresh no-follow opens of the canonical paths. This paired final
revalidation is one authorization boundary: either both canonical paths still
name the exact single-link bytes that were digested, or the operation fails
without mutation. Missing, unsafe, symlinked, hardlinked, non-regular, or
concurrently drifted artifacts fail closed. Non-Unix platforms must provide an
equivalent retained-identity/single-link guarantee or reject the operation.

After both reads succeed, the operation updates the SPP and VPP design and
diagram digests together, sets `design_review` to `pending`, increments the
generation, hydrates projections, computes the canonical record digest, and
uses the existing atomic store commit. The audit event records the operation,
the design and diagram references, both old and new digests, and prior approval
provenance. Existing transitions, execution evidence, validation evidence,
branch, worktree, and phase remain unchanged.

The operation succeeds even when an artifact digest is unchanged only if the
other authored artifact changed; a complete no-op is rejected to avoid using
refresh as an approval-reset mechanism without authored change.

## Subsequent authority

Review assignment and publication must continue to fail while design review is
pending. The existing `approve_design` route may reapprove bound or implemented
records, but approval must use a canonical lowercase
`fresh-session:<8-4-4-4-12 UUID>` reviewer and must bind the exact refreshed
design-ref/design-digest plus diagram-ref/diagram-digest tuple. `approve_design`
must fail closed whenever review assignment, review, publication, readiness, or
terminal authority exists; changing approval after assignment requires the
normal typed recovery to clear that authority first. `assign_review`
must reject any record whose approval does not cover that complete current
tuple. That guard is checked both before constructing assignment truth and
again while holding the Git-common canonical lock so a design-only,
diagram-only, approval-reset, wrong-worktree, or linked-worktree race cannot
create reviewer authority. Only then may implementation validation and a new
exact-head implementation review proceed.

## Failure and concurrency model

- stale generation or digest: reject before artifact reads;
- wrong card, phase, or missing current recovery provenance: reject;
- active or retained review/publication/readiness/terminal truth: reject;
- unsafe, missing, multi-link/aliased, or drifted design/diagram: reject;
- failure between artifact reads or during canonical commit: preserve the
  pre-operation canonical record through existing atomic commit recovery;
- any concurrent artifact replacement detected by the safe reader: reject;
- never delete or rewrite prior audit or transition entries.

## Focused proof

Tests cover a successful implemented recovery refresh; SPP/VPP parity; pending
approval; preserved implementation phase, topology, execution evidence, and
history; exact old/new audit provenance; new canonical design approval; and
review assignment blocked until approval. Negative tests cover wrong phase,
wrong card, missing/currently stale recovery provenance, active authority
fields, stale CAS, malformed reviewer UUIDs, missing/unsafe/drifted artifacts,
pre-existing hardlinks for both artifacts, replacement between paired reads
and final revalidation, replacement before commit, partial read or commit
failure, assignment/approval-reset and diagram-only drift races, wrong-worktree
and aliased-worktree invocation, and concurrent linked-worktree attempts under
the Git-common lock. Tests also prove same-phase approval rewrite is rejected
after assignment and that reviewed/published/terminal approval remains
rejected. Existing initialized/ready repair and normal approval
behavior remain green. A deterministic local lane proves AC-1 through AC-8;
AC-9 is a deferred live dependency observation that can pass only after #296 is
terminal and ancestral to #294 and therefore is not claimed by the unit lane.
