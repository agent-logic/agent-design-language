# Structured Task Prompt

Template: 1.0.0

Issue: 142

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Close the Sprint 3 production-integration gap through real three-voter Guardian/kernel wiring, a configurable non-voting shepherd, one movable quorum-leased polis Observatory, three smaller local Wuji models, private self-hosted Wuji/AWS models, and two strictly serial live phases culminating in AWS continuity after a live Wuji partition, without redesigning the merged authority modules or performing async closeout.

## Deliverables

- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime/src/guardian.rs
- adl-runtime/src/runtime_api.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/tests/distributed_runtime_operational.rs
- adl/tools/run_v092_distributed_runtime_observatory_demo.sh
- .csdlc/prepared/issues/142/validate-proof-receipt.rb
- docs/api/runtime-v3/v1/distributed.openapi.json
- docs/demo/runtime-v3/DISTRIBUTED_RUNTIME_OBSERVATORY_RUNBOOK.md

## Acceptance

1. AC-1: two real Wuji Guardian/kernel nodes converge in one authenticated polis
2. AC-2: one live Phase A polis Observatory identifies both nodes and shows one coherent redacted authority cut
3. AC-3: Phase A proves authenticated traffic, governed action, degradation, recovery or fail-closed behavior, and full teardown
4. AC-4: Phase B cannot start until Phase A processes, Observatory, ports, locks, state, and credentials are released
5. AC-5: the AWS identity is verified through agent-logic-admin before one ephemeral AWS node is launched
6. AC-6: one live Phase B polis Observatory identifies the Wuji and AWS nodes through private authenticated access
7. AC-7: Phase B proves cross-host action, degradation, recovery or fail-closed behavior, and full local/AWS teardown
8. AC-8: proof binds exact source, argv, nonzero process/test denominators, captures, cleanup predicates, and redacted artifacts
9. AC-9: a fresh independent exact-head review has no unresolved actionable findings before publication
10. AC-10: the implementation PR is not merged before both serial live Observatory demonstrations are shown to the operator

## Dependencies

- Merged WP-04.01 through WP-04.16 code and corrective authority repairs
- Independent Sprint 3 P1 production-integration finding
- Verified Agent Logic AWS business account and bounded SSM/private-connectivity permissions
- Separate umbrella reconciliation and async closeout work does not block implementation unless it changes runtime safety

## Inputs

- agent-logic/agent-design-language#142
- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime/src/guardian.rs
- adl-runtime/src/runtime_api.rs
- adl-runtime/src/distributed/
- adl-runtime/tests/distributed_guardian.rs
- docs/api/runtime-v3/v1/distributed.openapi.json
- docs/architecture/runtime-v3/DISTRIBUTED_GUARDIAN_ARCHITECTURE.md
- docs/security/runtime-v3/DISTRIBUTED_GUARDIAN_THREAT_MODEL.md

## Non Goals

- Parallel execution of the two demos
- One Observatory per runtime node
- Independent single-node processes presented as a polis
- Public unauthenticated or plaintext endpoints
- Kubernetes, permanent AWS infrastructure, multi-region scaling, or customer traffic
- Umbrella reconciliation or async lifecycle closeout
