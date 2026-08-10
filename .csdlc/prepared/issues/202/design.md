# Issue #202 Design — Authenticated Governed Learner Transport

## Problem

#191 intentionally derives a three-voter-only authenticated route cut. #199
needs a fourth node to replicate as an OpenRaft learner before governed
promotion, but a caller address or certificate cannot be allowed to widen the
voter cut. Pending removal also needs one shared durable exclusion authority
that both #201 operation authorization and transport session admission consult,
while still permitting a separately governed replication-only rejoin path.

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

The ordering is generation-pinned rather than circular. A `RemoveVoter`
prepare/finalize binds the old published exclusion generation; #201 publishes
and caches that exact finalized token before #202 may activate exclusion.
Retrieving the exact cached token after activation is cache-first and never
reruns live signer eligibility. Any new prepare/finalize that observes a
different exclusion generation fails and must be prepared again. Once exclusion
is published, the excluded target cannot sign or use ordinary authority, but a
quorum of the remaining nonexcluded voters may authorize a narrowly typed
`EnrollNonVoting` recovery intent for the target's new identity and boot. That
token creates the recovery learner admission; no admission is a prerequisite to
authorizing the token.

## Authority construction

- `VerifiedPolisRouteCut` remains the exact voter-only authority and retains its
  strict current voter/configuration parity.
- A new private-field `VerifiedLearnerAdmission` is constructed only from a
  `VerifiedAuthorityOperation` whose operation kind is `EnrollNonVoting`, whose
  canonical payload digest matches the supplied learner identity, guardian,
  stable Raft id, certificate generation, boot generation, address, old voter
  cut digest, and bounded expiration/operation index, and whose exact result is
  durably published.
- Each admission binds exactly one transport certificate generation. A
  certificate rotation requires a distinct successor #201 token whose payload
  binds the successor generation, the previous admission namespace, the
  authority-approved overlap end, and the successor operation digest. During
  overlap the certificate authority may verify both certificates, but neither
  token authorizes the other generation. The old admission is denied at its
  token deadline or overlap end, whichever is earlier.
- `VerifiedPolisLearnerTopology` contains the unchanged voter cut plus zero or
  one admission. A learner is never inserted into `AuthorityMembership` voter
  configurations or counted toward quorum.
- Caller address/configuration rows are accepted only after byte-for-byte match
  to the token-bound canonical learner payload. A certificate alone, local
  history, DNS result, or matching node name is never admission authority.

## Session and RPC boundary

The existing signed polis handshake gains a role-bound session variant. A
private-field `EstablishedLearnerSession` can be constructed only from a
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
checkpoint. Activation consumes only an exact #201 `RemoveVoter` token and
binds domain, polis, target identity/guardian/stable Raft id, old voter cut,
operation/log index, certificate and boot generations, reason code, and target
membership digest.

Its opaque published snapshot is consulted by:

- #201 signer eligibility and finalization quorum filtering;
- #191/#202 ordinary voter route/session admission and revalidation;
- #199 membership transition admission;
- later #200 concrete renewal, mutation, Shepherd, and Observatory adapters.

The snapshot distinguishes `ordinary_authority_denied` from one exact
`recovery_learner_allowed` admission. Recovery requires a separate current
#201 `EnrollNonVoting` token, new operation/replay namespace, current
certificate and boot generation, and exact target catch-up boundary. It grants
replication only. It never clears the pending exclusion or restores a vote.
An already-open ordinary connection is revalidated against the published
exclusion generation before every request and is closed before dispatch once
its target is excluded.

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
polis. Admission has a bounded log/snapshot catch-up boundary, wall/elapsed
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
- Crash tests inject before and after state write, checkpoint CAS, result-cache
  write, local marker, published-view flip, route installation, and route
  removal for both admission and exclusion.
- Machine evidence binds exact source, commands, the typed named denominator,
  strict Clippy, protected-source drift, immutable introduction, review, and
  squash-merge-safe validation.

## Non-goals

- Issuing #201 operation tokens or performing #199 learner catch-up decisions,
  joint/final membership changes, stable-id publication, or promotion.
- Concrete certificate, lease, FencingStore, owner, Shepherd, Observatory,
  migration, or recovery store reconciliation (#200).
- Guardian/kernel/API/WSS integration, models, AWS, live demonstrations, final
  #142 delivery, merge without operator authorization, or lifecycle closeout.
