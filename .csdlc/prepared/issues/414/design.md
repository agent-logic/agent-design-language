# Issue 414 Design: Bridge Existing Agent Continuity to Spot Reclamation

## Decision

Issue #414 reuses the existing Runtime v2 citizen lifecycle, snapshot,
rehydration, lineage, duplicate-denial, invariant, and wake authority. It adds
only the missing `adl` integration orchestrator, Spot interruption control
bridge, dedicated retained Runtime-volume contract, restore-before-admission
wiring, and a real CPU Ollama proof that the Runtime is habitable, not merely
bootable.

No new lifecycle states, snapshot schema, lineage model, or recovery system are
authorized.

## Existing authority consumed unchanged

- `adl/src/runtime_v2/agent_lifecycle_state.rs`: states `ACTIVE`, `QUIESCENT`,
  `SUSPENDED`, `DORMANT`, `SIMULATION`, `IN_TRANSIT`, `BOOTSTRAP`, `SHUTDOWN`,
  `FORCED_SUSPENSION`, `QUARANTINED`, `REJECTED`, and `ORPHANED`; Spot
  dehydration follows the existing `ACTIVE` -> `SUSPENDED` -> `DORMANT`
  transition while `QUIESCENT` remains a separate habitability behavior, and
  wake requires rehydration validation.
- `adl/src/runtime_v2/types.rs` and `adl/src/runtime_v2/snapshot.rs`:
  `RuntimeV2SnapshotManifest` and `RuntimeV2RehydrationReport`, exact active
  population, structural checksum, invariant checks, duplicate denial, strictly
  advancing trace sequence, and wake refusal.
- `adl/src/long_lived_agent.rs` and `adl/src/csm_continuity_capsule.rs`:
  operational `continuity_checkpoint.json`, ledger-aware status restore,
  safe-fail serialization, and portable checkpoint/restore artifacts.
- `adl-runtime-kernel/src/live_continuity.rs`: signed atomic Runtime v3 kernel
  and ingress generations, checkpoint deadline, lineage validation, and restore.

The existing `live_continuity` checkpoint API receives the smallest required
extension: an optional opaque resident-population subrecord remains within the
signed singleton `live_kernel` payload, and resident-aware restore must return
that subrecord for validation before admission. It adds no participant, service,
signature, lineage, or recovery authority.

## Resident Shepherd integration

The orchestration lives in the `adl` crate because that crate already owns the
public Runtime-v2 snapshot, lifecycle, long-lived-agent, and CSM continuity
capsule APIs and depends on `adl-runtime`. Persistence does not live in
`adl-runtime`, which would reverse the dependency direction and create a second
capsule authority. The rejected `ResidentAgentCapsule` and
`ResidentPopulationCheckpoint` types are removed.

The integration constructs one signed `live_kernel` resident subrecord from an
actual validated `RuntimeV2SnapshotManifest` whose active-citizen records and
index are built from the exact supplied resident and existing-agent population,
hashes of every existing per-agent CSM continuity
capsule/custody manifest, and the exact model/artifact/quantization/configuration
bindings for the local workers. It does not add a checkpoint participant or
change the singleton service set. It never serializes weights, prompt text,
secrets, endpoint identity, or raw tool arguments.

No successful `RuntimeV2RehydrationReport` exists at dehydration time. After
every CSM capsule is restored and its checkpoint/status authority validates,
the orchestrator constructs, validates, and persists the report from the
actually restored exact citizen set. Admission cannot open before that report
exists and validates against the signed snapshot.

Population handling is exact: all admitted resident agents appear once.
Missing, duplicate, partial, oversized, busy, or failed dehydration makes the
generation incomplete and prevents termination readiness.
Distinct inhabitants retain unique agent ID, role/instructions digest,
memory/state digest, task queue/sequence, tool authority, checkpoint lineage,
and activation record even when model artifacts are shared.

## Spot and storage bridge

Only a confirmed IMDSv2 Spot interruption notice triggers the existing
admission-close/checkpoint control with one absolute bounded deadline. Admission closes
before the resident traverses the existing `ACTIVE` -> `SUSPENDED` ->
`DORMANT` Spot path. `QUIESCENT` remains a separate useful-work/habitability
behavior. Every sequential status, stop, and capsule boundary checks the same
deadline. Because those existing APIs are synchronous, the callback is their
deadline supervisor: after setup it recomputes both remaining absolute-notice
time and remaining outer-supervisor time, runs dehydration in a bounded child,
and enforces TERM followed by KILL after two seconds. A three-second absolute
deadline margin and five-second outer margin prevent the child from surviving
either cutoff. On timeout or any later failure admission remains closed, every
existing stop intent is preserved (including concurrent operator authority),
and no termination-ready receipt is written. Checkpoint completion includes
resident plus kernel/ingress.
The normal validation command requests idle watcher shutdown but never kills
the watcher. Once a notice is accepted, durable active/terminal state makes the
accepted bounded callback take precedence; command completion waits for that
terminal receipt or failure before classifying the run.

Signed continuity lives on a dedicated retained/re-attached Runtime volume,
distinct from Cargo home, target, sccache, and build cache. Missing, ambiguous,
substituted, non-exclusive, or build-cache-aliased Runtime storage fails closed.
Both the source-host AWS dehydration callback and replacement-host restore
wrapper independently derive the mounted block-device serial, normalize it to
the approved EBS volume ID, hash it, and require equality with both the runner
contract and the signed resident subrecord; a path string alone is not volume
identity. Reference-host proofs bind an explicitly nonqualifying local identity.

Replacement startup first validates signed lineage, then validates the actual
Runtime-v2 manifest, complete capsule hashes, and exact model name,
quantization, artifact digest, and configuration digest. It restores each CSM
capsule into a disjoint temporary population root, validates status/checkpoint
authority, constructs and validates the actual post-restore Runtime-v2 report,
atomically renames a new immutable generation directory, and then
writes an admission-closed active-population pointer without deleting the prior
generation. It durably prepares the complete restore receipt and atomically
commits the global pointer admission-open before clearing any per-agent stop
intent. On any clear failure it atomically closes/restores the pointer,
re-stops every already-cleared agent, restores the prior receipt, and removes
the failed generation. Only successful completion returns admission open. Warm
model calls are forbidden until that receipt and open pointer both exist.

Linux/x86 bootstrap artifacts come only from the approved versioned, AES256,
public-blocked S3 bucket through a SHA256 manifest bound to an immutable reviewed
Git SHA and the exact runner and continuity-binary SHA256. The installer prefix is `shepherd/issue-414/<sha>/installer`; Ollama
0.31.1 comes from the existing Linux-amd64 runtime object, and llama3.1:8b,
qwen3:8b think:false, and phi4-mini retain the established source-digest model
store layout. Mac MLX/Metal stores are forbidden. Publication requires Linux
x86_64 and a clean exact reviewed commit, verifies every staged artifact, and
creates (never overwrites) the runtime, all three model stores, and their
manifest beneath that immutable prefix; the r7 runner requires the manifest
commit to equal its executing Git HEAD and verifies its own and the continuity
binary's digests; #268 owns staging/fetch and measured
execution. S3 is only a bootstrap cache, never continuity authority.

## Habitable CPU Shepherd proof

The local runner carries an exact `r7i.2xlarge` contract (8 vCPU/64 GiB) and
records truthful reference-host receipts. A reference receipt is always
nonqualifying: it records API-observed loaded-model bytes and request-level
configuration while explicitly marking external-server environment and RSS as
unverified. The managed lane requires a proof-owned Ollama PID, exact RSS/swap,
and verified launch environment, and the paid measured r7i.2xlarge execution is
explicitly deferred to #268. The local runner exercises
multiple distinct resident local Shepherd agents backed by pinned, digested
CPU-local models/configurations: required `llama3.1:8b` Q4 baseline,
`qwen3:8b` structured-agent comparison, and `phi4-mini` utility worker. A
bounded `gpt-oss:20b` shared escalation is optional and non-authoritative.
The conservative starting configuration is Ollama parallel=2,
max-loaded-models=2, context=8192, with no concurrent compilation; measured
loaded-model capacity, RAM/swap measurement availability, and latency decide
whether it remains admissible. Every
Shepherd must admit, perform a schema-valid useful bounded task, exercise
quiescent behavior separately, traverse suspended/dormant for Spot, dehydrate
as one complete population, restore
without missing or duplicate activation, and perform a deterministic next
useful task. Sentinel-only output is not proof.

An optional bounded call to a large external model may be measured separately,
but it cannot be required for admission, local useful work, dehydration,
rehydration, deterministic resume, or pass/fail authority.

The operator can observe and control lifecycle, admission, checkpoint,
rehydration, and task status without gaining private-state authority. A redacted
receipt records model artifact digest/quantization, worker count/context, cold
and warm latency, loaded-model bytes and capacity headroom, task result digests,
dehydration generation/digest, restored state digest, and next-transition
digest. No model weights, prompts, credentials, private host identity, or raw
tool arguments are retained.

## Owned implementation paths

- `adl/src/resident_shepherd_spot_continuity.rs`
- `adl/src/bin/adl_resident_shepherd_continuity.rs`
- `adl/src/lib.rs` (additive registration only)
- `adl/Cargo.toml` and `adl/Cargo.lock` (exact dependency/lock parity)
- `adl-runtime/src/lib.rs` (minimal existing-kernel API re-export only)
- `adl-runtime/src/agent_lifecycle.rs`, `adl-runtime/src/bin/adl-runtime-resident-shepherd-continuity.rs`, `adl-runtime/src/resident_shepherd_continuity.rs`, and `adl-runtime/tests/resident_shepherd_spot_continuity.rs` (intentional deletion of rejected parallel lifecycle/capsule authority)
- `adl-runtime-kernel/src/live_continuity.rs` (signed singleton subrecord API only)
- `adl/src/runtime_v2/agent_lifecycle_state.rs` (existing transition authority)
- `adl/src/runtime_v2/citizen.rs` and `adl/src/runtime_v2/contracts.rs` (exact active-population snapshot construction)
- `adl/src/runtime_v2/snapshot.rs` (actual post-restore rehydration report construction)
- `tools/aws_remote_validation/src/aws_remote_validation.rs`
- `tools/aws_remote_validation/scripts/remote_validation_runner.sh`
- `adl/tools/aws_spot_artifact_finalize.py`
- `adl/tools/test_aws_spot_artifact_finalize.sh`
- `adl/tools/run_aws_spot_remote_validation_lane.sh`
- `adl/tools/test_run_aws_spot_remote_validation_lane.sh`
- `adl/tools/run_issue414_cpu_shepherd_continuity.sh`
- `adl/tools/issue414_spot_dehydrate_callback.sh`
- `adl/tools/issue414_restore_and_admit.sh`
- `adl/tools/issue414_s3_linux_bootstrap.py`
- `adl/tools/test_issue414_s3_linux_bootstrap.py`
- `adl/tools/run_issue414_llama_baseline.sh`
- issue-local lifecycle, preparation, and evidence paths for #414

## LiveContinuity boundary

`adl-runtime-kernel/src/live_continuity.rs` receives only an opaque optional
subrecord and a validator-gated restore return. The validator runs before
ingress, recorder, generation, or lineage-head mutation. The singleton
`live_kernel` participant, signature, lineage, deadline, and service-set
authority remain unchanged. The `adl` orchestrator is the only layer that knows
the subrecord contains Runtime-v2 and CSM capsule authorities.

## Proof matrix

1. Exact signed Runtime-v2 manifest plus actual post-restore report, complete CSM capsule population,
   capsule/custody hashes, redaction, and deterministic next useful task.
2. Missing/partial/duplicate/tampered/rollback/model-or-volume substitution,
   oversized/busy agent, and deadline expiry all fail closed.
   Deterministic two-agent fault injection delays the second capture beyond its
   boundary and proves both real stop intents remain closed; admission fault injection
   proves the current agent is re-stopped and prior pointer/receipt/generation
   authority is restored.
3. Confirmed IMDSv2 notice only; the watcher invokes a bounded callback with the
   real notice JSON and deadline; admission close precedes existing lifecycle
   dehydration and complete checkpoint precedes termination readiness.
4. Dedicated Runtime volume is distinct from build cache; restore completes
   before admission and no duplicate activation occurs.
5. Multiple distinct pinned CPU-local residents: required `llama3.1:8b` Q4,
   `qwen3:8b`, and `phi4-mini`, with optional `gpt-oss:20b` escalation,
   prove useful work before/after recovery and cold/warm latency. Reference
   receipts cannot qualify resource headroom. The managed runner contract binds
   parallel=2/max-loaded=2/context=8192, actual PID RSS/swap, no concurrent
   compilation, and the exact r7i.2xlarge 8-vCPU/64-GiB envelope; #268 owns its
   paid measured execution.
6. The CPU runner and its focused integration-test contract prove exact pinned
   models/configuration, useful task receipts before dehydration, the actual
   shared lifecycle and signed LiveContinuity checkpoint, validator-gated
   complete-population restore, useful continuation afterwards,
   RAM/swap/latency metrics, and non-authoritative external-model exclusion.
7. Operator status/control visibility, exact scope, strict Clippy, diff hygiene,
   fresh exact-head review, CI, merge, typed finish, cache, and ancestry.

Historical three-model reference attempts remain explicit non-proving evidence.
Only the classified llama baseline reference hashes may be cited as #414 local
model evidence; exact three-model Linux/r7 qualification is deferred to #268.

## Non-goals and stop conditions

- No new continuity state/schema/lineage/recovery system.
- No Runtime v2 state or transition invention; the existing matrix consumes and
  re-exports the shared Runtime transition authority used by the adapter.
- No model-weight serialization, GPU, paid #268 launch, or #269 mutation; any
  optional external model remains non-authoritative and non-required.
- Stop on nonzero denominator failure, inability to separate Runtime storage,
  resource headroom failure, stale review, CI failure, or terminal mismatch.
