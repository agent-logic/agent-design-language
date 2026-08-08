# Distributed Guardian Architecture

Status: Frozen design gate for v0.92 WP-04

## Scope And Authority

Runtime v3 extends Guardian lifecycle ownership across authenticated nodes while
preserving the single-node boundary accepted by ADR 0054. Guardian remains
process 0 on every node. A distributed peer may provide evidence and may become
an owner only through the lease, fencing, migration, and commit protocol below.
Discovery, telemetry, cognition, and the network transport never grant
authority by themselves.

This document freezes the contract implemented by WP-04.01 through WP-04.16.
It is architecture, not product-completion evidence. Until WP-04.16 passes,
Runtime v3 remains the WP-03 single-node system.

## Invariants

1. One Runtime lineage has at most one active authoritative Guardian.
2. Every node and Guardian has a durable identity rooted in one explicit trust
   domain. A copied state directory does not create a new identity.
3. Enrollment is explicit, authenticated, replay-resistant, and fail-closed.
4. Every authority-bearing message binds the trust domain, node identity,
   Guardian identity, lineage, epoch, operation, nonce, and expiry.
5. Epochs are monotonic. A lease is bounded, renewable only by its current
   holder, and never valid after expiry or fencing.
6. Source authority survives relocation until target validation and fencing
   succeed. Target activation precedes commit only after the source is fenced.
7. Ambiguous ownership stops execution. Availability cannot override fencing.
8. Durable identity, certificate, membership, lease, fencing, migration, and
   audit records survive restart and are validated before readiness.

## Component Model

Each node contains one external Guardian and one Guardian-owned Runtime kernel.
The distributed module is registered only by WP-04.16 and exposes these bounded
services:

- identity and enrollment;
- certificate lifecycle;
- authenticated transport;
- seed discovery and join;
- membership and failure classification;
- epoch, lease, and fencing authority;
- signed capability and resource-weather evidence;
- deterministic placement;
- snapshot catalog and transfer manifests;
- migration, rollback, and recovery;
- redacted distributed projection and observability.

The Guardian owns child launch, restart, shutdown, and the authority lease. The
kernel owns typed state transitions and the authenticated API/WSS projection.
The Observatory and other consumers remain clients and cannot enroll nodes,
mint certificates, issue leases, or activate migrations.

## Identity And Enrollment

WP-04.01 creates stable node and Guardian identities. Enrollment accepts a
bounded signed request only from an operator-approved trust root, proves
possession of the enrolling key, consumes a nonce once, and records the trust
domain and identity generation durably. Duplicate enrollment is idempotent only
for the same identity and generation. Wrong-domain, expired, replayed, cloned,
or conflicting requests are rejected and audited.

Restoration verifies that durable identity agrees with configured trust roots
and certificate subjects before the node becomes ready. A mismatch quarantines
distributed operation while leaving local recovery available.

## Certificate Purposes And Lifecycle

WP-04.02 separates certificate and key purposes:

| Purpose | Holder | Permitted use |
| --- | --- | --- |
| Node identity | Node | Enrollment and durable node identity proof |
| Guardian control | Guardian | Guardian-to-Guardian authority messages |
| Transport | Runtime endpoint | QUIC/TLS peer authentication and encryption |
| Advertisement signing | Guardian | Capability and resource-weather evidence |
| Snapshot signing | Current owner | Catalog and transfer-manifest integrity |

Keys and certificates are not interchangeable across purposes. Rotation uses a
bounded overlap window and monotonically increasing generation. Revocation and
certificate expiry take effect before new sessions or authority renewal.
Certificate compromise fences the affected identity, withdraws advertisements,
rejects new transport, and requires operator-authorized re-enrollment. No
plaintext or verification-disabled recovery mode exists.

## Maintained QUIC/TLS Transport

WP-04.03 integrates a maintained QUIC implementation over TLS with mutual
authentication. The implementation uses library framing, cryptography, chain
validation, revocation input, flow control, stream bounds, cancellation, and
idle timeouts. ADL adds typed application messages above the maintained
transport; it does not add custom cryptography or custom wire framing.

Every session binds the authenticated peer to the expected node, Guardian,
trust domain, protocol version, and certificate purpose. Oversized, malformed,
unknown-version, expired, replayed, or peer-mismatched messages close the
bounded stream and emit a redacted reason code. Transport liveness is evidence,
not membership or ownership authority.

## Discovery, Join, And Membership

WP-04.04 treats configured seeds as addresses, never trust anchors. Join proves
enrollment and transport identity before proposing membership. WP-04.05 applies
authenticated join and leave events in deterministic order under a monotonic
membership epoch. Duplicate and out-of-order events are idempotent or rejected.

Membership state has a configured maximum size and a durable committed epoch.
Restart restores only a verified committed snapshot. A stale seed, silent node,
or network route cannot add, remove, or promote a member.

## Failure Detection And Partition Semantics

WP-04.06 classifies peers as `healthy`, `suspect`, `unavailable`,
`partitioned`, or `recovered` using bounded timers and authenticated evidence.
Failure detection never grants authority. Flapping is rate-limited and retains
the last committed membership and lease state.

During a partition, a node without a current verifiable lease cannot execute as
owner. A current holder may continue only within its lease bounds. Conflicting
epochs, unreachable fencing evidence, or uncertainty about the current holder
forces both candidates into a non-authoritative state until recovery proves one
valid owner.

## Epochs, Leases, And Fencing

WP-04.07 maintains a durable monotonic epoch per lineage. A lease records the
lineage, holder identity, epoch, issued time, expiry, policy bounds, and signing
authority. Renewal cannot change the holder and cannot revive an expired or
fenced generation.

WP-04.08 issues a fencing token from the durable epoch. Every state-changing
operation validates the current token. Stale lease holders, cloned state,
wrong-owner requests, and lower epochs are denied even if their process and
transport remain healthy. Recovery increments the epoch before a replacement
owner is activated.

## Advertisements And Placement

WP-04.09 and WP-04.10 publish signed, bounded, expiring capability and
resource-weather advertisements. Inputs include issuer identity, generation,
measurement time, expiry, schema version, and redacted values. Missing or stale
data is explicitly unavailable. Advertisements are evidence and cannot carry a
lease, fencing token, or placement command.

WP-04.11 makes deterministic placement decisions from committed membership,
valid fencing state, policy bounds, fresh advertisements, capacity limits, and
a stable tie-break order. A fenced node, stale input, wrong trust domain, or no
eligible target yields no placement. Cognition may request work but cannot
select or activate an owner outside this policy.

## Snapshot And Migration Protocol

WP-04.12 signs snapshot catalog entries and transfer manifests. Each binds the
lineage, source owner, source epoch, snapshot schema, content digest, byte
length, chunk digests, creation time, expiry, encryption context, and intended
target. Private state is never advertised in topology or logs. Corrupt,
incomplete, expired, replayed, or unauthorized transfers are rejected before
restore.

WP-04.13 owns the only relocation state machine:

`prepare -> quiesce -> checkpoint -> transfer -> validate -> fence -> activate -> commit`

- `prepare` selects an eligible target without transferring authority.
- `quiesce` stops new source work while retaining source authority.
- `checkpoint` creates a signed content-bound snapshot.
- `transfer` sends bounded authenticated chunks.
- `validate` restores in isolation and proves identity, schema, and digest.
- `fence` advances the durable epoch and disables source mutation.
- `activate` starts the target with the new lease and fencing token.
- `commit` records the sole owner and makes cleanup eligible.

Each transition is idempotent and durably audited. A target never becomes ready
before validation and fencing. A source is never deleted by this protocol.

## Rollback And Recovery

WP-04.14 applies failure-stage-specific recovery:

| Failure point | Safe outcome |
| --- | --- |
| Before `fence` | Abort target work and resume the validated source owner |
| During transfer or validation | Delete incomplete target material and retain source authority |
| After `fence`, before `activate` | Keep source fenced; recover target or explicitly issue a newer source lease |
| After `activate`, before `commit` | Verify the active token; fence ambiguity before selecting one owner |
| After `commit` | Treat the committed target as owner; old source remains fenced |

Relocation failure and rollback failure never enable both sides. If durable
records cannot establish one owner, both remain fenced and operator recovery is
required.

## Projection And Observability

WP-04.15 exposes a versioned, authenticated, redacted projection of node and
Guardian identity, membership epoch, certificate health, peer state, current
lease and fencing generation, advertisement freshness, placement decision,
migration phase, and last failure reason. Public or low-privilege views omit
keys, tokens, private state, exact resource details, and sensitive topology.

Every authority transition emits a correlation ID, lineage, actor identity,
old and new epoch, reason code, and durable record reference through the Runtime
API/WSS and tracing/Vector path. Logs are diagnostic evidence only. Readiness
requires verified local identity, certificate validity, restored durable state,
and no ambiguous ownership.

## Child Ownership And Integration

The exact WP-04.01 through WP-04.16 ledger in
`.csdlc/prepared/issues/5821/design.md` is normative for implementation scope.
Each child owns its declared module and proving target. WP-04.16 alone owns
`adl-runtime/src/distributed/mod.rs`, `adl-runtime/src/lib.rs`, production
registration, integrated adversarial behavior, and native-platform receipts.
WP-04-IMP issue #5862 coordinates dependency order but owns no product path.

## Compatibility And Rollback

Distributed mode is additive and disabled until configured with valid trust
material. Rolling back WP-04 removes distributed registration, expires remote
leases, fences remote ownership, and proves WP-03 single-node health from the
same durable lineage. It does not weaken TLS, erase audit evidence, invent a
new identity, or fall back to Runtime v2.

## Non-Claims

- This gate does not prove multi-node behavior or native portability.
- It does not make the network, Observatory, or cognition a polis authority.
- It does not define WP-14 ACIP/A2A schemas or WP-17 cross-polis identity policy.
- It does not authorize custom cryptography, custom framing, plaintext, or
  certificate-verification bypasses.
