# Distributed Guardian Threat Model

Status: Candidate security gate for independent v0.92 WP-04 review

## Scope And Assumptions

This model covers the Runtime v3 distributed Guardian design in
`docs/architecture/runtime-v3/DISTRIBUTED_GUARDIAN_ARCHITECTURE.md`. It assumes
WP-03 provides one authenticated, restartable single-node Guardian and kernel;
operators control enrollment roots; maintained QUIC/TLS and cryptographic
libraries are correctly integrated; and hosts may fail, partition, restart, or
be compromised independently.

The model excludes Runtime v2, v0.93 constitutional governance, cloud control
planes, user-facing application policy, and physical extraction resistance.
Architecture review cannot prove implementation or deployment security.

## Security Objectives

1. Preserve at most one authoritative owner for each Runtime lineage.
2. Authenticate every node, Guardian, transport peer, and authority message.
3. Prevent stale, cloned, replayed, expired, or wrong-domain state from gaining
   membership, lease, fencing, migration, or activation authority.
4. Preserve continuity and recoverability without exposing private state.
5. Keep failures bounded, observable, redacted, and fail-closed.

## Assets

- node and Guardian private keys, trust roots, and certificate generations;
- durable node identity and Runtime lineage identity;
- membership epochs, leases, fencing tokens, and owner records;
- snapshots, transfer manifests, continuity state, and migration journals;
- capability and resource-weather signing keys and observations;
- authenticated control/API/WSS channels and operator authorization;
- audit events, reason codes, correlation identities, and retained proof.

## Trust Boundaries And Entry Points

| Boundary | Entry point | Required controls |
| --- | --- | --- |
| Operator to enrollment service | Enrollment request | Approved root, proof of possession, nonce, expiry, explicit trust domain |
| Guardian to Guardian | QUIC/TLS session | Mutual TLS, purpose-bound certificate, peer identity, version and size bounds |
| Peer to membership | Join/leave event | Enrolled identity, deterministic epoch ordering, replay rejection |
| Peer to authority state | Lease/fence request | Current lineage, holder, epoch, expiry, policy authorization |
| Node to placement | Signed advertisements | Signature, freshness, bounds, redaction, evidence-only semantics |
| Source to target | Snapshot transfer | Authenticated target, signed manifest, chunk/content digests, encryption context |
| Operator/client to projection | HTTPS/WSS API | Existing Runtime authentication, authorization, versioning, redaction |
| Durable state to restarted process | Restore | Digest/schema checks, identity/trust match, monotonic epoch verification |

## Attacker Capabilities

Consider an attacker who can observe, delay, duplicate, reorder, or drop network
traffic; present stale valid messages; operate an enrolled node; compromise
fewer than a majority of current authority-ledger voters; copy a state
directory; exhaust bounded connections or transfer capacity; and
cause process, host, storage, or network failure. A host compromise may expose
that host's active purpose-bound keys.

The attacker is not assumed to break maintained cryptography, control an
operator enrollment root, or control a majority of the effective OpenRaft
voter configuration. A distributed authority group has at least three voters
and changes voters through joint membership. Every mutation sink verifies
distinct purpose-bound majority endorsements over one canonical committed
entry, so a malicious leader or minority cannot fabricate authority. Loss of a
majority halts new authority rather than selecting a local history. If those
assumptions fail, operator-led trust-domain generation recovery is required.

## Threats And Required Mitigations

### T1: Unauthorized or wrong-domain enrollment

- **Abuse path:** A node presents a self-signed identity, stolen request, or
  valid identity from the wrong trust domain.
- **Impact:** Unauthorized membership and a path toward Runtime ownership.
- **Priority:** High; the boundary is pre-membership and authority-adjacent.
- **Mitigations:** Operator-approved root, purpose-bound proof of possession,
  one-time nonce, expiry, trust-domain binding, durable generation, and audit.
  Wrong node or wrong trust domain is rejected before membership mutation.

### T2: Replay and stale lease activation

- **Abuse path:** Recorded enrollment, membership, advertisement, lease, or
  migration messages are replayed after expiry or epoch advancement.
- **Impact:** Stale authority, incorrect placement, or split-brain activation.
- **Priority:** Critical because integrity and continuity can both fail.
- **Mitigations:** Nonces, expiries, monotonic epochs, holder binding, durable
  replay state, current fencing-token checks on every mutation, and idempotent
  transition identifiers.

### T3: Partition-induced split brain

- **Abuse path:** A network partition leaves two Guardians believing they own
  one lineage, or failure detection promotes a replacement without fencing.
- **Impact:** Divergent cognition and durable state corruption.
- **Priority:** Critical.
- **Mitigations:** Failure detection is non-authoritative; OpenRaft commits
  authority through a stable majority or, during joint membership, a majority
  of both the old and new voter sets; a union majority missing either
  constituent majority is rejected. Each endorsement signs its signer identity
  and certificate generation, and enrollment plus verification reject duplicate
  effective control public keys, so one key cannot count as multiple voters.
  Leases are bounded;
  replacements use a newer majority-committed epoch only after the prior lease
  safety window; stale holders fail fencing checks; and quorum or clock
  uncertainty halts mutation. Availability never overrides one-owner proof.

### T4: Cloned state and identity collision

- **Abuse path:** An attacker or operator copies durable state and keys to a new
  host, then starts both copies.
- **Impact:** Duplicate Guardian identity and conflicting owner claims.
- **Priority:** Critical.
- **Mitigations:** Bind durable node identity, Guardian identity, trust domain,
  certificate generation, lineage, and fencing epoch. Each Guardian start also
  creates a non-persistent activation key. Quorum-issued leases bind its public
  key and every mutation proves possession. Cloned durable state lacks that
  private key; a second activation cannot renew and must wait for a newer
  committed epoch and the prior lease safety window. Collision detection still
  revokes and fences the copied identity.

### T5: Certificate compromise or certificate expiry

- **Abuse path:** A purpose key is stolen, a certificate expires mid-operation,
  or one purpose is reused to sign another class of message.
- **Impact:** Peer impersonation, forged evidence, or unavailable transport.
- **Priority:** High.
- **Mitigations:** Separate node, control, transport, advertisement, and
  snapshot purposes; bounded rotation overlap; revocation; expiry checks before
  session and renewal; bounded session lifetime; active session closure on
  revocation/generation updates; per-authority-operation certificate checks;
  generation tracking; and no verification bypass. Certificate compromise
  fences the identity and requires re-enrollment.

### T6: Transport downgrade, malformed input, or resource exhaustion

- **Abuse path:** A peer requests plaintext, an unknown protocol, oversized
  frames, unbounded streams, or repeated expensive handshakes and transfers.
- **Impact:** Authentication loss or targeted denial of service.
- **Priority:** High for availability, critical if downgrade succeeds.
- **Mitigations:** Maintained QUIC/TLS only, mutual authentication, fixed
  versions, library framing, input and stream limits, handshake/idle timeouts,
  cancellation, backpressure, per-peer quotas, and redacted reason codes.
  Authority certificates accept only the frozen Ed25519 algorithm, key and
  signature lengths, domain separator, SHA-256 digest, and deterministic prost
  encoding; unknown fields, duplicate fields, non-minimal varints, unsorted
  repeated fields, and alternative signature suites fail closed.

### T7: Forged capability or resource-weather evidence

- **Abuse path:** A node inflates capability, suppresses pressure, replays fresh
  observations, or injects sensitive values.
- **Impact:** Unsafe placement, denial of service, or information disclosure.
- **Priority:** Medium to high.
- **Mitigations:** Purpose-bound signatures, issuer generation, freshness,
  schema and value bounds, replay rejection, redaction, deterministic no-data
  policy, and exclusion of fenced nodes. Advertisements never grant authority.

### T8: Snapshot substitution or disclosure

- **Abuse path:** A target receives corrupt chunks, a manifest from another
  lineage, an expired transfer, or private state intended for another node.
- **Impact:** State compromise, continuity loss, or data exposure.
- **Priority:** High.
- **Mitigations:** Signed purpose-bound catalog and manifest, lineage/source/
  target/epoch binding, whole-content and chunk digests, length and expiry,
  authenticated encrypted transport, isolated restore, and deletion of
  incomplete target material.

### T9: Relocation failure

- **Abuse path:** Source, target, transport, storage, or validation fails at any
  migration phase.
- **Impact:** Lost availability, lost state, or two active owners.
- **Priority:** Critical.
- **Mitigations:** Durable idempotent phase journal; source authority retained
  through validation; target activation only after fence; explicit timeout and
  cancellation; and failure-stage recovery preserving snapshots and audit.

### T10: Rollback failure or ambiguous commit

- **Abuse path:** A crash occurs after fencing or activation but before commit,
  or both sides report inconsistent durable state.
- **Impact:** Permanent outage or split brain.
- **Priority:** Critical.
- **Mitigations:** Verify the majority-committed `openraft` authority-ledger
  epoch, log index, activation incarnation, and current fencing token at every
  mutation sink. `AuthorityCertificateV1` must contain distinct valid
  purpose-bound signatures satisfying the committed membership quorum: a
  strict majority in a stable configuration, or strict majorities of both the
  old and new voter sets during joint membership. A union majority missing
  either constituent majority, a leader assertion, or a minority cannot renew
  or replace authority.
  Never un-fence both sides; wait the old lease deadline plus clock/message
  uncertainty before replacement activation; and require operator action when
  a majority cannot establish one owner. Rollback failure leaves both
  candidates fenced rather than guessing.

### T11: Projection, log, or audit leakage and poisoning

- **Abuse path:** Low-privilege clients obtain topology, keys, tokens, private
  state, or exact resource data; a compromised peer injects misleading labels.
- **Impact:** Confidentiality loss and impaired incident response.
- **Priority:** Medium.
- **Mitigations:** Versioned authorization, field-level redaction, bounded
  labels, authenticated actor identity, correlation IDs, immutable reason codes,
  and separation between diagnostic evidence and authority state.

## Denial And Recovery Matrix

| Condition | Required safe behavior |
| --- | --- |
| Unknown, expired, revoked, or wrong-purpose certificate | Reject session or message; do not fall back |
| Replayed message or stale epoch | Reject and audit without state mutation |
| Suspected peer or partition | Preserve current bounded lease; do not promote from silence |
| Conflicting owner evidence | Fence both candidates pending authoritative recovery |
| Invalid advertisement | Treat input as unavailable and exclude it from placement |
| Corrupt or incomplete snapshot | Reject restore and retain source authority |
| Relocation failure before fence | Abort target and resume source |
| Relocation or rollback failure after fence | Keep ambiguity fenced; recover using a newer durable epoch |
| Certificate renewal failure | Stop renewal-dependent authority before expiry |
| Logging or projection failure | Preserve authority state; report bounded observability degradation |

## Verification Responsibilities

WP-04.01 through WP-04.15 each prove their positive and adversarial contract in
the exact owned test target. WP-04.16 proves integrated real multi-node
membership, partition, stale lease and cloned-state fencing, certificate
rotation/revocation/expiry, migration, relocation failure, rollback failure,
API/WSS continuity, shutdown, and native macOS/Linux/Windows receipts.

Security acceptance requires exact-head independent review, no unresolved
actionable findings, and retained artifact digests. Fixtures and prose do not
satisfy runtime proof.

## Residual Risk

- Host compromise can expose active keys until revocation and fencing propagate.
- Severe clock error can reduce availability; it cannot extend an expired lease.
- Loss of an authority-ledger majority halts new mutation and relocation until
  quorum recovery; this is an intentional consistency-over-availability choice.
- Simultaneous durable-store loss may require operator recovery from retained
  evidence and can leave the lineage unavailable.
- Library vulnerabilities remain supply-chain risk and require pinned,
  maintained dependency review and upgrade policy.
- Compromise of a majority of current authority-ledger voters defeats automated
  ownership safety and requires trust-domain reconstruction from retained audit
  and continuity evidence.
- A future cross-polis identity policy may add trust relationships; this gate
  deliberately does not authorize them.
