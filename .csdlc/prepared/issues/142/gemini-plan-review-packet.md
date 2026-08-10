# Gemini Findings-First Plan Review Packet — Issue #142

## Review target

Review the pre-execution design and validation plan for `agent-logic/agent-design-language#142`, titled:

`[v0.92][runtime] Operationalize distributed Runtime and polis Observatory across Wuji and AWS`

Return findings first, ordered P0 to P3. For each finding, state the violated safety or proof contract, the exact plan element at fault, and a concrete correction. Then give a verdict: `APPROVE`, `CHANGES REQUESTED`, or `BLOCKED`. Do not assume implementation exists. Keep the entire response under 900 words; if there are no findings, say so and give `APPROVE` without restating the plan or answering the questions individually.

## Prior Gemini findings and current disposition

- Out-of-band snapshot copy: fixed; only native consensus/recovery snapshot installation is allowed.
- Observatory split-brain: fixed; AWS binding requires committed Wuji fence/new epoch plus an Observatory ownership lease.
- Missing durable transition states: fixed; safety window, fence, activation, and Observatory lease are explicit states.
- AWS failure-domain ambiguity: fixed; the AWS voters must occupy distinct Availability Zones.
- Stale Wuji Observatory denial: fixed; stale restart must reject authority, lease acquisition, and stale reads.
- Added asymmetric partition and post-takeover AWS-voter-loss negatives.
- AWS standby identity: fixed; a distinct non-exportable identity is governed-admitted by the healthy hybrid quorum before snapshot creation or failure, never copied from Wuji or pre-authorized out of band.
- Negative-test scheduling: fixed; the Phase B live lane explicitly runs asymmetric partition and post-takeover one-of-three halt.
- Inference starvation: fixed; AWS inference and consensus have hard separate CPU/memory controls plus saturation proof.
- Pre-failure state truth: fixed; governed AWS shepherd admission, hybrid mutation, snapshot commit, and exact two-AWS catch-up are distinct durable states.
- Partition semantics: fixed; Wuji remains alive but asymmetrically isolated while stale authority and Observatory denial are exercised.
- Phase B model completeness: fixed; Wuji and both AWS voters run pinned bounded private local-model servers.
- Partition continuity: fixed; Wuji is never restarted to prove stale rejection. The partition is healed while it remains alive, then the higher AWS epoch forces demotion and authoritative synchronization.
- Inference-pressure scheduling: fixed; the live hybrid lane explicitly saturates bounded local inference and verifies consensus timing remains safe.
- Partition-harness isolation: fixed; Wuji is inspected only over a Wuji-local Unix-domain socket or isolated loopback and the harness cannot bridge Wuji and AWS.
- Shepherd failure domain/model access: fixed; the standby is resource-isolated beside the AWS voter designated to survive, the opposite-AZ voter is the later failure target, and shepherd inference uses only the survivor's private loopback model endpoint.
- Durable lifecycle mapping: fixed; inference saturation and AWS shepherd activation are explicit states, and premature pre-fence shepherd activation is a required negative.
- Observatory TTL: fixed; the quorum-enforced Wuji ownership lease must expire within the bounded safety window before AWS can grant a replacement lease.
- Cascading quorum math: fixed; after healed-partition demotion is proven, Wuji is re-partitioned before the opposite-AZ AWS voter is stopped, leaving a real one-of-three reachable voter set.
- Snapshot semantics: fixed; the plan proves a committed snapshot boundary and identical canonical per-voter materialization for healthy followers rather than falsely claiming a network snapshot transfer.
- Snapshot recovery: fixed; each AWS voter serially compacts the pre-boundary log and restarts from its snapshot while the other two voters preserve quorum.
- Consensus leadership: fixed; an AWS voter must win a new term with two explicit AWS votes before any Wuji fence or AWS ownership transition.

## Existing architecture constraints

- A distributed authority group has at least three voters.
- Stable quorum is a strict majority; loss of majority intentionally halts new authority.
- A node outside quorum cannot advance the log, renew authority, fence, or activate.
- Recovery must select a majority-committed prefix, never the numerically highest local history.
- Fence and replacement activation use a newer committed authority transition and a safety window.
- Exactly one Observatory exists per distributed polis, not per node.
- A polis shepherd launches with the polis, owns serial orchestration and the single Observatory endpoint, but is not a voter and cannot grant authority.
- Existing identity, certificate, membership, failure, lease, fencing, placement, migration, recovery, and projection modules remain authoritative.

## Proposed topology and serial phases

### Phase A — local proving topology

- Start exactly three real Runtime v3 Guardian/kernel voters on Wuji.
- Give every voter a distinct identity, control key, port set, durable state root, and activation incarnation.
- Start exactly one polis-level Observatory backed by one coherent redacted projection cut.
- Launch the polis shepherd before member admission; it owns the run state and Observatory lifecycle and stops only after reconciliation.
- Prove membership convergence, authenticated traffic, one governed mutation, loss of one voter while two retain quorum, recovery or truthful fail-closed behavior, and deterministic shutdown.
- Prove all processes, Observatory listeners, ports, locks, temporary credentials, topology leases, and test roots are released.
- Phase B must refuse to start without this cleanup receipt.

### Phase B — hybrid continuity topology

- Verify AWS profile `agent-logic-admin` resolves to the approved Agent Logic business account.
- Start exactly one voter on Wuji and two independently stateful voters in AWS, placed in separate AWS failure units.
- Use authenticated encrypted private transport and private Observatory access such as SSM; expose no unauthenticated public endpoint.
- The failure-injection harness must not hold simultaneous data-plane connections that bridge Wuji and AWS. It inspects isolated Wuji behavior only through a Wuji-local Unix-domain socket or loopback control channel that has no route to AWS.
- Reach a healthy three-voter committed configuration before snapshot or failure injection.
- While all three voters are healthy, have Wuji propose a snapshot boundary that the quorum commits at an exact log index.
- Require every voter to independently materialize the canonical snapshot from that exact committed prefix through the native snapshot/catalog path; manual copy cannot satisfy proof, and no network snapshot-transfer event is claimed for healthy current followers.
- Each AWS voter independently verifies the byte-identical canonical snapshot digest, trust domain, lineage, voter-set generation, epoch, committed log index, certificate/endorsement authority, restore eligibility, and native consensus catch-up provenance.
- Confirm both AWS voters have the committed log and independently materialized snapshot state needed to preserve the same committed prefix.
- Serially compact the pre-boundary log and restart each AWS voter, one at a time while Wuji and the other AWS voter retain quorum. Prove each restart loads the canonical snapshot root and cannot access or replay the compacted prefix.
- Asymmetrically partition the still-running Wuji Guardian/kernel and its Observatory ownership path from the AWS pair as the failure injection.
- Require one AWS voter to win a new consensus term with explicit votes from both AWS voters before any fence, owner activation, shepherd activation, or replacement Observatory lease can be committed.
- Wait for the quorum-enforced Wuji Observatory lease TTL to expire within the bounded safety window, and only then assert Wuji-local stale-read and lease denial through the isolated local control channel.
- Require the two AWS voters to retain strict two-of-three quorum, commit failure/fencing state, wait through the safety window, and activate exactly one AWS owner.
- Complete a new governed mutation at an index strictly greater than the pre-failure snapshot index.
- After an intentional bounded interruption during the safety window, bind exactly one AWS-hosted polis Observatory only after the AWS shepherd verifies the committed Wuji fence/new epoch and acquires the current Observatory ownership lease.
- Before that fence and lease exist, attempt AWS shepherd activation and Observatory binding and prove both fail closed.
- Show Wuji unavailable, the committed fence, epoch/index transition, active AWS owner, and continued coherent state.
- Pre-provision a restartable AWS shepherd standby. After Wuji loss it may reconstruct the orchestration/Observatory plane only from majority-committed polis state, never from Wuji-local progress, and remains outside the voter set.
- Give the AWS standby a distinct newly generated non-exportable identity and admit it through a governed mutation during healthy `hybrid_three_running`, before snapshot creation or failure injection. Never copy or pre-authorize the Wuji shepherd key out of band.
- Colocate that standby, with separate CPU/memory limits, beside the AWS voter designated to survive the later halt test. The cascading failure always stops the voter in the opposite Availability Zone. The standby uses only the survivor host's private loopback `ollama_http` endpoint; it has no public or partition-bridging model route.
- Heal the asymmetric partition without restarting Wuji and prove the still-running stale voter cannot mutate, renew, acquire the Observatory lease, serve stale reads, or overwrite AWS authority; it must demote and synchronize only through the explicit governed rejoin/recovery transition after observing the higher AWS epoch.
- After that healed-partition demotion and synchronization proof, re-partition Wuji, stop the AWS voter in the opposite Availability Zone, and prove the one remaining reachable AWS voter cannot commit new authority while the colocated shepherd remains alive and reports the halt.
- Terminate all ephemeral AWS compute, volumes, network attachments, sessions, credentials, listeners, locks, and test state and retain machine-verifiable cleanup evidence.

## Planned state machine

`idle -> wuji_three_running -> wuji_three_stopped -> hybrid_three_running -> aws_shepherd_admitted -> hybrid_mutation_committed -> hybrid_inference_saturated -> hybrid_snapshot_boundary_committed -> aws_consensus_caught_up -> all_voters_snapshot_materialized -> snapshot_prefix_compacted -> aws_snapshot_restart_proven -> wuji_asymmetrically_partitioned -> aws_consensus_leader_elected -> safety_window -> wuji_observatory_lease_expired -> wuji_fenced -> aws_owner_activated -> aws_shepherd_activated -> aws_observatory_leased -> aws_continuity_proven -> partition_healed_stale_wuji_demoted -> wuji_repartitioned -> aws_one_of_three_halted -> hybrid_stopped`

Every transition is durable, bounded, idempotent, and fail closed. Hybrid start is legal only from `wuji_three_stopped` with a valid cleanup receipt. No Phase A and Phase B process may overlap.

## Planned proof lanes

1. `three-voter-runtime-contract`: focused exact Rust integration target proving process launch, quorum, projection, snapshot eligibility, fencing, activation, stale-node denial, bounds, restart, and teardown.
2. `phase-a-wuji-three`: real three-process Wuji run with one Observatory, one-voter loss, recovery, and cleanup.
3. `phase-b-wuji-aws-recovery`: real one-Wuji/two-AWS run, quorum-committed snapshot boundary plus identical canonical per-voter materialization and serial compaction/restart proof, bounded maximum-concurrency context-heavy inference saturation that reaches configured limits with consensus timing proof, asymmetric partition while Wuji remains alive without a harness bridge, explicit two-AWS leader election, old Observatory lease TTL expiry and stale-read denial, premature pre-fence shepherd/Observatory activation denial, durable fence/owner/shepherd activation, post-failure mutation, quorum-leased Observatory recovery, healed-partition stale-Wuji demotion and synchronization, then Wuji re-partition plus loss of the opposite-AZ AWS voter proving a true one-of-three halt while the shepherd host survives, and teardown.
4. `exact-plan-runtime-and-receipt-review`: validates exact source and argv, nonzero node/test denominators, snapshot and committed-prefix digests, quorum transitions, Observatory captures, serial non-overlap, AWS account identity, cleanup predicates, redaction, and reviewer provenance.

The two live lanes share one exclusive serial group and may never execute concurrently.

## Configurable shepherd and model plan

- The runtime configuration must contain an explicit `shepherd_agent_ref` resolving to a governed agent definition with model/provider reference, limits, and policy. Missing, ambiguous, voter-bound, or unauthorized shepherd refs fail startup.
- The shepherd launches as part of polis startup but remains outside consensus authority.
- Phase A may select three smaller local models so Wuji can host all voters concurrently. Exact model IDs, artifact digests, memory/context limits, and health are captured; model capability does not alter Guardian authority behavior.
- Phase B runs pinned bounded self-hosted local models on the Wuji voter and both AWS voters through the already supported private `ollama_http` provider contract, unless a separate review approves another compatible local serving surface.
- Each AWS Availability Zone has its own pinned model artifact and bounded local server. Endpoints are private; startup, identity, inference smoke, restart, and teardown are proven. Hosted model APIs are not required.
- Each AWS voter has an independent EBS volume, network interface, and least-privilege instance role. No shared writable model/state volume is permitted.
- Guardian/kernel consensus and `ollama_http` inference have separate hard memory and CPU controls; inference saturation must not starve heartbeat, log replication, fencing, or lease work.

## Required evidence

- Exact source revision and executable argv.
- Node, process, trust-domain, and voter-set identities without private keys or raw certificates.
- Pre-snapshot, committed snapshot-boundary, per-voter canonical snapshot materialization, pre-failure, lease-expiry, fence, activation, post-failure mutation, and stale-rejoin epoch/index/digest chain.
- Proof that both AWS voters held the majority-committed prefix and identical canonical snapshot digest before the Wuji asymmetric partition was injected.
- One live polis Observatory capture per phase, with API/WSS machine capture and operator-visible view.
- Account/profile verification without credential disclosure.
- Process, port, lock, volume, network, SSM, credential, and state cleanup predicates.
- Fresh independently attributable exact-head implementation review after the plan is implemented.

## Non-goals

- Two-voter automatic failover.
- Parallel local and hybrid demos.
- One Observatory per node.
- Selecting the highest local epoch or trusting a copied snapshot as authority.
- Public unauthenticated endpoints, plaintext transport, shared identities/state, permanent AWS infrastructure, Kubernetes, customer traffic, umbrella reconciliation, or async lifecycle closeout.

## Questions Gemini must answer

1. Does this topology preserve quorum and prevent split-brain when Wuji disappears?
2. Is the snapshot/committed-prefix sequence sufficient, and what additional preconditions or evidence are mandatory?
3. Can the Observatory truthfully remain available and coherent during ownership transfer?
4. Are two AWS voters sufficiently independent, and what failure-domain constraints must be explicit?
5. Could stale Wuji regain authority through any replay, lease, certificate, epoch, or restore path?
6. Are the serial gate and teardown predicates strong enough to prevent cross-demo contamination?
7. What negative tests or live failure injections are missing?
8. Is any acceptance criterion likely to permit a false-green demonstration?
9. Is the shepherd launch, authority exclusion, AWS standby, and committed-state reconstruction lifecycle safe and sufficient?
