# Distributed Guardian Architecture

Status: Candidate design gate for independent v0.92 WP-04 review

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
   domain. Authority also requires a fresh activation incarnation whose private
   key is generated at process start and is absent from copied durable state.
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

Established sessions have a maximum lifetime no later than the earliest peer
certificate expiry or configured revocation-refresh deadline. A revocation or
generation update actively closes affected sessions. Every authority-bearing
operation revalidates the peer certificate purpose, generation, revocation
state, and expiry; a transport handshake is not sufficient authorization for
the lifetime of a QUIC connection.

## Maintained QUIC/TLS Transport

WP-04.03 uses `quinn` as the maintained QUIC implementation, its supported
`rustls` integration for TLS, and the existing `prost` stack for length-delimited
protobuf application messages. The Cargo manifest and lockfile pin exact
reviewed versions. Upgrades require maintained-release, advisory, license,
lockfile, and interop review; an abandoned dependency blocks release until a
reviewed replacement preserves this contract. QUIC stream framing, TLS,
cryptography, chain validation, flow control, cancellation, and idle timeouts
remain library-owned. ADL defines only bounded typed protobuf messages and does
not add custom cryptography or custom wire framing.

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

WP-04.07 maintains the authoritative per-lineage record in a majority-replicated
Guardian authority ledger implemented with the maintained `openraft` crate.
Only enrolled voting Guardians in the same trust domain participate. A
distributed authority group has at least three voters; joining nodes begin as
non-voting learners, and voter changes use OpenRaft's joint membership
transition. A majority of the effective voter configuration is the only
quorum. For a stable configuration, quorum means a strict majority of that
configuration. For a joint configuration, quorum means a strict majority of
both the committed old voter set and the committed new voter set; a strict
majority of their union is insufficient if either constituent majority is
missing. The committed log serializes membership-voter changes, epochs, lease
grants, activation incarnations, fences, certificate revocations, and owner
commits. The linearization point is the applied majority-committed log index.
A node outside a majority cannot advance the log, renew authority, or activate
a replacement. WP-03 single-node operation remains outside distributed mode
and cannot grant remote authority.

A lease records the lineage, holder identity, activation-incarnation public
key, leader term, committed log index, epoch, certificate generation, issued
time, expiry, policy bounds, and an `AuthorityCertificateV1`. Local durability,
a higher uncommitted term or epoch, transport reachability, and failure-detector
output are not authority. The activation private key is generated at Guardian
start and is never written into the state directory. Renewal is a new committed
log entry, proves possession of the same activation key, and cannot change
holder or incarnation. A clone therefore cannot renew a copied lease; a second
activation requires a newer committed epoch after the prior safety window.

`AuthorityCertificateV1` is the following frozen protobuf v3 wire contract;
field numbers, scalar types, and nesting are part of version 1 and may not be
reinterpreted:

```proto
syntax = "proto3";

message AuthorityCertificateBodyV1 {
  uint32 schema_version = 1;           // MUST equal 1
  bytes trust_domain_id = 2;
  bytes lineage_id = 3;
  uint64 voter_set_generation = 4;
  uint64 raft_term = 5;
  uint64 committed_log_index = 6;
  uint64 epoch = 7;
  bytes holder_node_id = 8;
  bytes holder_guardian_id = 9;
  bytes activation_key_sha256 = 10;    // exactly 32 bytes
  uint32 operation_class = 11;
  int64 issued_unix_seconds = 12;
  uint32 issued_nanos = 13;            // 0..999999999
  uint64 lease_duration_millis = 14;
  bytes policy_sha256 = 15;            // exactly 32 bytes
  uint32 signing_algorithm = 16;       // 1 = Ed25519
}

message AuthorityEndorsementV1 {
  bytes signer_guardian_id = 1;
  uint64 certificate_generation = 2;
  uint32 signing_algorithm = 3;        // 1 = Ed25519
  bytes signature = 4;                 // exactly 64 bytes, R || S
}

message AuthorityCertificateV1 {
  AuthorityCertificateBodyV1 body = 1;
  repeated AuthorityEndorsementV1 endorsements = 2;
}
```

`operation_class` is closed: `1=lease_grant`, `2=lease_renewal`, `3=fence`,
`4=activate`, `5=owner_commit`, and `6=revoke`; zero and every unknown value
are rejected. Identity fields contain the exact non-empty canonical identity
bytes from their owning durable records and receive no Unicode, case, or other
normalization. Endorsements are unique by `signer_guardian_id` and sorted by
unsigned lexicographic comparison of that byte string, with no secondary sort
key.

Version 1 uses RustCrypto `ed25519-dalek`: each
control public key is the 32-byte compressed Edwards-Y coordinate accepted by
`VerifyingKey::from_bytes`, and each signature is the 64-byte `R || S` encoding
accepted by `Signature::from_bytes`. The signing preimage is the ASCII domain
separator `ADL-AUTHORITY-CERTIFICATE-V1\0` followed by the deterministic
`prost::Message::encode_to_vec` encoding of the certificate body with the
endorsement list omitted. Version 1 forbids protobuf maps, requires fields in
declared tag order, requires repeated fields to be pre-sorted by their specified
identity order, rejects duplicate singular or signer fields, rejects unknown
fields, and rejects non-minimal varints before decoding. After those wire checks,
the verifier decodes with `prost`, re-encodes, and requires byte-for-byte equality
with the received canonical body. The signed digest is `SHA-256(preimage)`.
Verification calls `ed25519_dalek::VerifyingKey::verify_strict` on that 32-byte
digest and the parsed signature. Plain `verify`, Ed25519ph, Ed25519ctx,
non-canonical scalar or point encodings, and every other algorithm identifier,
key length, signature length, encoding, or canonicalization are rejected.

The certificate carries each distinct signer identity, certificate generation,
and Ed25519 signature. The signer identities must satisfy the exact quorum rule
of the committed membership named by the certificate: a strict majority for a
stable configuration, or a strict majority of both old and new voter sets for a
joint configuration. A union majority that lacks either constituent majority is
not authority. It uses no threshold scheme or custom cryptographic primitive.

A voter endorses only after durably applying the identical authority-ledger
entry. Voter changes use OpenRaft joint consensus, and verification uses the
voter-set generation named by the committed entry. Every mutation sink parses
the canonical message, recomputes its digest, verifies that distinct current-voter
signatures satisfy that committed membership's stable or joint quorum function,
and verifies their certificate purpose, generation,
revocation, and expiry, checks its applied log index is at least the named index,
proves activation-key possession, and enforces the operation class. A leader
value without those endorsements has no authority. Safety assumes compromised
signers cannot satisfy the committed membership's quorum function; satisfying a
stable majority, or both constituent majorities during joint membership,
requires operator trust-domain reconstruction.

Lease safety uses monotonic elapsed time, never wall-clock time alone. Voting
nodes enforce a configured maximum clock uncertainty measured through the
Chronosense boundary. A node outside that bound becomes non-authoritative. A
replacement cannot activate until the previous committed lease deadline plus
the maximum uncertainty and message-delay safety margin has elapsed. Restart
invalidates the local activation incarnation and requires quorum renewal.

WP-04.08 derives a majority-endorsed fencing token from the committed epoch,
activation incarnation, and log index. Every state-changing sink, including
durable writes, checkpoint commit, API mutation, and target activation,
validates the current certificate against its applied authority-ledger index and
monotonic deadline. A sink that cannot establish current ledger state refuses
mutation. Stale lease holders, cloned state, wrong-owner requests, and lower
epochs are denied even if their process and transport remain healthy. Recovery
commits a newer epoch before a replacement permit is issued.

Every voter restores and verifies its OpenRaft vote, log, committed membership,
snapshot, and applied state-machine index before participation. Quorum loss
halts new mutation authority. Recovery selects a history only through a
majority containing the committed prefix; if no such majority can be
established, all candidates remain fenced for operator-led trust-domain
generation recovery. The numerically highest local epoch is never selected by
itself.

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
- `fence` commits a newer authority-ledger epoch, waits the prior lease safety
  window, and causes every mutation sink to reject the source permit.
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
WP-04.03 is the sole manifest and lockfile owner and introduces the reviewed
`quinn`, `rustls`, `prost`, and `openraft` dependency set required by later
children; no sibling edits those shared files.
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
