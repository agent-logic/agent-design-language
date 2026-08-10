# Issue #142 Design — Operational Distributed Runtime and Polis Observatory

## Problem

Sprint 3 merged the distributed authority components and library-level integration, but the production Guardian/kernel entrypoints do not yet operate as a real multi-node polis. The retained proof does not launch networked processes or exercise the required API/WSS, partition, fencing, migration, recovery, Observatory, and shutdown behavior.

## Outcome

Provide one production-shaped distributed Runtime path and prove it in two strictly serial three-node topologies:

1. three Runtime/Guardian voters on Wuji;
2. only after full teardown, one voter on Wuji and two independently stateful voters in the approved Agent Logic AWS account.

Each topology owns exactly one polis-level distributed Observatory. The Observatory identifies both nodes and presents one coherent, redacted authority cut. It is not duplicated per node.

## Authority and safety boundaries

- Existing WP-04 distributed authorities remain authoritative; the launcher must invoke them rather than recreate or bypass them.
- Node identities, state roots, ports, credentials, and durable authority state are distinct and bounded.
- The Observatory exposes only the governed distributed projection and redacted operational status.
- Phase B is forbidden while any Phase A process, Observatory, port, lock, state lease, or credential remains active.
- The hybrid layout places two voters in AWS so loss of Wuji preserves a strict two-of-three majority; a two-voter layout is explicitly forbidden.
- Before failure injection, Wuji proposes a snapshot boundary that the three-voter quorum commits. Each healthy voter independently materializes the same canonical snapshot from that exact committed prefix through the authoritative snapshot/catalog path; manual file transfer is forbidden and the plan does not falsely claim a Raft snapshot-transfer event for already-current followers. Both AWS voters prove catch-up to the boundary and byte-identical canonical snapshot digest, trust domain, voter generation, epoch, committed index, and restore eligibility. Each AWS voter then serially compacts the pre-boundary log and restarts while the other two voters retain quorum, proving recovery loads the snapshot root without access to the compacted prefix before the failure demonstration begins.
- Wuji is then stopped as a real process/host-loss event. The AWS majority must fence the unavailable Wuji owner, preserve or restore the committed prefix, activate one authoritative AWS owner only after the safety window, and keep the polis Observatory available in AWS.
- AWS commands use only the verified `agent-logic-admin` business profile and bounded ephemeral infrastructure.
- Remote transport is authenticated and encrypted; remote Observatory viewing uses a private authenticated path such as SSM.
- Exact live proof and cleanup evidence are required; in-process fixtures and screenshots alone are not sufficient.

## Integration shape

The implementation must connect production Guardian and kernel entrypoints to the merged identity, certificate, networking, membership, failure-detection, lease, fencing, placement, snapshot, migration, recovery, and projection authorities. A single polis Observatory/API/WSS service reads one coherent projection cut and shows both members, health, membership epoch/index, and redacted authority summaries.

The polis shepherd is part of this lifecycle. The runtime configuration must name a `shepherd_agent_ref`; the referenced governed agent configuration selects its model, provider endpoint, limits, policy, and distinct identity. The shepherd starts before member admission, owns serial orchestration and the single Observatory endpoint, and terminates only after node and resource reconciliation. It is not a voter and cannot mint authority. During the healthy hybrid phase, the quorum admits a restartable AWS shepherd standby through a governed mutation and records its newly generated non-exportable identity before snapshot creation; copying or pre-authorizing the Wuji shepherd key is forbidden. The standby is resource-isolated beside the AWS voter designated to survive the later one-of-three halt test; that test always stops the voter in the opposite Availability Zone. The standby uses only that surviving host's private loopback `ollama_http` endpoint. After Wuji partition it reconstructs orchestration and Observatory state only from the majority-committed polis record, verifies the committed Wuji fence/new epoch, durably activates, and acquires the current Observatory ownership lease before binding the endpoint.

Runtime node model selection is configuration, not consensus authority. Phase A may use three smaller local models that fit Wuji concurrently, provided their exact model identities, weights digests, context/resource bounds, and provider health are proven and model failure cannot bypass Guardian authority. Phase B uses pinned bounded self-hosted local-model servers on the Wuji voter and both AWS voters through ADL's supported private `ollama_http` provider contract (or a separately reviewed compatible local server). Each AWS node has an independently verified model artifact, EBS volume, network interface, and least-privilege instance role in its own Availability Zone; no hosted model API is required. Guardian/kernel consensus work and model inference run under separate hard CPU and memory resource controls, with an inference-pressure test proving quorum timing remains safe. The plan includes bounded model acquisition or pre-baked storage, health and identity checks, private-only endpoints, cold-start timeout, inference smoke, restart, and full compute/volume/model-cache teardown.

The operator runner is a serial state machine:

`idle -> wuji_three_running -> wuji_three_stopped -> hybrid_three_running -> aws_shepherd_admitted -> hybrid_mutation_committed -> hybrid_inference_saturated -> hybrid_snapshot_boundary_committed -> aws_consensus_caught_up -> all_voters_snapshot_materialized -> snapshot_prefix_compacted -> aws_snapshot_restart_proven -> wuji_asymmetrically_partitioned -> aws_consensus_leader_elected -> safety_window -> wuji_observatory_lease_expired -> wuji_fenced -> aws_owner_activated -> aws_shepherd_activated -> aws_observatory_leased -> aws_continuity_proven -> partition_healed_stale_wuji_demoted -> wuji_repartitioned -> aws_one_of_three_halted -> hybrid_stopped`

Every transition is durable and fail-closed. Hybrid launch is rejected from any state other than a proven local-stopped state with released resources.

## Proof

- Focused configuration and authority-boundary tests.
- Real three-process Wuji proof with one live polis Observatory.
- Full Wuji teardown proof.
- Verified business-account identity, then real one-Wuji/two-AWS proof with one securely viewed polis Observatory.
- Wuji-proposed quorum-committed snapshot boundary, byte-identical canonical snapshots independently materialized by both healthy AWS voters, asymmetric partition while Wuji stays alive, explicit expiry of the old quorum-enforced Observatory lease, AWS majority fencing/activation, a truthful bounded Observatory interruption, continued governed mutation, partition healing with stale-Wuji demotion/synchronization, and stale authority/Observatory denial proof after lease expiry.
- Serial AWS restart proof after pre-boundary log compaction, proving snapshot-root restore rather than raw historical-log replay while quorum remains available.
- An AWS voter must win a new consensus term with two explicit AWS votes before the quorum may commit the Wuji fence or activate an AWS owner.
- A bounded maximum-concurrency, context-heavy local-model inference saturation injection while the hybrid polis is healthy or AWS-owned, proving Guardian/kernel heartbeat, replication, fencing, and lease timing remain within policy under model pressure.
- The partition harness has no route that can bridge Wuji and AWS. It inspects isolated Wuji only through a Wuji-local Unix-domain socket or loopback control surface and never holds simultaneous cross-partition data-plane connections.
- A premature AWS-shepherd activation or Observatory bind before a committed Wuji fence and current ownership lease is rejected.
- Cascading-failure proof: after healed-partition demotion/synchronization is proven, re-partition Wuji, then stop the opposite-AZ AWS voter; only the designated surviving AWS voter remains reachable and one-of-three must halt new mutation while the colocated shepherd stays alive.
- Asymmetric-partition proof: Wuji remains running while only the two AWS voters can communicate; Wuji loses authority/Observatory lease and AWS must not activate until the committed fence and safety window complete.
- Config proof for `shepherd_agent_ref`, three smaller Wuji-local models, and private self-hosted AWS `ollama_http` model services with pinned artifacts and bounded teardown.
- Failure/degradation, fencing, migration/recovery, and shutdown evidence in each applicable topology.
- Exact argv and source-bound receipt validation.
- Fresh independently attributable exact-head review.

## Explicit exclusions

No parallel demos, per-node Observatories, public unauthenticated endpoints, plaintext transport, permanent AWS resources, Kubernetes, customer traffic, umbrella reconciliation, or async lifecycle closeout.
