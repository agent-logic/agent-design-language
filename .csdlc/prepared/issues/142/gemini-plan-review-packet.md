# Gemini Findings-First Plan Review Packet — Issue #142

## Review target

Review the pre-execution design and validation plan for `agent-logic/agent-design-language#142`, titled:

`[v0.92][runtime] Operationalize distributed Runtime and polis Observatory across Wuji and AWS`

Return findings first, ordered P0 to P3. For each finding, state the violated safety or proof contract, the exact plan element at fault, and a concrete correction. Then give a verdict: `APPROVE`, `CHANGES REQUESTED`, or `BLOCKED`. Do not assume implementation exists.

## Existing architecture constraints

- A distributed authority group has at least three voters.
- Stable quorum is a strict majority; loss of majority intentionally halts new authority.
- A node outside quorum cannot advance the log, renew authority, fence, or activate.
- Recovery must select a majority-committed prefix, never the numerically highest local history.
- Fence and replacement activation use a newer committed authority transition and a safety window.
- Exactly one Observatory exists per distributed polis, not per node.
- Existing identity, certificate, membership, failure, lease, fencing, placement, migration, recovery, and projection modules remain authoritative.

## Proposed topology and serial phases

### Phase A — local proving topology

- Start exactly three real Runtime v3 Guardian/kernel voters on Wuji.
- Give every voter a distinct identity, control key, port set, durable state root, and activation incarnation.
- Start exactly one polis-level Observatory backed by one coherent redacted projection cut.
- Prove membership convergence, authenticated traffic, one governed mutation, loss of one voter while two retain quorum, recovery or truthful fail-closed behavior, and deterministic shutdown.
- Prove all processes, Observatory listeners, ports, locks, temporary credentials, topology leases, and test roots are released.
- Phase B must refuse to start without this cleanup receipt.

### Phase B — hybrid continuity topology

- Verify AWS profile `agent-logic-admin` resolves to the approved Agent Logic business account.
- Start exactly one voter on Wuji and two independently stateful voters in AWS, placed in separate AWS failure units.
- Use authenticated encrypted private transport and private Observatory access such as SSM; expose no unauthenticated public endpoint.
- Reach a healthy three-voter committed configuration before snapshot or failure injection.
- While all three voters are healthy, create a majority-committed snapshot on Wuji.
- Transfer it over the authenticated private channel to both AWS voters.
- Each AWS voter independently verifies the snapshot digest, trust domain, lineage, voter-set generation, epoch, committed log index, certificate/endorsement authority, and restore eligibility.
- Confirm both AWS voters have the committed log/snapshot state needed to preserve the same committed prefix; a copied snapshot alone is not treated as quorum authority.
- Stop the Wuji Guardian/kernel and its Observatory ownership path as the failure injection.
- Require the two AWS voters to retain strict two-of-three quorum, commit failure/fencing state, wait through the safety window, and activate exactly one AWS owner.
- Complete a new governed mutation at an index strictly greater than the pre-failure snapshot index.
- Keep or restart exactly one AWS-hosted polis Observatory and show Wuji unavailable, the committed fence, epoch/index transition, active AWS owner, and continued coherent state.
- Attempt to restart Wuji from its stale pre-failure state and prove it cannot mutate, renew, or overwrite AWS authority until explicit governed rejoin/recovery completes.
- Terminate all ephemeral AWS compute, volumes, network attachments, sessions, credentials, listeners, locks, and test state and retain machine-verifiable cleanup evidence.

## Planned state machine

`idle -> wuji_three_running -> wuji_three_stopped -> hybrid_three_running -> snapshot_verified_in_aws -> wuji_down -> aws_continuity_proven -> stale_wuji_rejected -> hybrid_stopped`

Every transition is durable, bounded, idempotent, and fail closed. Hybrid start is legal only from `wuji_three_stopped` with a valid cleanup receipt. No Phase A and Phase B process may overlap.

## Planned proof lanes

1. `three-voter-runtime-contract`: focused exact Rust integration target proving process launch, quorum, projection, snapshot eligibility, fencing, activation, stale-node denial, bounds, restart, and teardown.
2. `phase-a-wuji-three`: real three-process Wuji run with one Observatory, one-voter loss, recovery, and cleanup.
3. `phase-b-wuji-aws-recovery`: real one-Wuji/two-AWS run, snapshot transfer, Wuji shutdown, AWS quorum continuity, fencing/activation, post-failure mutation, Observatory continuity, stale-Wuji denial, and teardown.
4. `exact-plan-runtime-and-receipt-review`: validates exact source and argv, nonzero node/test denominators, snapshot and committed-prefix digests, quorum transitions, Observatory captures, serial non-overlap, AWS account identity, cleanup predicates, redaction, and reviewer provenance.

The two live lanes share one exclusive serial group and may never execute concurrently.

## Required evidence

- Exact source revision and executable argv.
- Node, process, trust-domain, and voter-set identities without private keys or raw certificates.
- Pre-snapshot, snapshot, pre-failure, fence, activation, post-failure mutation, and stale-rejoin epoch/index/digest chain.
- Proof that both AWS voters held the majority-committed prefix before Wuji stopped.
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
