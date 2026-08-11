# Structured Task Prompt

Template: 1.0.0

Issue: 5867

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement deterministic membership epochs and bounded topology convergence from authenticated join events.

## Deliverables

- adl-runtime/src/distributed/membership.rs
- adl-runtime/tests/distributed_membership.rs
- Focused positive and fail-closed negative tests
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

- WP-04.04 issue #5866
- WP-04-IMP issue 5862
- Architecture/security gate issue 5821 terminal

## Inputs

- docs/architecture/runtime-v3/DISTRIBUTED_GUARDIAN_ARCHITECTURE.md
- .csdlc/prepared/issues/5867/design.md
- adl-runtime/src/distributed/identity.rs
- adl-runtime/src/distributed/certificates.rs
- adl-runtime/src/distributed/transport.rs
- adl-runtime/src/distributed/discovery.rs

## Non Goals

- Sibling WP-04 paths
- Runtime v2 fallback
- Custom cryptography or plaintext
- WP-14, consumer UI, or v0.93 work
- Self-attested completion
