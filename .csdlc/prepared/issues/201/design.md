# Issue #201 Design — Establish the Committed Authority Protocol

## Problem

WP-04.16a provides authenticated three-voter OpenRaft transport and durable
consensus storage. The Runtime still needs a deterministic protocol that turns
an exact committed intent plus endorsements from current voters into an opaque
quorum-approved operation token. A leader, harness, caller, or replica-local
clock must not self-authorize merely because Raft replicated some bytes.

## Outcome

Add one deterministic committed-command protocol that consumes the exact
current `MembershipState` and `AuthorityMembership`, verifies endorsements from
an opaque local voter authority, durably publishes an exact result and retry
record through an external checkpoint, and emits an opaque operation token for
the downstream membership and concrete-store integrations.

This issue does not apply OpenRaft membership changes or mutate certificate,
lease, fencing, migration, or recovery stores. Governed membership is #199;
concrete-store reconciliation is #200. Kernel continuity, Observatory
presentation, models, cloud provisioning, and live qualification remain later
#142 children.

## Command protocol

Every state-changing operation uses two committed entries:

1. `PrepareAuthorityIntent` commits a canonical, domain-separated intent with
   polis id, trust domain, current membership epoch/index and voter-set digest,
   operation kind, expected prior protocol checkpoint, payload digest,
   canonical quorum-attested time token, and a unique bounded operation id.
2. Each current voter may endorse only that exact committed intent through an
   opaque `VoterEndorsementAuthority` bound to node, guardian, voter purpose,
   certificate generation, boot generation, and membership index. Raw signing
   keys and caller-produced endorsements are not accepted.
3. `FinalizeAuthorityIntent` carries the intent digest and endorsements. The
   state machine verifies a strict quorum against exact current
   `AuthorityMembership`; the leader cannot synthesize missing voters.
4. A durable protocol journal records the finalized token and canonical result.
   The result becomes readable only after the exact external protocol checkpoint
   CAS and retry record are durable.

Exact retries return the retained canonical result. Conflicting reuse,
superseded membership, wrong-domain evidence, missing or duplicate voters,
invalid keys, expired intent, rollback, and reordered finalization fail before
protocol publication.

The intent contains one canonical quorum-attested time token. Replica-local
clocks may determine whether a voter is willing to endorse, but replicated apply
consumes identical committed time bytes on every voter and cannot branch on a
local wall or monotonic clock.

## Authority boundary

- Voter identity, guardian, control key, voter purpose, and Raft id derive from
  concrete `MembershipState` plus exact `AuthorityMembership` parity. Caller
  route or configuration data is never voter authority.
- The protocol produces a private-field `VerifiedAuthorityOperation` token.
  Only #199 and #200 may consume it; public constructors, raw endorsements, and
  caller-selected quorum sets are absent.
- Membership coordination, certificate/lease/fence application, and external
  migration/recovery workflows do not run inside deterministic Raft apply here.
- Replay identity binds polis, peer generation, boot generation, committed
  index, operation id, request digest, and canonical response.
- Legacy `PolisCommand` variants that directly mint membership, fence, owner,
  Shepherd, Observatory, migration, or recovery authority are retired or
  explicitly rejected. Retained logs and snapshots are versioned and either
  migrate without minting authority or fail closed.

## Crash and publication boundary

The protocol owns a symlink-safe, exclusively locked, size-bounded canonical
journal. Initialization and each finalized result record expected old and new
external protocol checkpoints before publication. On restart it compares the
journal, result cache, and checkpoint authority:

- old everywhere: retry the exact step;
- new everywhere: advance the journal;
- a proved partial protocol commit: finish only the same exact operation;
- conflicting, regressed, ambiguous, or missing checkpoint authority: fail
  closed and require recovery/operator action.

No downstream consumer may receive a token whose protocol publication barrier
is incomplete. The Raft apply callback acknowledges only after the canonical
result, retry record, and external checkpoint are durable. This is per-object
protocol atomicity, not a transaction over downstream authority stores.

## Downstream split

- #199 consumes membership-operation tokens and implements
  `AuthorizedOld -> LearnerCaughtUp -> JointCommitted -> FinalCommitted ->
  AuthorityParityPublished`, including removal fencing and crash reconciliation.
- #200 consumes concrete-operation tokens and reconciles existing certificate,
  lease, fencing, owner, Shepherd, migration-token, and recovery-token
  authorities without claiming cross-store atomicity.
- #193 and later children consume those merged authorities for real kernel
  continuity and operational serving. They do not broaden this protocol.

## Proof

- Real three-voter OpenRaft tests prepare and finalize commands through the core
  protocol and never construct endorsements or verified tokens in the harness.
- Positive cases cover strict current quorum, canonical committed time,
  durable token publication, exact retry, and downstream opacity.
- Fault cases cover missing/duplicate/wrong-key endorsements, signer
  unavailability and rotation, membership mismatch, local clock skew at the
  endorsement boundary, crash at every journal/checkpoint boundary, rollback,
  replay conflict, corruption, capacity, symlink paths, and every retired
  legacy authority command.
- Machine evidence binds exact source, commands, a named nonzero denominator,
  strict Clippy, marker parity, protected-source drift, immutable evidence
  introduction, and eventual squash-merge topology.

## Non-goals

- OpenRaft learner/joint/final membership changes (#199).
- Concrete certificate, lease, fence, owner, Shepherd, migration, or recovery
  side effects (#200).
- Kernel checkpoint bundle export/import or snapshot materialization.
- Guardian/API/WSS/Observatory listener integration.
- Models, AWS infrastructure, live demonstration, final #142 delivery, or
  lifecycle closeout.
- Reimplementation of QUIC/OpenRaft transport or any existing authority store.
