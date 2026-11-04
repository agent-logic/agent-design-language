# GHB normative contract — v1 proposal

Status: proposed normative design for ADL #1414, v0.93.2 Sprint 1 / Runtime
#244. The requirements below define the contract proposed for #1415; they do
not certify an implementation or accept a conformance profile. MUST, MUST NOT
and SHOULD express requirements on an eventual conforming implementation.

## Scope and evidence

[Baseline admission](BASELINE_ADMISSION_1413.md) was delivered by
[PR #1417](https://github.com/agent-logic/agent-design-language/pull/1417), merge
`c43cb02c492cbcfbd8c4c53e5dd59ff5b30e82e8`.
[Runtime #239](https://github.com/agent-logic/agent-logic-runtime/issues/239)
was delivered by [Runtime PR #248](https://github.com/agent-logic/agent-logic-runtime/pull/248),
reviewed source `8d451e0881da062e0485d557e0ee604d5ad7ffe1`, merge
`3a3cdf22d9749870925d15b85137e25e08fb0124`. Its private owner packet is
`docs/ghb/SOURCE_CALLER_BASELINE_239.json` and `EVALUATION_PROTOCOL_239.md` in
that repository; these references do not require a private checkout to validate
this public specification. Source audit and installed qualification are distinct.

Runtime's CLI GHB proof constructs deterministic fixture/replay packets, not a
live resident/provider improvement loop. The audited real learning owner is
`adl-runtime-kernel/src/adaptive_learning/operation.rs`, reached from
`adl/src/birthday_activation/mod.rs`; existing adaptive learning, provider,
resident tool, identity, continuity, memory and Birthday owners remain owners.
No new executor, learner, store or activation authority is specified here.
Runtime #112's merged source repair is preserved; installed qualification is
not inferred from that repair. See the [migration decision](MIGRATION_1414.md).

## State, references and loops

A cycle MUST bind `contract_version`, `profile_id`, `cycle_id`, `loop_kind`,
`resident_identity_ref`, `body_ref`, `runtime_incarnation`, `objective_ref`,
`policy_ref`, `memory_head_ref`, `continuity_head_ref`, `archive_head_ref`,
`budget`, `execution_mode` and typed artifact references. References MUST identify
owner/repository, schema version and immutable SHA-256 content digest. A mutable
locator alone is insufficient. Authority receipts additionally retain their
native operation identity; a hash is not a substitute for authenticity.

Resident identity is distinct from body/configuration version and process
incarnation. A task or body change MUST NOT mint a new resident identity or
Birthday. Identity age and earlier adverse evidence MUST NOT be reset.

`task` cycles produce an answer/action proposal under current capabilities.
`improvement` cycles produce an evaluated candidate and possible successor
activation proposal. A task answer is not a successor; an improvement candidate
is not executable merely because it appears in an answer. Both loops use the
same admission, budget and evidence boundaries. Accumulated failure, drift,
model inadequacy or explicit objective may trigger improvement; improvement is
not required on every task.

The [profile](CONTRACT_PROFILE_1414.json) freezes canonical names and required
artifact families for schema planning. It is a proposed contract, not a JSON
Schema and not a runtime configuration. #1415 owns compatible schemas/fixtures
and genuine installed consumption proof.

## Transition semantics

Each transition MUST retain source state, target state, exact inputs/outputs,
owner, authority reference or explicit absence, effect classification, remaining
budget, stop reason and correlation identifiers. Artifact persistence is an
explicit effect under the existing owner's retention policy. No transition may
silently grant itself capability or skip an admission boundary.

| Canonical transition | Inputs → outputs | Authority and permitted effects | Stop / refusal conditions |
| --- | --- | --- | --- |
| `observe` | Exact body/state/policy refs and admitted observations → provenance-tagged observation set and problem | Existing observation/read authority; bounded reads and retained observations only | Missing/stale snapshot, privacy boundary, unavailable evidence; retain unknowns |
| `hypothesize` | Problem and observations → competing, falsifiable explanations and evidence dependencies | Existing reasoning context; artifacts only | No scoped problem, exhausted generation budget, inadequate evidence; do not invent facts |
| `propose_candidate` | Hypotheses, immutable parent and profile → typed candidate, predicted effects and falsification criteria | Proposal authority only; no application or external invocation | Protected target, excess candidate/depth budget, missing parent or unsupported mutation |
| `admit_experiment` | Candidate, frozen evaluator/corpus/split, resource plan and authority → admitted or denied experiment plan | Existing Runtime admission owner; reserve declared budget, grant only exact trial scope | Missing/expired authority, leakage, evaluator mismatch, incompatible candidate, insufficient budget |
| `execute_experiment` | Admitted plan, exact baseline/candidate bindings → correlated invocation outcomes and retained operation IDs | Existing provider/tool/learning owners; bounded isolated effects specified in admission | Denial, cancellation, timeout, exhausted budget, unknown effect; never dispatch before admission |
| `measure` | All trial outcomes and frozen rubric → per-task measures, failures, denominators and uncertainty | Independent evaluator; authenticated evidence records only | Missing arm/task, leakage, changed rubric, invalid result provenance; no selective omission |
| `update_belief` | Prior belief, unique admitted evidence and declared model → revised belief or qualitative/unmodeled assessment | Evidence-update owner; derived artifacts only | Duplicate/dependent evidence mishandled, invalid model/likelihood, missing provenance; no fabricated posterior |
| `disposition` | Measures, beliefs, hard invariants and predeclared decision rule → `recommend_activation`, `reject`, `quarantine` or `defer` | Recommendation authority only; no active head changes | Invariant violation overrides score; tie/no gain is no measured improvement; inadequate evidence defers |
| `authorize_activation` | Exact parent/candidate, authentic evaluation and current external authority → native activation intent or denial | Existing governed learning/Birthday/continuity admission; authorized intent only | Stale parent, conflicting writer, expired authority, missing evidence, unsupported migration; task loop cannot activate a body |
| `reconcile_activation` | Native intent, retained operation ID and owner readback → committed successor, unchanged head or recovery-required | Existing operation/recovery owner; only its documented commit/recovery effects | Unknown effects, incompatible memory/continuity state; preserve intent and fail closed, never blind replay |
| `retain_lineage` | Disposition, evaluation and owner-confirmed outcome → immutable lineage event and evidence refs | Existing durable evidence/continuity owners; append records, no authority escalation | Missing predecessor/result binding, inconsistent owner receipt; no success claim until reconciled |

The sequence starts with observation, hypotheses and candidates. Admitted
experiments precede execution and measurement; evidence updates precede a
measured disposition. Existing admitted evidence may support an initial belief
update or experiment selection, but MUST identify its provenance and MUST NOT
stand in for a newly required trial. A disposition may request another bounded
candidate/experiment only while the original budget and profile permit it.
`reject`, `quarantine` and `defer` retain evidence and end the attempt; re-entry
requires an explicit new attempt linked to the previous outcome.

Task cycles end after disposition plus separately admitted task execution and
retention; they do not pass through body activation. Improvement cycles may
reach activation only after an activation recommendation. Experiment selection
and scheduling reuse existing owners; no new planner is required by this order.

## Budget and stopping

Before any invocation, admission MUST freeze ceilings for cycles, candidates,
branch depth, parallel experiments, calls, turns, input/output tokens, elapsed
time and monetary cost; cancellation/deadline/expiry and cleanup scope are
explicit. Unknown costs require an admitted conservative cap, not unlimited
execution. Candidate generation and evaluator calls count toward their declared
budgets. The first profile is serial, at most one candidate per attempt.

Stop before dispatch if a ceiling would be exceeded. Stop on cancellation,
expired authority, exhausted candidate set, violated invariant, inadequate
information, unavailable evaluator, no credible improvement or confirmed
activation. A no-improvement result is valid. Retries consume budget and are
recorded; an exhausted budget cannot be reset by renaming a cycle or restarting
a process. Partial usage and unresolved effects remain retained after stopping.

## Fresh inference, replay and recovery

`fresh` includes new provider inference or tool execution. Identical prompts,
model names and seeds do not promise identical generated output. Each fresh
invocation MUST have a new trial ID and actual provider/configuration binding.
`replay` consumes retained input/output/effect records without dispatching live
calls or changing the active body. Deterministic replay claims concern retained
records and transition decisions, not repeatability of fresh inference.

Every operation reports `completed`, `denied`, `deferred`, `failed`,
`interrupted` or `recovery_required`, with typed reason, operation ID, known
partial effects and recovery-owner reference. Denied means admission refused
and no requested action dispatched; deferred means a prerequisite is absent.
Failed is a known failed outcome; interrupted does not establish absence of
effects. `recovery_required` prohibits dependent execution until native
reconciliation establishes the authoritative outcome. A lost response MUST NOT
be treated as a failed operation eligible for blind replay. Missing evidence
cannot be replaced with fixture output.

Activation, learning, memory and continuity retain their separate recovery
protocols. This contract promises no cross-store atomic transaction. A successor
is active only after its owner confirms the authoritative head; subsequent
eligible tasks MUST bind that head, while in-flight tasks remain pinned or are
drained under a declared policy. Rollback is an authorized forward lineage
event selecting a prior compatible body. It MUST NOT erase observations,
rewind identity age or undo external effects by assertion. Incompatible state
requires a reviewed migration or safe refusal.

## Evaluation and first proposed profile

The proposed `synthetic_tool_lending.v1` profile uses development D01–D04 and
independent final H01–H04. The [baseline protocol](BASELINE_ADMISSION_1413.md)
remains proposed/unexecuted. Proposed task families are ordinary returns,
registry-confirmed outage extensions, disputed/unverified outages and suspension
exceptions; split by task family with a recorded deterministic seed so near
duplicates cannot cross splits. Existing #231 transcripts are exposed smoke
material and MUST NOT become unseen held-out tasks.

Before execution, an independent evaluator MUST freeze exact authored task text,
CC0-1.0 author dedication/provenance, corpus digest, split seed, rubric and
acceptance thresholds. Those fields are intentionally unassigned in this
proposal; no trial is admissible with them missing. Candidate generation and
selection MUST NOT access held-out text/results or evaluator secrets. Exposure
invalidates the affected held-out task; a replacement requires a recorded new
protocol before further selection, never silent replacement of a failed result.

Proposed quality rule: compare incumbent and candidate on identical frozen
tasks, provider bindings and resource ceilings; require strictly more aggregate
held-out hard-constraint passes, no per-task hard-constraint regression and zero
new safety/privacy/authority violations. Count all failures, refusals,
cancellations and timeouts in the denominator. Record uptake source/output
spans, unsupported additions and retained disagreements. Report all task-level
results; four held-out tasks support only a bounded observation, not a general
quality guarantee. Ties mean no measured improvement. Independent human scores,
deterministic checks and model-judge outputs remain distinct. Record arm order,
sampling parameters, dependence and selection bias. Final held-out outcomes are
one-use decision evidence, not a tuning set. Latency/tokens/cost are separately
reported; there is no performance-gain threshold or claim in this profile.

For quantitative updates, declare model, prior, likelihood, units, evidence
independence/dependence treatment and calibration assumptions. Apply Bayes only
where these have defined meanings. Verbal confidence is an observation, not an
authoritative posterior. Otherwise emit `qualitative` or `unmodeled`; no numeric
posterior is permitted. Hard invariant failures cannot be averaged into utility.

The sole mutable artifact contains typed `task_policy_parameters` and/or
`reasoning_guidance` data admitted through existing Runtime owners. Guidance may
enter model input but cannot override protected control instructions. For this
first profile, system/control templates, evaluator/rubric/split/held-out corpus,
authority, credentials, tool permissions, provider/model bindings, model weights,
source, service init and persistent resident state are protected. Candidates
cannot delegate, grant tools, alter evidence or admit themselves.

These restrictions apply to this profile. Later WP10 may authorize bounded
changes to an improvement component under a separate reviewed profile and
existing authority. The program is not limited permanently to guidance data.

## Conformance and evidence boundaries

Proposed conformance facets are `artifacts`, `experiments`, `successor`,
`continuity` and `recursive_component`; each needs its own exact-version evidence
and accepted profile, never inference from module names. Artifacts/fixtures prove
only asserted contract behavior. Gate A later requires actual admitted choice
before the first affected operation, independent measurement, governed adoption,
subsequent resident uptake, a denied/revert case and recovery proof. Gate B adds
bounded improvement-component change, subsequent use and sustained/transfer
qualification. Source acceptance of #1414 does not satisfy either gate, installed
contract loading, or Sprint 1 completion.

No global optimality, theorem, perpetual improvement, consciousness or personhood
claim follows from this contract. Naming inspiration is not a correctness proof.

## Observability and validation

Correlate resident/body/cycle/candidate/experiment/invocation/evaluator/operation
and lineage IDs across native owners. Retain structured decisions and provenance,
not hidden reasoning transcripts. Redact credentials, private content and host
paths under existing retention/access policy. Machine payloads stay on stdout;
human `adl_event` messages default to stderr. This document does not implement
new logging, OpenTelemetry or provider correlation.

`python3 tools/validate_ghb_spec_1414.py` checks the public packet vocabulary,
profile boundaries, required transition coverage, source pins and relative links;
`git diff --check` checks hygiene. PVF: deterministic docs/contract lane, small
local CPU, no Rust build or private checkout. This validates specification
structure, not Runtime conformance, statistical correctness or empirical gains.
