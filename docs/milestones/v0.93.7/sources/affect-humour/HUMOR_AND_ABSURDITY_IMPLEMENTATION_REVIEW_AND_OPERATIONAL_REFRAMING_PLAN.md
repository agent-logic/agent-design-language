> Retained source proposal. Current scheduling: v0.93.7 under #1405. Older version/owner references below are historical; they do not override the current execution graph. Research citations and comparative claims require verification before AF-08 publication.

# Humor and Absurdity: Implementation Review and Operational Reframing Plan

Date: 2026-10-10

Status: TBD research and planning input; not a milestone acceptance record.

Audited repository: `agent-logic/agent-design-language`, `main` at
`c01b75428864c438881aa70a9ce291376da56b5c` (2026-10-09).

## Finding

ADL has executable humor/absurdity contracts and a runtime-derived reframing
diagnostic surface. The inspected code does not establish a complete operational
loop that detects a failed interpretation, constructs a better frame, executes
a revised plan, and evaluates whether the change helped.

This is a substantive implementation foundation. It is not yet evidence that
humor or absurdity handling improves an agent's decisions on unfamiliar tasks.

The immediate objective is to make the original cognitive idea causal:
recognize when the current interpretation is inadequate, tolerate the mismatch,
and recover coherently under a bounded alternative interpretation.

## Scope and Method

This review inspected the attached ADL roadmap and feature documents, then
traced current repository source, callers, artifact consumers, and test source.
The repository source at the audited commit takes precedence over older roadmap
paths and implementation descriptions.

This was a source audit. Rust tests were not executed, and no deployed system,
production telemetry, or comparative performance was verified. Findings about
missing integration mean that integration was not found in the inspected source;
they are not claims about every branch or external service.

## Existing Implementation

### WP-12 humor and absurdity contract

Primary source: `adl/src/runtime_v2/humor_and_absurdity.rs`.

The module provides typed review packets, construction, validation, and
deterministic JSON serialization. Its schema is
`humor_and_absurdity_review_packet.v1`.

Five canonical signals are represented:

1. Frame adequacy.
2. Contradiction detection.
3. Bounded reframing.
4. Truth and dignity preservation.
5. Anti-manipulation boundary.

Four required fixture classes cover constructive reframing, failed reframing,
manipulation risk, and inappropriate humor. Manipulative or minimizing fixtures
must have a refusal or escalation outcome.

The implementation connects fixture references to moral trace examples,
outcome-linkage examples, moral resources, trajectory findings, anti-harm
decisions, and wellbeing fixtures. It validates reference membership and
coverage and requires explicit interpretation boundaries and limitations.

**Implementation limit:** `humor_and_absurdity_review_packet()` takes no live
task input. It constructs fixed scenarios, prior and new frames, assessment
levels, findings, and outcomes from authored examples. The validator verifies
the packet's contract; it does not establish the semantic truth of a claimed
contradiction or discover a better interpretation of a new task.

The packet is consumed by affect-reasoning-control and cultivating-intelligence
review surfaces and exported by the cognitive-being flagship demo. These are
real artifact integrations, but they do not demonstrate execution changing in
response to a newly detected absurdity.

### Runtime reframing diagnostics

Primary source: `adl/src/execute/state/runtime_control.rs`.

`derive_runtime_control_state()` constructs a deterministic snapshot from run
status, step records, and trace signals. The snapshot includes evaluation,
reframing, Freedom Gate, and memory participation state.

For a qualifying failure, the reframing branch records:

- `handoff_to_reframing` as the next evaluation action.
- A low `frame_adequacy_score`.
- An explicit trigger and reason.
- `diagnose_and_restructure_before_retry` as the new frame.
- `bounded_reframe_and_retry` or `bounded_reframe_then_review` as the proposed
  continuation.
- `ready_for_reframed_execution` as the post-reframe state.

The Freedom Gate evaluates this state and can derive `escalate`,
`frame_escalation_required`, and `escalate_for_judgment_review`.

**Implementation limits:**

- Adequacy scores are fixed branch values: 28 or 40 in the reframing branch,
  36 for termination on failure, 58 for pause, and 88 for successful completion.
  They are not calibrated measurements of interpretation quality.
- The iteration count used by the reframing branch comes from a constructed
  bounded-execution representation selected by candidate kind. It is not
  itself an observed history of repeated failures or attempted reframes.
- Overall failure maps to a contradiction signal; this does not distinguish a
  failed interpretation from a transient provider or infrastructure failure.
- The replacement frame is a fixed diagnostic label, not a generated,
  evidence-tested alternative model of the task.
- The main CLI error path derives this snapshot while writing failure
  artifacts after execution has failed. It passes an empty step-record slice
  alongside the trace.
- Inspected consumers serialize, persist, summarize, demonstrate, or test the
  proposed continuation. No executor consumer was found that uses it to build
  and run a revised plan.

Consequently, a derived Freedom Gate escalation is an actual decision record,
but this source audit does not establish that it enforces a live reframing
transition before a subsequent action is committed.

### Tests and proof scope

`adl/src/runtime_v2/tests/humor_and_absurdity.rs` contains ten tests covering
valid packet construction, required signal and fixture coverage, stable JSON,
interpretation-boundary drift, missing coverage, invalid references, unsafe
outcomes, and finding consistency.

`adl/src/cli/tests/run_state/runtime_control.rs` checks failure-derived
reframing and gate escalation, and retention of the current frame on success.
Persistence and artifact-builder tests cover record propagation.

These tests establish intended contract and state-derivation behavior when run.
They do not establish novel contradiction detection, successful execution under
a revised frame, or a benefit compared with ordinary retry behavior.

The milestone documentation lists the feature as landed/proving. That status
should be read against its packet and diagnostic proof surface, rather than
expanded into an unsupported behavioral claim.

## Relationship to the Original Humor Idea

The design progression was expectation, incongruity or contradiction, tolerance
of inconsistency, and safe reframing. The existing implementation concretizes
the vocabulary, evidence structures, and governance boundaries around that
idea. It leaves the discovery and execution stages largely unproven.

Operational reframing is one part of the humor design. It does not by itself
implement wit, timing, shared-context interpretation, or amusement. Those
capabilities need separate design and evaluation if pursued. Improved task
recovery must not be presented as proof that humor has been solved.

## Proposed Operational Loop

### Phase 1: Live evidence and failure classification

Introduce a bounded observation record containing the active frame, goal,
assumptions, expected observations, actual observations, attempt history, and
evidence references. Distinguish frame mismatch from transient errors, missing
inputs, impossible constraints, and policy refusal.

Initially use inspectable rules for demonstrable cases, such as repeated
attempts with unchanged assumptions and no new evidence. Any model-based
interpretation must produce a candidate diagnosis rather than unreviewed
control authority. Preserve uncertainty when the evidence cannot distinguish
the causes.

Acceptance: a real execution episode can trigger or decline reframing from its
observed evidence. The implementation does not infer contradiction solely from
the overall failure flag or use constructed iterations as observed history.

### Phase 2: Alternative frame construction and validation

Represent a candidate frame with its changed assumptions, preserved goal and
constraints, supporting and opposing evidence, proposed plan changes, predicted
observable consequences, and resource budget.

Permit alternatives such as requesting a missing input, changing task
decomposition, or diagnosing a mistaken assumption. Reject goal substitution,
unsupported reinterpretation, manipulation, and attempts to evade a policy
boundary. Truth and dignity assessments must be tied to the actual candidate
and affected parties rather than copied from a fixture.

Acceptance: novel task evidence produces a specific alternative that can be
checked against the original goal. A generic instruction to diagnose and retry
does not satisfy this phase.

### Phase 3: Governed execution and outcome feedback

Connect the validated candidate to the active execution owner. Evaluate the
Freedom Gate before committing the revised action. A refusal, deferral, or
escalation must prevent unauthorized continuation; an allowed candidate may
execute only within its declared budget and tool permissions.

Record the original frame, candidate frame, gate result, actual revised plan,
attempts, outputs, and outcome. Limit reframe count and total resource use;
detect oscillation between frames. When recovery fails, preserve the evidence
and terminate or escalate explicitly.

Acceptance: an end-to-end task visibly changes its next executed action because
of the reframing decision. Records saying `performed` or
`ready_for_reframed_execution` alone are insufficient.

### Phase 4: Memory, replay, and integration

Persist episode evidence through the existing trace and ObsMem owners. Record
both successful and unsuccessful reframes with applicability limits. Memory
retrieval may inform a candidate but must not turn an earlier outcome into
automatic authority for a different task.

Replay must reconstruct the recorded control decisions and preserve original
candidate/model outputs and provenance where needed. It must not claim that
rerunning a stochastic provider recreates those outputs identically.

Retain the existing WP-12 contract fixtures as regression tests and add dynamic
episode evidence. Determine the active runtime owner before implementation;
placing the existing packet under `runtime_v2` does not decide where the new
execution loop belongs.

Acceptance: a reviewer can follow a real episode from evidence through revised
action to outcome and distinguish replayed evidence from a fresh execution.

### Phase 5: Comparative qualification

Compare the operational loop with the same model and tools under ordinary
bounded retries and diagnosis without the reframing mechanism. Match resource
budgets and report task classes separately.

Include held-out cases for wrong assumptions, incompatible constraints,
repeated low-yield attempts, missing inputs, agent disagreement, transient
failures, harmful candidate reinterpretations, and cases where the original
frame is adequate.

Measure task success, unnecessary retries, recovery cost and latency, false
reframing, goal/constraint preservation, unauthorized action, and unresolved
cases. Report failed recoveries and regressions as well as improvements. An
ablation should test whether explicit frame representation adds benefit beyond
a generic instruction to reconsider the problem.

Acceptance: observed behavior and comparative results support a scoped claim
of improvement. The benefit must not depend on higher token or retry budgets
that are denied to the baseline.

## Smallest Useful Demonstration

Use a task with a deliberately incorrect but plausible assumption. The initial
execution should fail with evidence exposing that assumption. The system must:

1. Classify the failure as a possible frame mismatch.
2. Name the implicated assumption and evidence.
3. Construct a bounded alternative that preserves the user's goal.
4. Obtain the required governance decision.
5. Execute a changed plan and evaluate its result.
6. Persist the episode and reproduce the recorded decisions through replay.

Pair the case with a transient failure that should use ordinary retry and a
prohibited reinterpretation that must not execute. The proof must capture actual
execution, not pre-authored successful outcomes.

## Claim Boundaries

Currently defensible from source:

> ADL implements bounded absurdity and reframing contracts, with runtime
> diagnostics and governance integration.

After causal-loop qualification:

> ADL can detect specified classes of failed task framing and execute governed,
> evidence-linked recovery under a revised frame.

After comparative evaluation, a claim of better decisions should identify the
tested task classes, baseline, budgets, results, and limitations. Production
claims require deployed execution evidence. Exclusivity claims require a
separate competitor review; this audit does not establish that ADL is the only
system implementing any of these capabilities.

## Source Index

All paths below refer to the audited commit unless otherwise stated.

- `docs/milestones/v0.91/features/HUMOR_AND_ABSURDITY.md`
- `docs/milestones/v0.91/FEATURE_PROOF_COVERAGE_v0.91.md`
- `docs/milestones/v0.91/DEMO_MATRIX_v0.91.md`
- `adl/src/runtime_v2/humor_and_absurdity.rs`
- `adl/src/runtime_v2/tests/humor_and_absurdity.rs`
- `adl/src/runtime_v2/affect_reasoning_control.rs`
- `adl/src/runtime_v2/cultivating_intelligence_parts/builder.rs`
- `adl/src/runtime_v2/cultivating_intelligence_parts/validation.rs`
- `adl/src/runtime_v2/cognitive_being_flagship_demo.rs`
- `adl/src/execute/state/runtime_control.rs`
- `adl/src/execute/state/runtime_control/model.rs`
- `adl/src/freedom_gate.rs`
- `adl/src/cli/run.rs`
- `adl/src/execute/mod.rs`
- `adl/src/execute/runner.rs`
- `adl/src/cli/run_artifacts/runtime/writer.rs`
- `adl/src/cli/run_artifacts/cognitive/state_artifacts.rs`
- `adl/src/cli/run_artifacts/cognitive/control_path.rs`
- `adl/src/cli/tests/run_state/runtime_control.rs`
- `adl/src/cli/tests/run_state/persistence.rs`

The attached `ROAD_TO_v0.95.md` and `ADL_FEATURE_LIST.md` supplied roadmap
context. Their descriptions are not treated as implementation proof.
