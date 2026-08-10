# Gemini Plan Review — Issue #142

- Provider: Gemini
- Model: `gemini-3.1-pro-preview`
- Response ID: `PQ96aojjHo6pmtkP7NeJ6A4`
- Prompt tokens: 1,618
- Candidate tokens: 1,557
- Verdict: **CHANGES REQUESTED**

## Findings

### P0 — Consensus bypass via out-of-band snapshot transfer

The proposed manual authenticated snapshot copy could false-green recovery without proving consensus replication. AWS voters must receive the snapshot and committed log prefix only through the authoritative consensus/recovery protocol, with native catch-up evidence.

### P0 — Observatory split-brain during partition

The AWS shepherd cannot bind the Observatory merely after observing Wuji loss. It must verify a quorum-committed fence/new epoch and acquire the current Observatory ownership lease before serving.

### P1 — Missing durable fencing and activation states

The state machine must explicitly persist `wuji_fenced` and `aws_owner_activated` between `wuji_down` and `aws_continuity_proven`.

### P2 — Ambiguous AWS failure domains

The two AWS voters must occupy distinct Availability Zones rather than vaguely separate failure units.

### P3 — Stale Wuji Observatory denial not proven

A stale Wuji restart must fail to acquire the Observatory lease and refuse stale reads as well as mutation authority.

## Additional required negatives

- Asymmetric partition: Wuji remains running but cannot reach AWS; its authority and Observatory service must stop after lease loss while AWS waits for quorum fencing.
- Cascading failure: after AWS takeover, stop one AWS voter; the remaining one-of-three must halt new mutation.

## Reviewer conclusions

- Two AWS voters preserve quorum after Wuji loss, but only if Observatory/shepherd activation is quorum-gated.
- A brief intentional Observatory interruption during the safety window is truthful; continuous zero-downtime service must not be claimed.
- A copied snapshot is not authority. Majority-committed prefix and native consensus installation are mandatory.
- The serial Phase A cleanup gate is strong if machine verified.

## Second-pass review

- Model: `gemini-3.1-pro-preview`
- Response ID: `NRB6as3AFuqI6dkPsMjLSA`
- Prompt tokens: 2,036
- Candidate tokens: 1,334
- Verdict: **CHANGES REQUESTED**

### P0 — AWS shepherd identity and authorization undefined

The AWS standby must have a distinct non-exported identity. It must be pre-authorized in the initial polis config or admitted by a governed mutation before Wuji failure; copying the Wuji shepherd key is forbidden.

### P1 — Negative tests acknowledged but not explicitly scheduled

The live hybrid lane must explicitly execute asymmetric Wuji partition and post-takeover loss of one AWS voter, proving the remaining one-of-three halts mutation.

### P2 — Co-located model inference can starve consensus

Each AWS local-model server must have hard resource isolation from Guardian/kernel consensus work, including memory limits and CPU allocation, with a starvation-pressure regression.

### Additional constraints

- The two AWS voters must not share EBS volumes, network interfaces, or one globally revocable instance role.
- `aws_consensus_caught_up` must prove both AWS log indices equal the committed Wuji index before failure injection.

## Third-pass review

- Model: `gemini-3.1-pro-preview`
- Response ID: `pRB6aqKUFLeCqtsP18ifMA`
- Prompt tokens: 2,252
- Candidate tokens: 1,384
- Verdict: **CHANGES REQUESTED**

### P0 — Pre-failure mutation and snapshot states absent

The durable state machine must record the governed AWS shepherd admission mutation and the committed snapshot before AWS catch-up.

### P1 — Partition proof contradicts `wuji_down`

The live state must be `wuji_asymmetrically_partitioned`, leaving Wuji running but network-isolated so stale authority and Observatory denial are genuinely tested.

### P1 — Phase B Wuji model configuration missing

The Wuji voter also needs a pinned bounded local-model server for the pre-failure governed mutation.

### P2 — Shepherd admission timing ambiguous

The distinct AWS shepherd identity must be admitted through a governed mutation during the healthy hybrid phase, rather than optionally pre-authorized out of band.

## Fourth-pass review

- Model: `gemini-3.1-pro-preview`
- Response ID: `axF6aqygFfSLmtkPwLSX6A4`
- Prompt tokens: 2,413
- Candidate tokens: 1,178
- Verdict: **CHANGES REQUESTED**

### P0 — Restart weakens the asymmetric-partition proof

Restarting Wuji would convert the scenario into a crash test and could erase stale in-memory behavior. Heal the partition while Wuji remains alive, then prove the higher AWS epoch forces demotion and authoritative synchronization.

### P1 — Inference saturation was not scheduled

The live hybrid lane must deliberately saturate bounded context-heavy local-model inference and prove Guardian/kernel heartbeat, replication, fencing, and lease timing remain safe.

### P2 — Stale-state name implied restart

Rename `stale_wuji_rejected` to `partition_healed_stale_wuji_demoted` so retained lifecycle truth matches the physical test.

## Fifth-pass review

- Model: `gemini-3.1-pro-preview`
- Response ID: `0hF6ar6BDafKqtsP1N_o2A0`
- Prompt tokens: 2,534
- Candidate tokens: 1,317
- Verdict: **CHANGES REQUESTED**

### P1 — The test harness could bridge the partition

Inspect isolated Wuji only through a Wuji-local Unix-domain socket or loopback path, and forbid the harness from holding simultaneous data-plane connections to Wuji and AWS.

### P1 — AWS shepherd failure domain was ambiguous

Resource-isolate the standby beside the AWS voter designated to survive, and target the opposite-AZ voter in the later one-of-three halt test.

### P2 — Durable inference and shepherd activation states were absent

Add explicit `hybrid_inference_saturated` and `aws_shepherd_activated` states.

### P3 — AWS shepherd model access was ambiguous

Use only the designated surviving host's private loopback `ollama_http` service; do not add a public or partition-bridging model route.

### Additional negative required

Attempt AWS shepherd activation and Observatory binding before the Wuji fence and lease exist; both must fail closed.

## Sixth-pass review

- Model: `gemini-3.1-pro-preview`
- Response ID: `lxJ6auMO2-TPsg-H38rxCw`
- Prompt tokens: 2,843
- Candidate tokens: 1,506
- Verdict: **CHANGES REQUESTED**

### P0 — Observatory lease expiry was implicit

Make the quorum-enforced Wuji Observatory lease TTL expire within the bounded safety window before AWS can receive a replacement lease, and record that state durably.

### P0 — The final quorum-loss topology was mathematically inconsistent

After healed-partition demotion is proven, re-partition Wuji before stopping the opposite-AZ AWS voter so the surviving reachable set is truly one-of-three.

### P1 — Healthy followers do not need a network snapshot transfer

Prove a quorum-committed snapshot boundary and byte-identical canonical snapshots independently materialized by each healthy voter, rather than claiming a Raft snapshot-transfer event that would not occur for current followers.

### P2 — Stale-read denial timing was underspecified

Assert Wuji-local stale-read and lease denial only after the old Observatory lease TTL has expired, using the isolated local control channel.

## Seventh-pass review

- Model: `gemini-3.1-pro-preview`
- Response ID: `ORN6apiQObTqz7IPr-m3GQ`
- Prompt tokens: 3,144
- Candidate tokens returned before truncation: 324
- Verdict: **INCOMPLETE — MAX_TOKENS; not treated as approval**

### P0 — Snapshot recovery could fall back to the historical log

Serially compact the pre-boundary log and restart each AWS voter from its canonical snapshot root while the other two voters retain quorum.

### P1 — Consensus leadership was implicit

Require an AWS voter to win a new term with explicit votes from both AWS voters before the quorum commits any Wuji fence or AWS ownership transition.

## Final eighth-pass review

- Model: `gemini-3.1-pro-preview`
- Response ID: `2RN6atenNYbgz7IPmomJ4As`
- Prompt tokens: 3,363
- Candidate tokens: 28
- Verdict: **APPROVE**

Gemini reported no findings and concluded that the revised plan addresses the prior safety, topology, recovery, model-serving, shepherd, Observatory, and proof requirements.
