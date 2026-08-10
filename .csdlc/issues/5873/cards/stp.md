# Structured Task Prompt

Template: 1.0.0

Issue: 5873

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement deterministic bounded placement from membership, fencing, capability, and resource-weather inputs.

## Deliverables

- adl-runtime/src/distributed/placement.rs
- adl-runtime/tests/distributed_placement.rs
- Focused deterministic positive and fail-closed negative tests
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

- WP-04.05 membership issue #5867 closed through merged PR #102 and merge revision ancestral
- WP-04.06 failure-detection issue #5868 closed through merged PR #106 and merge revision ancestral
- WP-04.07 lease issue #5869 closed through merged PR #107, with corrective issue #5909 PR #120 merged and its exact merge revision ancestral before execution
- WP-04.08 fencing issue #5870 closed through a merged PR whose exact merge revision is ancestral before execution
- WP-04.09 capability issue #5871 closed through merged PR #89 and merge revision ancestral
- WP-04.10 resource-weather issue #5872 closed through merged PR #93 and merge revision ancestral
- WP-04-IMP umbrella issue #5862 as coordination parent, not a terminal child gate
- Architecture/security gate issue #5821 terminal

## Inputs

- docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md
- .csdlc/prepared/issues/5821/design.md
- adl-runtime/src/distributed/membership.rs
- adl-runtime/src/distributed/failure_detection.rs
- adl-runtime/src/distributed/lease.rs
- adl-runtime/src/distributed/capability_advertisement.rs
- adl-runtime/src/distributed/resource_weather.rs
- adl-runtime/src/distributed/fencing.rs

## Non Goals

- Sibling WP-04 paths
- Runtime v2 fallback
- Custom cryptography or plaintext
- WP-14, consumer UI, or v0.93 work
- Self-attested completion
