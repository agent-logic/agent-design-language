# Structured Task Prompt

Template: 1.0.0

Issue: 5874

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement authenticated snapshot catalog entries and content-bound transfer manifests without exposing private state.

## Deliverables

- adl-runtime/src/distributed/snapshot_catalog.rs
- adl-runtime/tests/distributed_snapshot_catalog.rs
- Focused authenticated positive and fail-closed negative tests
- Digest-bound exact-revision execution proof
- Independent exact-head review and rollback evidence

## Acceptance

1. Implement only the declared exclusive paths
2. Preserve Guardian, authentication, bounds, determinism, durability, and redaction invariants
3. Run the exact named test with nonzero test enforcement
4. Prove applicable stale, replay, malformed, unauthorized, failure, and recovery cases
5. Bind all evidence to the exact source revision and artifact digests
6. Complete independent review and child-owned typed closeout

## Dependencies

- WP-04.02 certificate issue #5864 closed through merged PR #62 and merge revision ancestral
- Corrective lease issue #5909 PR #120 merged and its exact merge revision ancestral before fencing or snapshot execution
- WP-04.08 fencing issue #5870 closed through a merged PR whose exact merge revision is ancestral before snapshot execution
- WP-04-IMP umbrella issue #5862 as coordination parent, not a terminal child gate
- Architecture/security gate issue #5821 terminal

## Inputs

- docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md
- .csdlc/prepared/issues/5821/design.md
- adl-runtime/src/distributed/certificates.rs
- adl-runtime/src/distributed/fencing.rs

## Non Goals

- Sibling WP-04 paths
- Runtime v2 fallback
- Custom cryptography or plaintext
- WP-14, consumer UI, or v0.93 work
- Self-attested completion
