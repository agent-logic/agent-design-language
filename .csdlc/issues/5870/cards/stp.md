# Structured Task Prompt

Template: 1.0.0

Issue: 5870

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Enforce one authoritative owner per lineage and reject stale, cloned, or partitioned actors.

## Deliverables

- adl-runtime/src/distributed/fencing.rs
- adl-runtime/tests/distributed_fencing.rs
- Quorum-authorized fence and revoke operations that do not require old-holder activation possession, while every holder-authorized activation or mutation still requires current holder proof
- Strict committed next-epoch transitions with a portable durable safety floor that survives restart, restore, and rollback
- Fresh current AuthorityMembership verification and an exact operation allowlist at every authority-sensitive entry point
- Atomic immediately durable replay and fence receipts with fail-closed recovery from torn, corrupt, missing, or capacity-exhausted state
- Deterministic focused tests for path and symlink safety, restart, rollback, replay, capacity, and absent current membership
- Exact-revision proof with an issue-specific machine-derived negative-marker denominator and independent exact-head review

## Acceptance

1. Implement only the declared exclusive paths
2. Preserve Guardian, authentication, bounds, determinism, durability, and redaction invariants
3. Run the exact named test with nonzero test enforcement
4. Prove applicable stale, replay, malformed, unauthorized, failure, and recovery cases
5. Bind all evidence to the exact source revision and artifact digests
6. Complete independent review and child-owned typed closeout

## Dependencies

- #5868 failure detection must be closed by a merged commit ancestral to the exact execution base
- #5869 lease baseline must be closed by a merged commit ancestral to the exact execution base
- Corrective PR #120 for #5909 must be merged and ancestral
- Same-repository issue #121 and stacked PR #123 must be merged and ancestral after their declared base
- Both corrective merges #120 and #123 must be present before #5870 may bind or implement
- #5862 is the coordination umbrella and is not a substitute for dependency terminality
- #5821 is the terminal architecture gate

## Inputs

- docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md
- .csdlc/prepared/issues/5821/design.md
- adl-runtime/src/guardian.rs
- adl-runtime/src/networking.rs
- adl-runtime/src/runtime_api.rs
- adl-runtime/src/distributed/certificates.rs
- adl-runtime/src/distributed/membership.rs
- adl-runtime/src/distributed/failure_detection.rs
- adl-runtime/src/distributed/lease.rs

## Non Goals

- Sibling WP-04 paths
- Runtime v2 fallback
- Custom cryptography or plaintext
- WP-14, consumer UI, or v0.93 work
- Self-attested completion
