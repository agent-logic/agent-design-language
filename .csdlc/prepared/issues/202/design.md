# Issue #202 Design — Authenticated Governed Learner Transport

## Problem

#191 intentionally derives a three-voter-only authenticated route cut. #199
needs a fourth node to replicate as an OpenRaft learner before governed
promotion, but a caller address or certificate cannot be allowed to widen the
voter cut. Pending removal also needs one shared durable exclusion authority
that both #201 operation authorization and transport session admission consult,
while still permitting a separately governed replication-only rejoin path.

Merged #201 exposes the coarse `AuthorityOperationKind::Membership` operation
class plus the byte-identical retained artifact through a crate-private sealed
accessor. `EnrollNonVoting` and `RemoveVoter` in this design are canonical
issue-local discriminators inside those membership artifact bytes. They are not
#201 enum variants, are not values of an `operation_kind` field, and never
broaden the public #201 token API.

## Outcome

Extend the existing QUIC/OpenRaft authority with a distinct, bounded learner
topology: the unchanged exact voter cut plus at most one learner named by an
opaque #201 operation token. The learner session permits only AppendEntries and
InstallSnapshot replication. It cannot vote, endorse, finalize authority,
renew, mutate, act as Shepherd, or acquire/serve the polis Observatory.

Add a crash-reconciled `PendingMembershipExclusionAuthority` activated only by
an exact #201 removal token. #201 voter eligibility and ordinary transport
session admission consult its published snapshot. A future governed rejoin uses
a different #201 enrollment token, current identity/certificate/boot, and one
explicit recovery-learner admission without restoring old authority.

The ordering is generation-pinned rather than circular. A membership artifact
whose canonical issue-local discriminator is `RemoveVoter` binds the old
published exclusion generation; #201 publishes and caches that exact finalized
membership operation before #202 may activate exclusion.
Retrieving the exact cached token after activation is cache-first and never
reruns live signer eligibility. Any new prepare/finalize that observes a
different exclusion generation fails and must be prepared again. Once exclusion
is published, the excluded target cannot sign or use ordinary authority, but a
quorum of the remaining nonexcluded voters may authorize a narrowly typed
membership artifact whose canonical issue-local discriminator is
`EnrollNonVoting` for the target's new identity and boot. That exact operation
creates the recovery learner admission; no admission is a prerequisite to
authorizing it.

## Authority construction

- `VerifiedPolisRouteCut` remains the exact voter-only authority and retains its
  strict current voter/configuration parity.
- A new crate-private #202 membership-artifact adapter accepts only a durably
  published `VerifiedAuthorityOperation`, calls #201's sealed
  `artifact_for_sealed_consumer` accessor, requires the coarse operation class
  to be `AuthorityOperationKind::Membership`, and validates the exact retained
  canonical bytes, artifact domain, and issue-local discriminator before
  decoding any learner field. It exposes neither the accessor nor a generic
  artifact conversion publicly.
- A new private-field `VerifiedLearnerAdmission` is constructed only when that
  adapter validates the canonical issue-local `EnrollNonVoting` discriminator
  and its payload digest exactly matches the supplied learner identity,
  guardian, stable Raft id, certificate generation, boot generation, address,
  old voter cut digest, and bounded expiration/operation index, and the exact
  #201 result is durably published.
- Each logical learner-admission lineage has exactly one published current
  transport certificate generation. Rotation requires a distinct successor
  #201 membership operation whose canonical `EnrollNonVoting` payload binds the successor generation, the previous
  admission namespace, the authority-approved overlap end, and the successor
  operation digest. The successor is journaled and staged privately while the
  old admission remains the sole published current admission. One atomic
  successor-generation view flip makes the new token namespace the sole current
  admission and closes every retained old-generation session before another
  request. The old generation is denied at the earliest of its token deadline,
  the authority overlap end, or that published successor flip. During overlap
  the certificate authority may verify both certificates, but neither token
  authorizes the other generation and the topology never exposes two current
  admissions.
- `VerifiedPolisLearnerTopology` contains the unchanged voter cut plus zero or
  one admission. A learner is never inserted into `AuthorityMembership` voter
  configurations or counted toward quorum.
- Caller address/configuration rows are accepted only after byte-for-byte match
  to the token-bound canonical learner payload. A certificate alone, local
  history, DNS result, or matching node name is never admission authority.

## Session and RPC boundary

The dependency-free transport core is rooted at
`distributed/transport/core.rs`. Production learner authority, factory, and
runtime integration live below its private `transport::governed` subtree while
the historical public `distributed::{learner_transport, polis_runtime}` paths
remain source-compatible reexports. Raw QUIC send, receive, handshake, and
authority mutation are private ancestor operations. The factory privately owns
the sole non-clone transport mutation owner; governed descendants receive only
opaque wire sessions, handshake permits, request/response permits, and shared
dispatch leases. No crate sibling can mint an allow-all authority, arbitrary
permit, or exclusion transition.

Every durable authority root generates and checkpoints one nonzero random
transport instance id in a separate canonical object, without changing the
existing admission schema. Reopen preserves that id. Exact peer pins use a
versioned canonical JCS tuple of endpoint role, stable Raft id, node id, and
Guardian id. Both ordinary and learner signed handshakes bind sender and
intended receiver instance ids. A retained or fresh connection from an
alternate authority root is denied, while a legitimate restart with the same
durable root remains accepted.

The signed polis handshake has a role-bound learner variant. A private-field
`EstablishedLearnerSession` can be constructed only from a
`VerifiedPolisLearnerTopology`; generic polis request APIs reject learner
bindings. A typed learner network surface exposes only AppendEntries and
InstallSnapshot sends. OpenRaft's required learner-side `vote` implementation
returns authority denied without opening a stream or sending bytes. The single
server ingress maps a closed message-kind enum and authorizes the opaque session
role before decoding or dispatching a payload; unknown or future kinds fail
closed. The session binds domain, polis, source and target node/guardian
identities, stable Raft id,
certificate and boot generations, exact current voter-cut digest, learner
operation digest, role `replication_only_learner`, sequence namespace, message
kind, payload digest, address, deadline, and protocol version.

For a learner target:

- `append_entries` and `install_snapshot` are permitted through the same bounded
  canonical framing, mTLS/SPKI authorization, replay cache, deadlines, and
  concurrent dispatcher as #191;
- `vote`, client write, #201 prepare/finalize/endorse, lease renewal, mutation,
  Shepherd activation, and Observatory acquisition/serving reject before
  dispatch;
- reconnect requires an exact current admission or its governed successor;
- promotion invalidates the learner namespace and #199 installs the exact voter
  route only after final membership publication;
- removal invalidates the ordinary voter namespace immediately through pending
  exclusion. It does not automatically grant a learner session.

## Shared pending exclusion

`PendingMembershipExclusionAuthority` owns a symlink-safe, exclusive,
size-bounded canonical journal, exact result cache, and external node-local
checkpoint. Activation consumes only a durably published #201 membership
operation whose sealed exact artifact validates the canonical issue-local
`RemoveVoter` discriminator. The decoded payload binds domain, polis, target
identity/guardian/stable Raft id, old voter cut, operation/log index,
certificate and boot generations, reason code, and target membership digest.

Its opaque published snapshot is consulted by:

- #201 signer eligibility and finalization quorum filtering;
- #191/#202 ordinary voter route/session admission and revalidation;
- #199 membership transition admission;
- later #200 concrete renewal, mutation, Shepherd, and Observatory adapters.

The snapshot distinguishes `ordinary_authority_denied` from one exact
`recovery_learner_allowed` admission. Recovery requires a separate current
#201 membership operation whose sealed artifact validates the canonical
issue-local `EnrollNonVoting` discriminator, a new operation/replay namespace,
current certificate and boot generation, and exact target catch-up boundary.
It grants replication only. It never clears the pending exclusion or restores
a vote.
An already-open ordinary connection is revalidated against the published
exclusion generation before every request and is closed before dispatch once
its target is excluded.

Exclusion, successor flip, admission expiry, and authority-cut replacement all
serialize through the same factory transition mutex, stable route-lock order,
exclusive transport fence, current-view update, durable commit, and route
drain. Dispatch takes the route lock and one shared transport lease, revalidates
the exact authority instance, voter-cut digest, admission generation, peer pin,
and exclusion view, then retains that same lease through QUIC stream creation,
the actual OpenRaft effect, and the response send/receive. There is no dropped
lease or nested read acquisition between authorization, effect application, and
response. After a committed transition every retained public entry point fails
before a new governed data stream.

Learner boot attestation is runtime-owned opaque custody. Advancing the durable
boot authority returns a non-clone generation custody bound to the node and
shared authoritative store. Attestation establishment binds that custody to an
exact `LocalNodeGuardianIdentity` and current admission. The custody rereads and
holds the durable generation guard while the nonextractable Guardian signer
signs each live handshake; advancing to generation N+1 makes retained N custody
unable to attest or sign.

## Crash and publication boundary

Admission and exclusion initialization, durable state, external checkpoint,
and result cache use the same reconcile-before-publish discipline as #191/#201.
The expected old and new checkpoint values are journaled before any state or
session-visible change. On restart:

- old everywhere retries only the exact operation;
- exact new checkpoint plus missing local marker completes that marker;
- exact durable state plus old checkpoint retries the same CAS;
- conflicting token, identity, cut, role, certificate, boot, address, digest,
  generation, or checkpoint fails closed.

No session or eligibility snapshot observes a new admission/exclusion until the
state, result cache, and checkpoint agree and one published generation flips.
The exact injected crash denominator covers, independently for admission and
exclusion: journal introduction, durable-state write, result-cache write,
external checkpoint CAS before/after outcomes, local checkpoint marker,
published-view flip, route installation/removal, and restart reconciliation.
Bounded canonical reads use opened-handle size limits and reject symlink
ancestors, replacement/grow-after-open races, MAX+1 bytes, noncanonical bytes,
rollback, and corruption.

## Bounds and lifecycle

There is at most one learner admission and one active membership transition per
polis. A privately staged successor belongs to the same admission lineage and
is never returned by the published topology before its atomic generation flip.
Admission has a bounded log/snapshot catch-up boundary, wall/elapsed
deadline policy, RPC size, concurrent stream count, queue, replay cache, and
retry window. The opaque admission binds an authority-provided absolute deadline
plus a monotonic elapsed budget and uncertainty bound. The trusted clock sample
and derived deadline are persisted in the journal. Transient cancellation closes
only the connection. Durable admission expiry is a separate checkpointed,
idempotent transition driven by that trusted clock; restart clock rollback,
ambiguous elapsed time, and route installation that crosses the deadline fail
closed. Expiry removes only the learner admission/session after durable
publication; it does not mutate voter membership.

The learner transport owns no model, Guardian, API/WSS, or cloud behavior. It
does not choose when the learner is caught up or promoted; #199 does.

## Serial integration order

#200 and #202 both own authority and runtime integration surfaces. The original
serial gate was satisfied before #202 product work: #200 merged, #202
synchronized to the merged ancestry, and the implementation was later rebased
again onto exact current `origin/main` `dbd060d1e`. The current governed subtree
keeps #202's sole-owner mutation boundary separate from #199 membership
coordination. Any future main change touching the protected runtime or typed
card paths requires another exact-main synchronization and proof regeneration
before publication.

## Proof

- A real four-node Quinn/OpenRaft test retains the exact three-voter cut, admits
  one token-authorized learner, replicates log entries and installs a canonical
  snapshot, and proves the learner never votes or accepts authority/client work.
- Removal activates pending exclusion, tears down/rejects the ordinary voter
  namespace, and permits only a separately token-authorized replication-only
  recovery learner with new identity generation.
- Rotation/reconnect tests cover certificate overlap, post-overlap denial, boot
  change, old and successor generation namespaces, cross-generation mismatch,
  lost response/cache-first exact retry before and after exclusion, stale
  admission, and exclusion during an active connection.
- The canonical `certificate_overlap_authorized` proof case must exercise the
  old session during overlap, private successor staging, crash before and after
  the successor view flip, new-generation success, retained-old-session
  revalidation/closure at the flip, cross-generation token mismatch, and old
  denial at each earliest-boundary variant without ever observing two current
  admissions.
- Crash tests inject before and after state write, checkpoint CAS, result-cache
  write, local marker, published-view flip, route installation, and route
  removal for both admission and exclusion.
- Machine evidence binds exact source, commands, the typed named denominator,
  strict Clippy, protected-source drift, immutable introduction, review, and
  squash-merge-safe validation.
- The semantic contract remains exactly thirty-six named learner behaviors.
  The implementation runner contains forty-two passing tests because six
  deterministic infrastructure/race checks supplement those semantic cases.
  Machine proof also requires exactly thirteen public-boundary tests and
  twenty-nine named behavior subassertions.
- Standalone `distributed_transport`, `distributed_discovery`, and
  `distributed_runtime_transport` targets compile unchanged. Deterministic
  races prove the exclusion and expiry writers wait across real request/effect
  and response boundaries, retained sessions emit no post-transition governed
  data stream, boot N cannot sign after N+1, and a fresh alternate-root peer is
  rejected while the persisted same-root peer remains accepted after restart.

## Non-goals

- Issuing #201 operations or performing #199 learner catch-up decisions,
  joint/final membership changes, stable-id publication, or promotion.
- Concrete certificate, lease, FencingStore, owner, Shepherd, Observatory,
  migration, or recovery store reconciliation (#200).
- Guardian/kernel/API/WSS integration, models, AWS, live demonstrations, final
  #142 delivery, merge without operator authorization, or lifecycle closeout.
