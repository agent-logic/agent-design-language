# SIM-09 prospective pilot protocol v1

Freeze this protocol and its Git revision in the activation decision before any
post-resume journey begins. Sample the first **30 consecutive eligible issue
journeys** at entry; failures, abandonment and censoring remain in that sample.
Do not replace adverse journeys with later successes. Further exposure belongs
to a separately identified extension, never a reset of the initial denominator.

Eligible: a unique issue lifecycle journey admitted under the activated candidate
and supported platform, beginning with its first post-resume operational attempt.
Include new issues and already-bound issues that continue after resume. Repeated
commands/retries for the same journey stay in its original entry. A later
amendment belongs to that same journey unless a prospective protocol amendment
explicitly defines a new lifecycle episode before it starts.

Prospective exclusions: read-only inventory with no operational journey;
synthetic/test fixtures; unsupported platforms; other repositories; attempts
before the recorded resume; and journeys after entry 30. Record every considered
entry, exclusion category, time and actor before learning its outcome. A guard
denial, invalid input, missing dependency, slow run or failure is not an exclusion.
A discovered retrospective eligibility error remains visible as a deviation and
cannot be used to substitute a later successful journey.

At entry, assign ordinal, issue, repository, branch/worktree, exact executable and
source revision, lifecycle phase, entry wall time, protocol revision, dependency
class, work type, proof profile and scale. Append only; corrections reference the
original event. Register before first attempt, including attempts that fail early.

For every attempt retain journey/attempt IDs, command, correlation and intent
identity, operation ID, before/after generation and digest, outcome, reason,
effects, invalidation cause/affected evidence, exit status, wall start/end,
monotonic duration, and evidence references. Record missing fields as null with a
reason; never invent identity/timing. Preserve all retries and guard denials.
Classify applied, successful no-op, expected wait, invalid intent, stale request,
expected guard denial, unexpected fault and recovery-required separately.

Record each wait interval with start/end, monotonic duration, owner and category:
dependency, human decision, review, CI, remote service, provisioning or recovery.
Conversation gaps are not automatically active work. Report setup, active and
wait durations separately; do not subtract overlapping waits twice. Record clock
source/process boundaries and mark incomparable monotonic intervals missing.

At a prospectively declared observation cutoff, each eligible journey has one
mutually exclusive disposition: not attempted, active, completed, failed,
abandoned or censored. Completed requires verified terminal reconciliation and
eligible cleanup, not merely a green PR. Failed attempts inside an eventually
completed journey remain failed attempts. Abandoned means explicit owner cessation;
censored means outcome not observable by cutoff, with reason. Active is available
for interim snapshots; unresolved active rows become censored at final cutoff.

Reconcile:

- considered = eligible + excluded (with reasons);
- eligible = not attempted + active + completed + failed + abandoned + censored;
- attempted = active + completed + failed + abandoned + censored;
- command attempts = first attempts + all retries (each with a recorded outcome).

A registered journey with no actual attempt stays not attempted; do not count it
as a success. Report journey failure and attempt failure denominators separately.
Report guard denials and expected waits separately from defects. Publish all
missing observations, manual interventions, recovery visits and deviations.

Stratify by work type (docs/code/tooling), dependency topology (independent/stacked),
proof profile and scale declared at entry. Show actual per-stratum counts, not
quotas achieved by replacing entries. Report median/P90 setup, active and wait
times with missing-data counts; pre/post comparison is observational. Report
first-pass yield conditional on explicitly stated valid prerequisites alongside
unconditional counts so exclusions cannot hide usability failures.

With zero failures in 30 independent comparable journeys, the one-sided 95%
upper bound is `1 - 0.05^(1/30)`, approximately 9.5%. That is not proof of 99%
reliability. Shared operator, host, issue stack and workload correlations weaken
independence. Publish those clusters and actual exposure; use pilot variance to
plan further sampling rather than making rare-event claims.

SIM-09 owns an append-only entry/attempt/wait/disposition ledger and its final
scorecard. This document defines the record contract; it does not claim that a
pilot collector or any of the 30 journeys has been executed by SIM-08.
