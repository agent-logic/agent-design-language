# Issue 5869 Design: WP-04.07 Epoch and lease authority

## Outcome And Boundary

Implement OpenRaft majority-committed authority, joint membership, canonical
authority certificates, monotonic epochs, and bounded leases as prerequisites
for distributed ownership decisions. This child is one exclusive implementation slice under
WP-04-IMP issue #5862; it does not absorb sibling work or
receive completion credit from the #5821 architecture gate.

## Source Baseline

- `docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md` defines the milestone feature and claim boundary.
- `.csdlc/prepared/issues/5821/design.md` freezes the Guardian-owned architecture, threat model, dependency graph, and sixteen-child denominator.
- `adl-runtime/src/guardian.rs`, `adl-runtime/src/networking.rs`, `adl-runtime/src/topology.rs`, and `adl-runtime/src/runtime_api.rs` are current Runtime v3 integration authorities.
- `adl-runtime/tests/guardian_cli.rs` and `adl-runtime/tests/runtime_api_wss.rs` are retained launch and authenticated carrier proof inputs, not substitutes for this child's named proof.

## Owned Paths

- `adl-runtime/src/distributed/lease.rs`
- `adl-runtime/tests/distributed_lease.rs`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Design And Failure Semantics

Implement the OpenRaft authority ledger and canonical
`AuthorityCertificateV1` contract frozen by #5821. A distributed group has at
least three voters; membership changes use joint consensus; lease grant and
renewal are majority-committed entries; and each certificate binds the voter
generation, term, applied log index, epoch, holder, activation-key digest,
operation class, lifetime, and policy digest. Endorsements must satisfy the
exact committed OpenRaft quorum function: a strict majority of the stable voter
set, or strict majorities of both old and new voter sets during joint
membership. A majority of the union that lacks either constituent majority is
rejected. `AuthorityCertificateV1` uses algorithm identifier `ed25519`,
RustCrypto `ed25519-dalek`, 32-byte compressed public keys, 64-byte `R || S`
signatures, the exact `ADL-AUTHORITY-CERTIFICATE-V1\0` domain separator, and the
deterministic prost body encoding and strict unknown/duplicate/non-minimal-field
rejection rules frozen by #5821. Quorum-valid purpose-bound endorsements,
activation-key possession, certificate validity, monotonic-time
safety, and applied-index checks are mandatory. The implementation must preserve Guardian as process 0,
bounded queues and timeouts, authenticated transport, deterministic
projections, durable state authority, redaction, and fail-closed behavior.
Missing, stale, replayed, malformed, unauthorized, wrong-domain, or
resource-exhausted inputs remain explicit failures and never trigger an
insecure fallback.

## Dependencies

- WP-04.05 issue #5867
- WP-04-IMP issue #5862 coordinates ordering but owns no child product path.
- #5821 must be terminal before implementation binding.

## Proof Boundary

Exact nextest target distributed_lease proves three-voter majority and joint
membership behavior, canonical AuthorityCertificateV1 encoding and digest
binding, exact Ed25519 key/signature/domain/protobuf rejection behavior,
distinct quorum endorsements, rejection of a union majority that lacks either
the old-set or new-set majority, activation-key possession, applied
mutation-sink checks, monotonic epochs, lease renewal and expiry, certificate
revocation/expiry, quorum loss, malicious-leader/minority denial, clock
uncertainty, stale-holder denial, and restart recovery.

The execution receipt must bind the exact source revision, exact argv,
nonzero selected test count, output and artifact SHA-256 digests, runner
identity, negative cases, and native platform identity where claimed.
Hand-authored status booleans, retained fixtures, and prose do not prove
working behavior.

## Rollback

Expire issue-created leases, restore only the last majority-committed authority
state, and leave all candidates fenced when no quorum can prove that state.

## Estimate

Budget this bounded epoch-and-lease child under the typed medium profile:
6 elapsed hours, 80,000 reasoning tokens, and 60 minutes of focused validation
and review. Lease expiry, renewal, epoch monotonicity, and replay rejection are
proved inside one temporal contract;
replan before widening paths, dependencies, proof surface, or rollback authority.

## Non-Goals

- Sibling WP-04 paths, WP-14 protocol reconciliation, consumer UI work, or v0.93 governance.
- Runtime v2 fallback, custom cryptography, plaintext transport, or unbounded queues.
- Completion credit from issue creation, architecture approval, fixtures, or self-attested receipts.
