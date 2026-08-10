# Structured Task Prompt

Template: 1.0.0

Issue: 5878

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Register and integrate the distributed module, then prove real multi-node Guardian behavior, API/WSS continuity, adversarial failures, and native macOS/Linux/Windows receipts.

## Deliverables

- adl-runtime/src/distributed/mod.rs
- adl-runtime/src/lib.rs
- adl-runtime/tests/distributed_guardian.rs
- adl/tools/validate_v092_distributed_guardian.sh
- adl/tools/validate_v092_distributed_native_receipts.rb
- Register and integrate only the terminal #5863-#5877 distributed contracts while preserving authenticated Runtime API and WSS continuity
- Prove deterministic bounded multi-node behavior, partitions, fencing, migration, recovery, rollback or disable behavior, redaction, and fail-closed errors
- Retain exact nonzero integration proof and machine-verified macOS, Linux, and Windows native receipts bound to one protected source revision
- Obtain independent exact-head security and correctness review before publication

## Acceptance

1. Implement only the declared exclusive paths
2. Preserve Guardian, authentication, bounds, determinism, durability, and redaction invariants
3. Run the exact named test with nonzero test enforcement
4. Prove applicable stale, replay, malformed, unauthorized, failure, and recovery cases
5. Bind all evidence to the exact source revision and artifact digests
6. Complete independent review and child-owned typed closeout

## Dependencies

- #5909 PR #120 must merge and be ancestral before #5870 may execute
- #5863, #5864, #5865, #5866, #5867, #5868, #5869, #5871, and #5872 must each be merged, closed, and ancestral
- #5870 must be merged, closed, and ancestral after the #5909 corrective merge
- #5873 and #5874 must both be merged, closed, and ancestral after their declared #5870 and sibling gates
- #5875 must be merged, closed, and ancestral after #5873 and #5874
- #5876 must be merged, closed, and ancestral after #5875
- #5877 must be merged, closed, and ancestral after #5876
- Exactly all fifteen implementation children #5863 through #5877 plus corrective #5909 must be terminal and ancestral before #5878 may bind or implement
- #5862 is the coordination umbrella and is not a substitute for child terminality
- #5821 is the terminal architecture gate

## Inputs

- docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md
- .csdlc/prepared/issues/5821/design.md
- adl-runtime/src/guardian.rs
- adl-runtime/src/networking.rs
- adl-runtime/src/runtime_api.rs
- adl-runtime/src/distributed/identity.rs
- adl-runtime/src/distributed/certificates.rs
- adl-runtime/src/distributed/transport.rs
- adl-runtime/src/distributed/discovery.rs
- adl-runtime/src/distributed/membership.rs
- adl-runtime/src/distributed/failure_detection.rs
- adl-runtime/src/distributed/lease.rs
- adl-runtime/src/distributed/fencing.rs
- adl-runtime/src/distributed/capability_advertisement.rs
- adl-runtime/src/distributed/resource_weather.rs
- adl-runtime/src/distributed/placement.rs
- adl-runtime/src/distributed/snapshot_catalog.rs
- adl-runtime/src/distributed/migration.rs
- adl-runtime/src/distributed/recovery.rs
- adl-runtime/src/distributed/projection.rs
- docs/api/runtime-v3/v1/distributed.openapi.json

## Non Goals

- Sibling WP-04 paths
- Runtime v2 fallback
- Custom cryptography or plaintext
- WP-14, consumer UI, or v0.93 work
- Self-attested completion
