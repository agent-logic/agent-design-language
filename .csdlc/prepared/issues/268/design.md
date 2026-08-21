# Issue #268 — Six-hour Runtime v3 Spot qualification

## Authority and boundary

Run one bounded six-hour Runtime v3 production qualification in the Agent Logic
business AWS account. The operator authorized issue #268 on 2026-08-17 with a
hard total cost ceiling of USD 20. Issue #269 is excluded and requires a new
decision.

The repository-native Spot owner remains the only AWS mutation surface. It
must verify the `agent-logic-admin` account proof, immutable source revision,
Standard Spot quota, price estimate, task tags, deadline, kill
switch, and exact-owner cleanup before launch. No personal/default account,
GPU, On-Demand fallback, second attempt, or unrelated-resource mutation is
allowed.

## Source-grounded implementation

Issue #266 supplied the bounded soak configuration, sampler, evaluator,
evidence, receipt, cancellation, and cleanup contracts. Issue #267 plus #373
and #374 supplied the production Guardian/kernel workload and required
fault/recovery receipts. Terminal #414 supplies the existing Runtime-v2-backed
resident dehydration/rehydration bridge, signed complete-population
LiveContinuity binding, retained Runtime-volume contract, IMDSv2 callback, and
Linux Ollama bootstrap contract. Terminal #415 supplies exact early builder
failure diagnostics. #268 must consume those authorities from their ancestral
merge commits and must not recreate them.

Extend `adl-runtime-lifecycle-soak` with one fixed `six_hour_qualification`
suite. Qualification requires at least 21,600 monotonic seconds of production
exposure. A cycle already in flight at the deadline may finish only under an
explicit 600-second timeout, so measured overshoot must be no more than 600
seconds. The receipt records minimum, measured, and overshoot seconds. Both the
21,600-second minimum and 600-second cap are constants rather than
caller-controlled inputs. Existing suites and
short-qualification semantics remain unchanged.

Add an issue-owned shell entrypoint that:

1. fails closed unless issue, revision, profile, region, Spot-only posture,
   authorization marker, USD 20-or-lower ceiling, deadline, cancellation path,
   output root, and direct-host Runtime qualification command are resolved;
2. performs a no-mutation preflight and records current Spot prices;
3. consumes a portable AWS request whose immutable revision, 25,200-second
   provider timeout, disabled fallback, and `estimated_max_cost_microusd` of
   20,000,000 are verified before launch;
4. launches exactly one repository-native asynchronous Spot run using only
   `r7i.2xlarge` (8 vCPU/64 GiB); capacity failure is terminal and cannot select
   another type;
5. uses EC2 user data to install ordinary Amazon Linux package prerequisites,
   mounts one issue-owned persistent EBS Runtime volume, validates and reuses
   the reviewed Linux/x86 Runtime installation when present, builds only the
   canonical ADL Runtime when that installation is absent, and materializes the
   pinned Linux/x86 Ollama runtime plus model stores from immutable,
   version-pinned S3 objects onto that volume before executing the
   fixed six-hour suite;
6. retains time-series, workload, fault, lifecycle, redacted launch, and final
   digest-bound evidence;
7. on success, failure, interruption, timeout, or cancellation, terminates only
   the exact run-tag-owned instance and independently proves zero remaining
task-owned instances.

The existing Ollama provider remains the sole Runtime model-server boundary.
Ollama owns model loading, unloading, scheduling, local inference, and optional
cloud routing. ADL owns resident identity, role, tool authority, task state,
admission, continuity, and recovery. No parallel provider or model-management
layer is permitted. Cloud/frontier escalation is optional and non-authoritative;
credentials are never checkpointed and local work/recovery cannot depend on it.
S3 remains the canonical bootstrap authority for model bytes: no model is
rebuilt, repackaged, or republished by #268. The persistent EBS Runtime volume
contains only the checksum-verified installed/materialized Ollama and model
copy plus package-managed ordinary prerequisites, the pinned ADL Runtime build,
and continuity state. Its installation receipt makes subsequent mounts
idempotent and fail closed on any VersionId, checksum, architecture, source
revision, or package-set drift. Build cache is a separate ephemeral filesystem
and is never treated as Runtime continuity authority. Qualification source
revision changes outside the reviewed Runtime inputs do not force a rebuild.

The six distinct resident identities are fixed by
`adl/tools/issue268_six_resident_uts_plan.json`: Shepherd/controller and
reviewer/escalation use `llama3.1:8b`; planner and tool executor use `qwen3:8b`;
Runtime observer and recovery custodian use `phi4-mini:latest`. They share
Ollama weights but retain distinct identity, role digest, tool authority,
sequence, completed/pending UTS cases, task/report digests, and checkpoint
lineage. Inference is serial (`max_concurrent_inference=1`) and compilation is
not concurrent.

Their real workload is a governed long-lived Runtime cycle through
`adl agent tick`, with each resident's model output routed through UTS-to-ACC
compilation, Freedom Gate, and the Runtime-owned `runtime.observe` adapter.
The named pre/post cases remain drawn from the canonical 33-task panel. Every
resident completes a named case before dehydration, resumes a distinct named
case only after validated restore, and retains exact completed/pending case
identities. A completed side effect or case may not replay after recovery. The
final receipt binds all six resident identities to their pre/post UTS reports
and the signed continuity generation.

Extend the shared Spot wrapper only where the reviewed #414/#415 integration
requires it and keep `--max-spot-retries` at zero. No other provider policy
change is authorized.

The wrapper passes zero Spot retries and never retries under a new instance or
run identity. The owner validates projected maximum cost as hourly price times
the full 25,200-second provider timeout, not merely the workload interval. A Spot
interruption is a truthful terminal outcome, not permission to erase or restart
the attempt.

An atomic durable launch claim is created before the asynchronous owner starts;
later invocations can only resolve that exact claim. If the manager dies before
cleanup, validation discovers only instances carrying the exact run-id tag,
terminates those exact instances, waits for termination, clears the temporary
private identifier list, and repeats the independent zero-instance query.

## Proof

Local proof covers the fixed minimum 21,600-second monotonic denominator,
bounded final-cycle overshoot, rejection of any duration override, exact
`r7i.2xlarge` resources, the six unique roles/model allocation, real UTS task
membership, completed-case replay denial, exact continuity checkpoint fields,
mandatory authorization/cost/Spot-only fields, stale or missing evidence,
cancellation, cleanup, redaction, and exact scope without provider mutation.
An explicitly authorized idempotent launch operation starts
the one asynchronous run and returns after ownership is established; repeated
invocation may only resolve that same run identity and cannot create a second
instance. It is not itself a claim that six hours passed. A separate
terminal-status command fails until the owner reports a terminal outcome, and
receipt/cleanup validation then proves the elapsed denominator and independent
zero-instance cleanup. These are three serial lifecycle gates, not a parallel
PVF wave: launch must complete first, terminal status must be observed later,
and validation runs only after terminal cleanup.
Exact-head review after evidence capture must find no actionable issue before
publication and finish.

## Stop conditions

- resolved AWS identity is not the retained Agent Logic account;
- Standard Spot quota is insufficient;
- current estimated total exceeds USD 20;
- immutable source, direct-host command, or retained Runtime resolution fails;
- task-owned resources already exist for the run id;
- any required production/fault receipt is absent;
- cleanup cannot prove zero remaining task-owned instances;
- a second attempt would be required without new operator authorization.
