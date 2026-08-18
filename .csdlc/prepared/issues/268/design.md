# Issue #268 — Six-hour Runtime v3 Spot qualification

## Authority and boundary

Run one bounded six-hour Runtime v3 production qualification in the Agent Logic
business AWS account. The operator authorized issue #268 on 2026-08-17 with a
hard total cost ceiling of USD 20. Issue #269 is excluded and requires a new
decision.

The repository-native Spot owner remains the only AWS mutation surface. It
must verify the `agent-logic-admin` account proof, immutable source revision and
builder image, Standard Spot quota, price estimate, task tags, deadline, kill
switch, and exact-owner cleanup before launch. No personal/default account,
GPU, On-Demand fallback, second attempt, or unrelated-resource mutation is
allowed.

## Source-grounded implementation

Issue #266 supplied the bounded soak configuration, sampler, evaluator,
evidence, receipt, cancellation, and cleanup contracts. Issue #267 plus #373
and #374 supplied the production Guardian/kernel workload and all required
fault/recovery receipts. Their squash merges are present on the exact base.

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
   output root, and immutable builder image are resolved;
2. performs a no-mutation preflight and records current Spot prices;
3. consumes a portable AWS request whose immutable revision, 25,200-second
   provider timeout, disabled fallback, and `estimated_max_cost_microusd` of
   20,000,000 are verified before launch;
4. launches exactly one repository-native asynchronous Spot run using only
   `c7i.2xlarge`; capacity failure is terminal and cannot select another type;
5. executes the fixed six-hour suite against the canonical production binaries;
6. retains time-series, workload, fault, lifecycle, redacted launch, and final
   digest-bound evidence;
7. on success, failure, interruption, timeout, or cancellation, terminates only
   the exact run-tag-owned instance and independently proves zero remaining
task-owned instances.

Extend the shared Spot wrapper only to expose and forward a validated
`--max-spot-retries` value to its existing owner binary; #268 always supplies
zero. No other provider policy changes.

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
bounded final-cycle overshoot, rejection of any duration override, mandatory
authorization/cost/Spot-only fields, stale or
missing evidence, cancellation, cleanup, redaction, and exact scope without
provider mutation. An explicitly authorized idempotent launch operation starts
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
- immutable source or builder-image resolution fails;
- task-owned resources already exist for the run id;
- any required production/fault receipt is absent;
- cleanup cannot prove zero remaining task-owned instances;
- a second attempt would be required without new operator authorization.
