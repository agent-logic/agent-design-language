# Issue #199 Design — Govern Learner, Joint, and Final Membership

## Problem

WP-04.16a supplies secure durable OpenRaft, #201 supplies opaque
quorum-approved authority-operation tokens, and #202 supplies an authenticated
replication-only learner route plus a shared pending-exclusion authority. The
Runtime still needs one crash-resumable coordinator that changes OpenRaft
membership and publishes matching `MembershipState`, `AuthorityMembership`,
and route truth. A learner, removed voter, leader, caller configuration, or stale
local history must never self-promote or retain voting authority.

## Outcome

Add a bounded `MembershipTransitionCoordinator` that consumes only an exact
#201 membership-operation token and the merged #202 learner/exclusion authority.
It performs governed non-voting enrollment, learner catch-up, standard OpenRaft
membership change, durable joint/final observation, and reconcile-before-visible
Runtime membership publication. Exact retries and restarts resume the same
transition.

This issue does not create #201 authority tokens or reimplement transport. It
does not apply certificate, lease, FencingStore, owner, Shepherd, Observatory,
migration, or recovery side effects; those concrete adapters are #200. It does
not launch Guardian/kernel processes or run Wuji/AWS demos.

## Authoritative input and stable identities

- A private-field #201 token binds polis, trust domain, operation id, committed
  index, the coarse `AuthorityOperationKind::Membership`, exact old configuration
  digest, canonical payload digest, and result/checkpoint identity. The reused
  #202 `EnrollNonVoting` and `RemoveVoter` artifacts carry the exact candidate
  identity, voter-cut digest, target-membership digest, deadline, and canonical
  metadata; they do not carry complete guardian-to-Raft-id maps. The #199-owned
  `PromoteVoter` artifact additionally binds the candidate Raft id and exact old
  and target stable-map digests. The complete maps are loaded from durable local
  state and accepted only when their canonical digests match that artifact.
- The old cut is accepted only when `MembershipState`, `AuthorityMembership`,
  #191 durable OpenRaft membership/history, #202 route/exclusion view, and the
  currently published membership view agree exactly on domain, index, voters,
  guardians, control keys, certificate generations, stable Raft ids, and every
  nonempty joint configuration.
- Existing Raft ids never change. A candidate id must be nonzero, absent from
  every current identity, and equal to the id in the digest-bound token payload.
  `AuthorityMembership` gains a collision-checked stable-map constructor; its
  legacy sorted-enumeration constructor is not used for governed transitions.
- Caller addresses remain transport hints for the one already-authorized #202
  learner session. They never choose identity, role, certificate, or Raft id.

## Operation model

Every transition uses the one coarse #201 `Membership` operation kind. The
sealed canonical artifact carries one issue-local discriminator; it is never
represented as a new `AuthorityOperationKind` variant or inferred from caller
input:

1. `EnrollNonVoting` inserts a new or previously removed candidate into
   `MembershipState` as `NonVoting`, records its stable Raft id in the
   coordinator registry, and authorizes only the #202 replication learner route.
   It does not alter the OpenRaft voter set.
2. `PromoteVoter` requires that exact non-voting enrollment, catches the learner
   to the authorized boundary, changes OpenRaft membership, and only after final
   commitment publishes the candidate as a Runtime voter.
3. `RemoveVoter` begins shared pending exclusion, changes OpenRaft membership
   with `retain=false`, and publishes final absence plus a pending-exclusion
   receipt that #200 uses for concrete fence reconciliation.

Governed rejoin is deliberately two operations: a new `EnrollNonVoting` token
followed by a separate `PromoteVoter` token. Retained local membership, logs,
snapshots, ids, roles, tokens, or certificates cannot combine those operations
or restore voting authority.

`EnrollNonVoting` and `RemoveVoter` reuse the exact sealed artifact contracts
merged with #202. `PromoteVoter` is a #199-owned sealed artifact discriminator
under the same coarse `Membership` kind. The coordinator consumes artifacts
only through the crate-private sealed accessor and validates exact canonical
domain, bytes, digest, old/target cut, operation id, and committed index.

#199 adds one narrow governed observation surface to the #202 factory boundary.
Successful learner-admission or pending-exclusion activation returns an opaque
`GovernedMembershipAuthorityReceipt` containing the canonical operation digest,
external durable generation, and published state digest. A read-only observation
method returns that same receipt only while the exact operation remains current,
or `None` otherwise. Receipt construction, durable admission/exclusion state,
route mutation, and transition ownership remain private to #202. Callers cannot
forge a receipt, choose a generation, or obtain the private state behind it.

## Non-voting enrollment state machine

`EnrollNonVoting` has its own bounded durable protocol rather than borrowing the
voter-transition phases:

1. `EnrollmentAuthorized` journals the exact #201 token and sealed
   `EnrollNonVoting` artifact, old and target
   non-voting published-view digests, old and target stable-id registry digests,
   exact #202 learner-admission digest, expected checkpoint, result digest, and
   three distinct indices: the #201 authority-protocol committed index, the
   next Runtime membership-event index, and the unchanged current OpenRaft
   membership log id. These indices are never inferred equal.
2. `LearnerAdmissionRequested` invokes only the merged #202 governed factory
   port with the verified admission and exact current time. The coordinator
   does not stage, mutate, or transact over #202 private state. It journals the
   requested operation digest and returned
   `GovernedMembershipAuthorityReceipt` before proceeding.
3. `EnrollmentReconciled` observes that exact receipt again through the
   governed read-only port, then idempotently prepares the local
   `MembershipState` Join-as-NonVoting event and collision-checked stable-id
   registry entry. Their canonical old/new digests are recorded separately.
   A missing, different, expired, or superseded #202 generation fails closed.
4. `EnrollmentCheckpointed` writes the canonical result and exact retry entry,
   advances the external node-local checkpoint, and reconciles either side of
   every local-write/CAS/result-marker crash window.
5. `EnrollmentPublished` atomically flips one durable local enrollment-view
   generation containing the reconciled local digests and the observed #202
   generation/result identity. Only this view makes the local non-voting
   candidate visible. `PromoteVoter` must bind both that exact local generation
   and the still-current externally owned #202 admission generation.

Restart compares the enrollment journal, local prepared objects, result cache,
checkpoint, published generation, and the #202 governed observation. An absent
external call is retried only with the exact operation identity; an exact
already-published #202 generation is observed and recorded; a later local step
repairs only its missing marker. Any conflicting member, stable id, external
generation/result, index, digest, token, checkpoint, or local generation fails
closed. This is an idempotent saga across an owned external authority, not one
atomic transaction or private #202 staging protocol.

## Voter transition state machine

Promotion and removal use one durable transition record and only these phases:

1. `AuthorizedOld`: persist the exact token, stable maps, old/target cuts,
   expected checkpoint, and operation digest before an OpenRaft side effect.
   Removal invokes the governed #202 exclusion activation port and journals its
   exact `GovernedMembershipAuthorityReceipt`. It does not mutate #202 private
   state directly or claim that local and external publication are atomic.
   From this phase the target cannot create ordinary voter sessions, endorse
   #201 operations, renew, mutate, become Shepherd, or acquire/serve the polis
   Observatory. The only permitted exception is the exact governed #202
   replication-only learner route for a later rejoin operation.
2. `LearnerCaughtUp`: promotion invokes the standard OpenRaft learner API once
   and requires matched log or installed canonical snapshot to reach the exact
   authorization boundary. Removal records an explicit not-applicable marker.
3. `JointCommitted`: invoke the standard OpenRaft membership-change API and
   verify a durable membership-history entry containing the exact old and new
   voter configurations. Progress requires the standard majority of every
   nonempty configuration.
4. `FinalCommitted`: verify a later durable membership-history entry containing
   the exact uniform target configuration and final log id. A successful leader
   response without those durable observations is insufficient.
5. `AuthorityParityReconciled`: persist a publication intent, observe the exact
   current #202 admission/exclusion operation and generation receipts, then
   idempotently reconcile local `MembershipState` and stable-map
   `AuthorityMembership`. Verify all observed external and local digests against
   final OpenRaft membership. The coordinator never rewrites #202 route or
   exclusion objects.
6. `AuthorityParityPublished`: after the exact external checkpoint and result
   cache are durable, atomically flip one durable published-view generation.
   All coordinator, route, and later #200 accessors consume only that view.

OpenRaft may implement joint and final consensus inside one
`change_membership(..., retain=false)` call. The coordinator does not invent a
second consensus algorithm. `PolisStateMachineStore` is extended narrowly to
persist one bounded canonical membership-history record for every applied
membership log entry before acknowledging its apply batch, so joint and final
remain observable even when applied in the same batch. A crash that leaves the
cluster joint is resumed only when that exact durable joint digest matches the
authorized target; the standard API is then used to finish uniform membership.

## Removal and rejoin

- `retain=false` is mandatory for full removal. An old voter is not silently
  retained as a learner by the removal call.
- The shared #202 exclusion authority distinguishes ordinary sessions from an
  exact later recovery admission. An excluded node has no generic route, vote,
  endorsement, renewal, mutation, Shepherd, or Observatory authority.
- Final removal invalidates its old voter retry/session namespace. A later
  `EnrollNonVoting` token may authorize one fresh replication-only identity and
  boot/certificate generation; promotion remains a separate token and phase.
- A removed node reconnecting without that new token receives no data plane.

## Crash and publication boundary

Each voter owns an exclusive, symlink-safe, size-bounded canonical transition
journal and an external node-local checkpoint namespace bound to domain, polis,
node, guardian, boot generation, protocol version, operation id, old/target cut
digests, stable-map digest, Raft log id, coordinator phase, reconciled object
digests, published-view generation, and result digest.

Before enrollment staging, learner, membership-change, exclusion, voter
reconcile, checkpoint, result, or view-flip side effects, the exact pending
phase is durable. On restart the
coordinator compares its journal, checkpoint, bounded membership history,
current OpenRaft membership/metrics, shared exclusion state, private reconciled
objects, published view, and exact retry cache:

- if the side effect did not happen, retry only the same exact operation;
- if its exact expected state is already durable, advance;
- if a later safe phase is proved, reconcile the missing local marker;
- if any cut, map, log id, token, checkpoint, route, object, view, or result
  conflicts or regresses, fail closed and require operator recovery.

No local public membership/authority view or downstream receipt changes until
all external generation observations, local reconciliation, checkpoint, and
result steps complete and the local published-view generation flips. #202 may
already expose its own durable admission or exclusion generation; until exact
parity is observed, #199 remains fail closed and publishes no local parity.
This is a crash-resumable idempotent saga, not a transaction over #202 or #200.

## Concurrency and authority changes

Only one transition may be active per polis. A second operation, conflicting
retry, stale leader, or membership-generation change fails before a side effect.
A new leader resumes the exact journaled operation or starts none. Concurrent
add/remove requests never compose implicitly.

Every phase revalidates the live certificate and committed membership required
for that phase. Expired, revoked, superseded outside authorized overlap,
wrong-purpose, wrong-key, wrong-boot, or wrong-generation identities fail
closed. Rotation requires an exact governed token and cannot rewrite stable
Raft ids.

## Proof

- A real authenticated four-node test uses #202 to enroll a non-voter learner,
  catches it up, promotes it, observes same-batch joint and final history, flips
  exact authority parity, removes an old voter with `retain=false`, and proves
  pending exclusion from ordinary sessions and endorsements.
- A committed-joint restart test stops the leader after joint persistence,
  elects/resumes safely, and finishes only the authorized uniform target.
- Rejoin restarts the removed node with old state and proves separate new
  enrollment and promotion tokens, current certificate/boot, fresh learner
  route, catch-up, joint/final commitment, and publication are all required.
- Stable-id proof adds/removes lexically earlier identities without changing any
  old id and rejects duplicate, zero, missing, remapped, or colliding ids.
- Crash proof injects before/after the enrollment journal, before/after the
  governed #202 admission call and generation observation, local
  MembershipState preparation, stable-map preparation,
  enrollment checkpoint/result, enrollment view flip, before/after governed
  exclusion activation and generation observation, learner call,
  joint history write, final history write, each voter reconcile, voter
  checkpoint/result, and voter visible-view flip.
- The retained proof uses twelve behavior-specific public cases rather than a
  substituted name facade. Nine production assertions separately bind the
  governed receipt, exact authorized cuts, current-operation history, durable
  crash/retry classification, real removal and fresh-node rejoin, checkpoint,
  parity, publication, and strict Clippy surfaces.

## Non-goals

- Creating or verifying #201 endorsements or tokens.
- Implementing #202 transport or pending-exclusion authority.
- Applying concrete certificate, lease, FencingStore, owner, Shepherd,
  Observatory, migration, or recovery store mutations (#200).
- Kernel continuity, Guardian/API/WSS, models, AWS, live demonstrations, final
  #142 delivery, merge without operator authorization, or lifecycle closeout.
- Replacing OpenRaft joint consensus with custom voting logic.
