# Issue #200 Design — Crash-Safe Authority Reconciliation Barrier

## Problem

#201 produces opaque quorum-approved operation tokens, but independent concrete
stores cannot be made one transaction. Calling those stores directly from
replicated apply would mix deterministic consensus with local clocks and
external effects, while a wrapper that returns success before durable
reconciliation would expose partial authority.

## Outcome

Add one reusable `AuthorityReconciliationBarrier` that accepts only a finalized
#201 token, journals an exact ordered adapter plan before any side effect,
reconciles individually idempotent steps, advances a node-local external
checkpoint, retains the canonical result, and then atomically publishes one
reconciliation generation. Until publication, the barrier issues no current
read or mutation permit for the affected authority lineage.

#203 supplies the production certificate/lease/fence/serving adapters. #204
supplies migration/recovery executors. This issue proves the barrier with a
test-only deterministic adapter and does not mutate a production authority
store.

## Authority boundary

- `FinalizedAuthorityOperation` remains opaque and constructible only by #201.
  The barrier verifies its trust domain, polis, operation id and kind, payload
  and result digests, quorum authorization-time evidence, exact membership
  digest/index, committed log id, protocol checkpoint, and retry namespace.
- `AuthorityAdapterPlan` and `AuthorityStepReceipt` have private fields. A
  crate-private sealed registry selects the adapter kind/version named by the
  token. Production callers cannot inject a trait object, closure, raw receipt,
  or completion boolean. The only nonproduction adapter is compiled under
  `cfg(test)`.
- The node-local checkpoint object is namespaced by trust domain, polis, node,
  guardian, boot generation, protocol instance, authority lineage, adapter
  kind/version, and reconciliation slot. A candidate binds the finalized token,
  ordered plan, step-receipt set, result, retry-cache, and published-view
  digests.
- Private `AuthorityReconciliationPermit` values bind the exact lineage,
  adapter kind/version, published generation, action class (`Read` or one exact
  mutation operation), and operation digest. Possession is not self-authorizing:
  every downstream use must call back into the barrier to verify that the
  permit still matches the current published view and that no newer Pending or
  Reconciling generation exists. Beginning Pending atomically invalidates all
  prior permits. A read permit cannot reach a mutation boundary. #203/#204 must
  require this validation at every authority-restoring read/mutation boundary;
  #200 does not claim those integrations already exist.

Legacy direct `PolisCommand` authority variants, caller-produced tokens,
locally inferred success, raw signing keys, replica-local history, and local
clock values cannot create a plan, receipt, result, or permit.

## Deterministic time

The barrier transports the exact canonical quorum-authorization time evidence
from #201 unchanged. It never samples a local clock to choose a replicated
result. A later adapter may use a local clock only for a pre-step safety gate.
`NotReady` or `Unsafe` from that gate is transient: it performs no external
effect and writes no step receipt, result, canonical failure, or phase advance.
Clock rollback or ambiguity leaves the same step pending so an exact later retry
may continue it. Every durable step input and result remains a deterministic
function of committed token time. Token time digest, uncertainty policy, and
inclusive deadline are part of the plan/checkpoint identity. Drift requires a
new #201 operation rather than reinterpretation.

## State machine

Every operation has one bounded canonical record and the following phases:

1. `Pending`: under an exclusive symlink-safe lock, validate the token and
   current published generation, derive the exact adapter kind/version and
   ordered step plan, and fsync a journal containing old/new checkpoint values,
   expected result/retry digests, and all authority bindings before invoking a
   step.
2. `Reconciling`: invoke one idempotent registered step at a time. After each
   step returns its opaque deterministic receipt, verify its index/input/output
   digests and fsync the updated private record before the next step. Missing,
   duplicate, reordered, forged, or conflicting receipts fail closed.
3. `Checkpointed`: after every exact step receipt is durable, write the
   canonical result/retry cache, perform the external checkpoint CAS, and
   reconcile the old/new/ambiguous outcomes. A completed CAS is never rolled
   back because a later marker write fails.
4. `Published`: write the local checkpoint marker and atomically flip one
   published view generation containing the token/result/plan/receipt/checkpoint
   digests. Only this view can mint a permit. The journal is retained or
   compacted only after the published view is independently readable.

An exact retry first checks the durable result cache so it never reauthorizes or
re-executes adapter steps, but a cache hit is not itself success. The barrier
must verify the bound journal and complete receipt set, reconcile the external
checkpoint, complete a missing local marker or published view, and return only
from exact `Published`. Cached result plus old checkpoint retries the same CAS;
cached result plus exact new checkpoint completes marker/view publication;
missing, corrupt, regressed, or conflicting view/checkpoint state fails closed.
A different token or payload using the same operation id is a conflict.

## Restart and rollback

Restart opens all files through bounded handles, rejects symlink ancestors and
replacement/growth races, canonical-reencodes every complete object, and then
compares journal, state, result cache, published view, and external checkpoint:

- old everywhere resumes the exact pending operation;
- durable step prefix plus old checkpoint resumes at the first missing step;
- all steps/result durable plus old checkpoint retries the same CAS;
- exact new checkpoint plus missing local marker/view completes publication;
- exact published view returns the retained result;
- conflicting checkpoint, coherent local rollback behind the external
  checkpoint, corrupt/noncanonical bytes, unknown adapter version, or ambiguous
  state fails closed and yields no permit.

Initialization uses the same journaled none-to-generation-zero CAS and is safe
under two competing opens. Each store root has one crash-released exclusive
writer lock. Reads are capped by opened-handle metadata plus `MAX+1` streaming;
metadata-check-then-unbounded-read is forbidden.

## Visibility and fail-safe partial progress

The barrier does not claim atomicity across downstream stores. It provides one
authoritative visibility boundary: while any operation for a lineage is
Pending, Reconciling, or Checkpointed, `read_permit` and `mutation_permit` return
`ReconciliationRequired`. Starting a newer Pending generation invalidates every
retained older permit, and every permit use revalidates lineage, adapter,
generation, action, and operation against the live view. Later adapters may
expose only fail-safe facts during that interval, never renewal, activation,
serving eligibility, or a later mutation. Raw partially updated store objects
are not a published authority view.

## Bounds

There is at most one active operation per authority lineage and a configured
bounded number per polis. Operation ids, steps, receipts, payload/result bytes,
journal generations, retry entries, and retained views all have explicit
limits. N+1 admission fails before journal or adapter mutation. Cancellation
closes the caller wait but does not erase durable work; retry/restart reconciles
the same operation. No unbounded queue, scan, allocation, response, or evidence
artifact is permitted.

## Proof

The proof has two layers. Module/unit tests inside the owned
`authority_reconciliation.rs` compile with the only `cfg(test)` sealed
deterministic adapter and prove the complete state/adapter behavior. The
integration target builds the library normally and proves only the externally
reachable opaque API and production denial surfaces; it cannot construct or
register the test adapter. Together they prove one-step and multi-step success,
publication-aware exact retry, denial of read/mutation permits until
publication, current action-scoped permit success, retained-old permit denial,
wrong-lineage/action denial, and read-to-mutation escalation denial. Fault
injection covers
journal introduction, every step before/after effect and receipt fsync,
result-cache write, checkpoint CAS before/after outcomes, marker, view flip, and
restart. Negative cases cover every authority binding, legacy/public forgery,
step ordering/receipt forgery, rollback, checkpoint collision, canonical/bounded
file handling, transient clock gating with no durable advance, capacity, and
unsafe paths.

The exact machine denominator is the following thirty-six unique cases:

`happy_single_step`, `happy_multi_step`, `exact_retry_cached_result`,
`pending_blocks_read`, `pending_blocks_mutation`, `published_permit_current`,
`missing_201_token`, `public_token_forgery_denied`, `legacy_command_denied`,
`wrong_domain`, `wrong_polis`, `wrong_node`, `wrong_guardian`, `wrong_boot`,
`wrong_protocol_instance`, `wrong_membership`, `wrong_operation_kind`,
`wrong_adapter_version`, `wrong_time_digest`, `conflicting_retry`,
`reordered_step`, `duplicate_step`, `missing_step`, `forged_step_receipt`,
`crash_after_journal`, `crash_each_step`, `crash_after_result`,
`crash_before_checkpoint`, `crash_after_checkpoint`, `coherent_rollback`,
`capacity_n_plus_one_no_partial`, `state_or_lock_symlink_rejected`,
`corrupt_journal_rejected`, `noncanonical_state_rejected`,
`opened_handle_growth_rejected`, `checkpoint_object_collision`.

Producer and validator must require exact name/result/marker parity, strict
Clippy, clean protected source, immutable evidence introduction, current review,
and squash-merge-safe source/evidence validation.

## Non-goals

- Production certificate, lease, fence, owner, Shepherd, or Observatory adapter
  behavior (#203).
- Migration/recovery external effects and receipts (#204).
- OpenRaft membership (#199), learner routing/exclusion (#202), Guardian/kernel
  integration, models, AWS, live qualification, final #142 delivery, merge
  without operator authorization, or lifecycle closeout.
