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
  index, operation kind, exact old configuration digest, canonical payload
  digest, and result/checkpoint identity. For membership operations, the
  digest-bound canonical payload contains the candidate identity and role,
  expected certificate generation, exact authorized candidate Raft id, and the
  complete old and target guardian-to-Raft-id mappings.
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

Membership change is expressed through three exact token kinds:

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

## Voter transition state machine

Promotion and removal use one durable transition record and only these phases:

1. `AuthorizedOld`: persist the exact token, stable maps, old/target cuts,
   expected checkpoint, and operation digest before an OpenRaft side effect.
   Removal atomically activates the shared #202 pending-exclusion authority.
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
5. `AuthorityParityReconciled`: persist a publication intent, idempotently
   reconcile the exact `MembershipState`, stable-map `AuthorityMembership`, and
   #202 route/exclusion state, and verify their digests against final OpenRaft
   membership. Intermediate objects remain private and non-authoritative.
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

Before learner, membership-change, exclusion, concrete reconcile, checkpoint,
or view-flip side effects, the exact pending phase is durable. On restart the
coordinator compares its journal, checkpoint, bounded membership history,
current OpenRaft membership/metrics, shared exclusion state, private reconciled
objects, published view, and exact retry cache:

- if the side effect did not happen, retry only the same exact operation;
- if its exact expected state is already durable, advance;
- if a later safe phase is proved, reconcile the missing local marker;
- if any cut, map, log id, token, checkpoint, route, object, view, or result
  conflicts or regresses, fail closed and require operator recovery.

No public membership/route/authority view or downstream receipt changes until
all private reconciliation, checkpoint, and result steps complete and the
single published-view generation flips. This is coordinated fail-closed
publication, not a transaction over #200 stores.

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
- Crash proof injects before/after every enrollment, exclusion, learner call,
  joint history write, final history write, each concrete reconcile, checkpoint,
  result-cache write, and visible-view flip.
- The exact typed 36-case denominator additionally covers stale cuts, wrong
  keys/certificates/domain, learner lag/divergence, missing snapshots, old-only
  and new-only joint progress, concurrency, retry conflict, rollback,
  corruption, capacity, and unsafe paths.

## Non-goals

- Creating or verifying #201 endorsements or tokens.
- Implementing #202 transport or pending-exclusion authority.
- Applying concrete certificate, lease, FencingStore, owner, Shepherd,
  Observatory, migration, or recovery store mutations (#200).
- Kernel continuity, Guardian/API/WSS, models, AWS, live demonstrations, final
  #142 delivery, merge without operator authorization, or lifecycle closeout.
- Replacing OpenRaft joint consensus with custom voting logic.
