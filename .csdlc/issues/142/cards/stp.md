# Structured Task Prompt

Template: 1.0.0

Issue: 142

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Close the Sprint 3 production-integration gap through real three-voter Guardian/kernel wiring, a configurable non-voting shepherd, one movable quorum-leased polis Observatory, three smaller local Wuji models, private self-hosted Wuji/AWS models, and two strictly serial live phases culminating in AWS continuity after a live Wuji partition, without redesigning the merged authority modules or performing async closeout.

## Deliverables

- adl-runtime/Cargo.toml
- adl-runtime/Cargo.lock
- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/continuity.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/control.rs
- adl-runtime-kernel/tests/continuity.rs
- adl-runtime-kernel/tests/observatory.rs
- adl-runtime-kernel/tests/openapi_contract.rs
- infra/runtime-v3/runtime-init.toml
- infra/runtime-v3/distributed-runtime-init.example.toml
- adl-runtime/src/guardian.rs
- adl-runtime/src/runtime_api.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/tests/distributed_runtime_operational.rs
- adl/tools/adl_ollama_shepherd_runner.py
- adl/tools/run_v092_distributed_runtime_observatory_demo.sh
- .csdlc/prepared/issues/5862/proof-receipt-contract.rb
- .csdlc/prepared/issues/53/test-proof-receipt-contract.rb
- .csdlc/prepared/issues/142/validate-proof-receipt.rb
- docs/api/runtime-v3/v1/distributed.openapi.json
- docs/api/runtime-v3/v1/observatory.openapi.json
- docs/demo/runtime-v3/DISTRIBUTED_RUNTIME_OBSERVATORY_RUNBOOK.md

## Acceptance

1. Phase A starts exactly three real Runtime v3 Guardian/kernel voters on Wuji and they converge in one authenticated trust domain.
2. Exactly one Phase A polis Observatory is shown live and identifies all three voters from one coherent redacted authority cut.
3. Phase A proves authenticated traffic, a governed distributed action, one-voter loss with retained quorum, recovery or truthful fail-closed behavior, and deterministic shutdown.
4. Phase B cannot start until every Phase A process, Observatory, port, lock, state lease, temporary credential, and test root is released.
5. The runtime config names an explicit shepherd_agent_ref and governed agent/model/provider policy; the Wuji and AWS shepherds have distinct non-exportable identities, launch with the polis, remain excluded from the voter set, and cannot mint or bypass distributed authority; the AWS standby identity is admitted by a governed mutation while the healthy three-voter hybrid polis still has quorum, before snapshot creation or failure injection, and premature pre-fence activation or Observatory binding fails closed.
6. Phase A may use three smaller local models that fit Wuji concurrently, but exact model identities, weight digests, resource/context bounds, health, and failure isolation are proven and model capability never changes authority semantics.
7. After verifying profile agent-logic-admin resolves to the approved business account, Phase B starts exactly one Wuji voter and two independently stateful AWS voters in distinct Availability Zones.
8. All three Phase B voters run pinned bounded private self-hosted local-model services through the supported ollama_http contract or a separately reviewed compatible local provider; the Wuji voter and both AZ-separated AWS voters prove model identity, health, inference and restart, while AWS also proves independent artifacts, volumes, interfaces and roles, hard CPU/memory isolation from consensus, bounded cold start, starvation safety, no hosted-model dependency, and full model-cache teardown.
9. While all three hybrid voters are healthy, the polis commits the governed AWS shepherd-admission mutation, then Wuji proposes an exact snapshot boundary that the quorum commits; both AWS log indices must equal that committed index and all three voters must independently materialize the same canonical snapshot digest through the native snapshot/catalog path; each AWS voter serially compacts the pre-boundary log and restarts from the snapshot while the other two retain quorum, and manual copy or raw-log replay cannot satisfy proof.
10. During an asymmetric partition where Wuji remains alive but the AWS pair communicates, one AWS voter wins a new consensus term with explicit votes from both AWS voters before the strict majority durably records the safety window and Wuji fence, activates exactly one AWS owner, and completes a governed mutation without selecting an uncommitted local history.
11. After a truthful bounded interruption, the quorum-enforced Wuji Observatory lease TTL expires within the bounded safety window; only then may the AWS shepherd durably activate after verifying the committed Wuji fence/new epoch and acquire the replacement Observatory ownership lease before binding exactly one AWS-hosted polis Observatory; it is resource-isolated beside the AWS voter designated to survive the later opposite-AZ voter-loss test and uses only that host's private loopback model endpoint.
12. The asymmetric-partition harness has no route or simultaneous data-plane connections that can bridge Wuji and AWS and inspects Wuji only through a Wuji-local Unix-domain socket or isolated loopback; when the partition heals without restarting Wuji, the still-running stale voter is rejected from mutation authority, renewal, Observatory ownership and stale reads, demotes on the higher AWS epoch, and synchronizes only through an explicit governed rejoin or recovery transition.
13. After healed-partition Wuji demotion and synchronization are proven, Wuji is re-partitioned and the opposite-AZ AWS voter is stopped; the single remaining reachable AWS voter is one-of-three and must halt new mutation while the colocated shepherd remains alive and reports the halt.
14. Phase B terminates every ephemeral AWS compute, volume, model cache, network attachment, SSM session, credential, process, Observatory, port, lock, and test root, with machine-verifiable cleanup evidence.
15. Retained proof binds exact source, argv, config, shepherd/model identities, node/process identities, nonzero tests, consensus snapshot/committed-prefix provenance, Observatory lease and captures, serial phase transitions, AWS account/AZ identity, and cleanup predicates without secrets or private paths.
16. Gemini review findings are resolved and a later independent exact-head implementation review has no unresolved actionable findings; the PR is not merged until the operator is shown both serial live demonstrations.
17. The implementation proves the original live #5878 production contract end to end through real Guardian and kernel processes, authenticated API and WSS, live partition, quorum fencing, migration, recovery, continued mutation, Observatory movement, and bounded shutdown; library registration or in-process module tests alone cannot satisfy this criterion.
18. The retained proof contract validates from the eventual squash-merged main topology without falsely requiring unpublished branch commits to remain ancestors, while still binding the reviewed source tree, immutable evidence tree, merge result, and exact live receipts.
19. The proof validator rejects every protected production, test, runner, API, config, and proof-contract change after the proving revision unless a new exact source revision and evidence receipt are produced and independently reviewed.

## Dependencies

- Merged Sprint 3 distributed Runtime and corrective authority implementations on main through merge d3a0d69a4c1507eb038392741d163d8341bd95d1.
- The independent Sprint 3 P1 production-integration finding that requires real production entrypoint wiring rather than module registration alone.
- Supported private ollama_http local-provider contract with pinned model artifacts available on Wuji and ephemeral AWS hosts.
- Verified Agent Logic business AWS account through profile agent-logic-admin, two Availability Zones, SSM/private-connectivity authority, and ephemeral compute/storage/network capacity.
- Operator availability to observe the single live Observatory in each serial phase and to authorize merge after both demonstrations.
- Separate umbrella reconciliation and asynchronous lifecycle closeout do not block implementation unless they change Runtime safety or ancestry.

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

- Parallel execution of the Wuji-only and Wuji-AWS demonstrations.
- One Observatory per node or multiple simultaneous Observatory owners for one polis.
- Independent singleton processes presented as a distributed polis.
- Using a shepherd or model server as a voter, authority oracle, or bypass around quorum decisions.
- Hosted-model fallback, public model endpoints, public unauthenticated APIs, or plaintext transport.
- Manual snapshot copying, selecting uncommitted local history, or restarting Wuji to simulate partition recovery.
- Kubernetes, permanent AWS infrastructure, multi-region scaling, customer traffic, umbrella reconciliation, or asynchronous lifecycle closeout.
