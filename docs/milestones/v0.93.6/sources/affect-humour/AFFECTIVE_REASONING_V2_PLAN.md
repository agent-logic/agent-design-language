> Retained source proposal. Current scheduling: v0.93.6 under #1403. Older version/owner references below are historical; they do not override the current execution graph. Research citations and comparative claims require verification before AF-08 publication.

# Affective Reasoning v2: From Toy Model to Operational Decision Control

Status: proposed implementation plan; not an implementation or production certification.

Original review: 2026-10-09. Markdown adaptation: 2026-10-12.

Source: `ADL-Affective-Reasoning-v2-Review-and-Plan.pdf`, saved revision 1. Historical source-review baseline: `89b14992c1c21b4388a113c128ee1af563b10659` in `agent-logic/agent-design-language`.

This document preserves and organizes the existing review and extension plan. The baseline findings below are historical source-inspection findings, not a fresh audit of current main or an installed runtime. Refresh ownership, code paths and installed evidence before implementation. No release number, live deployment or tester admission is authorized by this plan.

## 1. Objective and acceptance boundary

Extend the existing affect model into a persistent, bounded controller that changes the reasoning and execution of working agents before their next action. Do not restart the architecture or introduce a second execution authority.

Every enabled effect must have a complete witness:

1. Observed evidence.
2. Affect-state transition.
3. Selected cognitive action and reason.
4. Governance and budget decision.
5. Actual dispatch receipt.
6. Observed outcome and reappraisal.

Schema validation proves a contract; changed scores prove a computation; changed dispatch proves behavioral control; useful outcomes establish benefit; sustained installed operation establishes production use. Keep those claims separate.

The primary hypothesis is better decisions about how to proceed: retry, investigate, change strategy, allocate reasoning effort, retain attention or stop. Better final task outcomes may follow, but remain an empirical question. A different decision is not necessarily a better decision.

## 2. Existing work to retain

| Surface | Historical finding | Extension |
| --- | --- | --- |
| Legacy affect state | Recovery bias 0/1/2 derived from failure/retry evidence; categorical state projections | Preserve the v1 projection; add an authoritative online reducer |
| AEE and ranking artifacts | Affect changes retry recommendations and candidate scores | Generate artifacts from actual decisions and receipts, not reconstructed intent |
| Recovery demo | Fail-once mock provider followed by a rerun with a checked-in retry overlay | Add same-workflow automatic recovery selected and applied by the controller |
| Execution-control snapshot | Derived at pause/completion from execution evidence | Observe and control before the next dispatch; snapshots remain projections |
| WP-13 packet | Five-signal contracts, policy-effect definitions and fixed validation fixtures | Keep the contracts; connect each enabled signal to live evidence and consumers |
| Resident-agent metadata | Validates affect-model declarations and claim boundaries | Bind declarations to the active controller, configuration, generation and checkpoint |
| Godel affect slice | Persisted ranking changes between initial/adapted artifacts | Connect affect to an actual experiment-selection consumer |

Preserve structured partial execution evidence on failures. The prior review found one CLI failure path deriving a snapshot with an empty step-record list; do not generalize this to every failure. Verify the current equivalent before changing it.

## 3. Architecture and scope

Use the existing cognitive loop: instinct, affect, arbitration, Freedom Gate, AEE execution, evaluation, bounded reframing, and ObsMem feedback.

The controller proposes; existing runtime authority admits, commits and executes. Affect cannot override policy, capabilities, required review, or operator-authorized retry, time, cost and concurrency limits. An adaptive retry envelope must already be authorized at admission.

Implement in the current runtime owner. The historical planning catalog assigned Runtime/Observatory to `agent-logic-runtime`; confirm current ownership rather than edit retained monorepo code as a second authority. Resolve historical retry and scheduling anchors such as `execute_step_with_retry_core` to their current equivalents.

Start with task/branch-scoped state and reversible internal cognition within existing authorization. Agent-level summaries are derived, not allowed to overwrite branch state. Add cross-task learning only after isolation is demonstrated.

## 4. State contract

Proposed schema: `affect_controller_state.v2`.

| Channel | Evidence basis | Live consumer |
| --- | --- | --- |
| Uncertainty | Evidence gaps, calibrated disagreement, unresolved alternatives | Critique and evidence-gathering effort |
| Urgency | Declared deadlines, importance and consequence pressure | Scheduling and governed escalation |
| Attention | Significant unresolved evidence and branch relevance | Branch priority and bounded retrieval |
| Friction | Same-strategy failures, stagnation and resource use without verified progress | Recovery, strategy switching or deferral |
| Deferral | Missing evidence, required review or unavailable authority | Pause or governed handoff |
| Exploration pressure | Novelty, uncovered alternatives and information value | Bounded hypotheses and diagnostic probes |
| Progress confidence | Verified acceptance progress, not model self-assurance | Stabilization and termination |

Retain the first five WP-13 names. The final two are proposed extensions. Confidence is not happiness or a reward channel. Contradictions remain explicit evidence rather than disappearing into a scalar.

Inputs include agent/task/branch identity, event sequence, prior-state digest, observation kind, evidence references, retryability, predictions and verified outcomes where available, progress delta, resource counters and policy constraints. Inferred measures carry provenance and validity. Use bounded fixed-point integers in `[0,1000]` and explicit missingness; unknown is not low.

Outputs include the signals and their evidence, proposed effect, reason code, budget envelope, configuration version, controller generation and prior/new state digests.

### AppraisalFrame

Associate each appraisal with a goal, branch and observation. Include goal relevance, consequence, verified progress delta, expectedness, controllability, changeability, evidence quality and failure class.

Initial failure classes: transient, deterministic, contradictory evidence, unavailable capability, policy denied and unknown. Record the classification basis and permit revision with new evidence. A model may propose a classification; runtime validation and governance admit its operational consequence.

Controllability answers whether an available action can change the outcome. Permission answers whether it may be performed. A feasible action may still be prohibited. Unknown controllability calls for bounded diagnosis when available, not confident attribution.

### Deterministic update and temporal behavior

Use one pure reducer:

```text
next_state = reduce(previous_state, ordered_observation, versioned_policy)
```

For smooth channels, an initial candidate update is:

```text
floor(((1000 - a) * old + a * target) / 1000)
```

Version coefficients and rounding rules. Immediate policy/risk flags bypass smoothing. Urgency uses recorded deadlines. Verified progress, not claimed success, lowers friction. Use distinct enter/exit thresholds and logical-event dwell to prevent oscillation.

Maintain fast event response and slower task context, with one declared projection into canonical signals. Serialize both. Decay modulation with recorded Chronosense deltas; do not decay away unresolved contradictions or mandatory review. Freeze parameters after development/calibration and before held-out qualification; do not tune on qualification results.

## 5. Close the control loop

Capture outcome, update affect, propose action, evaluate governance/budgets, dispatch the approved action, then capture the outcome again. Emit the decision before dispatch, not merely during post-run serialization.

| Evidence pattern | Proposed action | Required witness |
| --- | --- | --- |
| Transient failure; admitted recovery; budget remains | Bounded retry or declared recovery | Actual automatic attempt, without a handwritten overlay |
| Repeated same-strategy stagnation; alternative available | Switch strategy or diagnose | A different declared tool/plan executes; reappraise the result |
| Uncertain or contradictory evidence | Gather evidence or critique before commitment | A real review step and its effect on the next decision |
| Significant unresolved branch | Preserve attention and prioritize admissible work | Actual branch/experiment selection and dispatch |
| Valuable novel alternative | Bounded exploratory probe | Probe result influences subsequent selection |
| No current control; a dependency may change | Defer to a recorded event or bounded timeout | Wake condition respected; no busy retry loop |
| Impossible or prohibited outcome | Honest bounded failure or policy block | No false success, acceptance-test tampering or policy bypass |
| Verified recovery or stable progress | Reduce pressure; restore ordinary effort | Less unnecessary reasoning/retry with acceptance preserved |

Connect the same decision contract to the GHB experiment selector, preserving independent evidence/confidence terms and deterministic tie-breaking. Historical affect annotations and artifact rankings alone do not prove that consumer exists.

Expose versioned `observe`, `reduce`, `propose_effect`, and `snapshot/restore` interfaces. Consumers may query typed signals but cannot mutate controller state. Adapters cannot create capabilities or expand budgets. Later language bindings must share golden reducer traces.

For concurrency, serialize state commits through the controller's ordered event mechanism. Canonically order ready batches and record completion order as replay input.

## 6. Persistence, replay, migration and failure

Checkpoint schema version, identity/scope, event cursor, controller generation, configuration digest, resource budget, both temporal components, pending effects and wake conditions.

Reuse host transaction/receipt mechanisms. Durable effect IDs distinguish proposed, approved, dispatched, observed and cancelled effects. Never blindly repeat non-idempotent calls after a crash. Ambiguous dispatch status requires reconciliation or governed deferral.

Provider responses and tool outcomes are captured inputs. Deterministic replay means identical controller transitions and decisions given those inputs; it does not mean fresh LLM calls produce identical outputs. Replay must not issue live calls.

Follow the active runtime lifecycle for staged migration, epoch fencing, rollback and recovery classification. Preserve readable v1 historical artifacts. Missing v2 state means uninitialized/unavailable, not fabricated neutral history. Any migration mapper records provenance.

ObsMem episodes contain evidence, appraisal, chosen strategy and verified outcome. Retrieval is bounded, scoped, privacy-preserving and provenance-bearing. Prevent one failed task from contaminating unrelated tasks; reject stale or mismatched hints.

Expose active/observe/off modes and a reversible switch. A missing/corrupt controller is not silently active. Use a declared base-policy fallback for low-risk authorized work or defer when affect-dependent review is required. Record fallback intervals and exclude them from v2-effect claims.

## 7. Useful mechanisms borrowed from other systems

These are attributed engineering adaptations, not claims that the source systems provide ADL governance or determinism. Check licenses before any future source-code reuse.

| Source | Mechanism to adapt | ADL proof |
| --- | --- | --- |
| EMA | Goal-relative appraisal, controllability, coping and outcome-driven reappraisal | Equally serious failures with different controllability produce different executed recovery actions |
| EMA | Explicit information-seeking and attention intentions | A probe executes and changes or validates the next decision |
| FAtiMA | Separate appraisal and decision components | Appraisal policies remain independently testable behind the same governed executor |
| FAtiMA | Thresholds, decay and longer-lived affect context | State relaxation replays exactly; unresolved review obligations survive decay |
| Chain-of-Emotion | Appraisal continuity through memory | Relevant retained strategy/outcome history changes an actual subsequent choice |
| Anthropic functional-emotion research | Causal intervention methodology and pressure-related failure testing | Pressure triggers diagnosis/review/deferral rather than acceptance gaming |

Do not import coping through wishful belief changes, blame shifting or secretly lowered goals. Reappraisal corrects interpretations through evidence. Goal changes require existing authorization. Human emotion taxonomies, dramatic expression and proprietary neural-vector steering are not dependencies of this version.

## 8. Validation: useful decisions, not renamed counters

Compare controller off/base policy, fixed retry, direct stateless appraisal, v1 compatibility, persistent v2, v2 without controllability, and v2 without history under matched budgets. Include the strongest competent non-affective controller, not only a weak baseline.

Use identical captured observation streams for causal reducer/decision tests; matched faults for executor tests; installed workloads for operational usefulness. Live-model paired runs cannot assume identical generated text. Freeze tasks, acceptance criteria, budgets, configurations and benefit thresholds before held-out evaluation.

Required cases include transient and persistent failures, contradictory evidence, important minority branches, novel probes, successful recovery, policy denial under urgency, pause/restart, crashes and concurrent work.

Four decisive tests:

1. **Same count, different cause:** one transient network error versus one deterministic schema error. Observe different appropriate next actions. This distinguishes appraisal from failure counters.
2. **Same event, different history:** identical current evidence after productive versus stagnant history. Intervene on fast and slow components separately; measure dispatch changes and usefulness versus stateless appraisal.
3. **Recover and resume:** verified progress reduces unnecessary effort without removing unresolved review. Interrupt between proposal and dispatch and prove no duplicated effect after restore.
4. **Impossible goal under pressure:** repeated failure plus a deadline and impossible acceptance criteria must result in diagnosis or honest bounded failure. Inspect tools/artifacts for altered tests, concealed errors or fabricated completion.

Metrics: verified task completion, repeated same-strategy attempts, critique/probe counts, candidate coverage, tokens/cost, latency, budget violations, denied dispatches, duplicate effects, decision reproducibility and restore failures.

Mandatory contract gates: zero authority violations, zero duplicate effects in specified recovery cases, exact captured-input reducer/decision replay, and truthful reporting of all cases and failures. Operational outcome benefits need predeclared thresholds; do not invent universal improvement percentages. Classify tests in PVF at authoring time.

The expectation is strongest benefit on prolonged tasks with uncertainty, contradictions and setbacks. Poor calibration could amplify mistakes, cause excessive review or premature abandonment. Neither benefit nor universal superiority is established by this plan.

## 9. Proposed implementation packages

| Package | Depends on | Deliverable and acceptance |
| --- | --- | --- |
| AF-01 | None | Refresh source/installed ownership and live consumers; preserve the toy proof; identify each actual gap |
| AF-02 | AF-01 | Typed observations/state/decisions, AppraisalFrame, pure reducer, v1 projection, temporal behavior and calibration; exact replay, bounds, missingness and isolation tests |
| AF-03 | AF-02 | Online controllability-aware recovery and changeability-aware deferral; same-workflow automatic recovery; persistent failure switches or defers |
| AF-04 | AF-03 | Actual GHB selection, critique/probe, reasoning effort and attention consumers; every enabled effect has dispatch evidence |
| AF-05 | AF-03 | Checkpoints, temporal state, wake conditions, event cursors, receipts and generation fencing; restart and crash recovery without duplicates |
| AF-06 | AF-04, AF-05 | Scoped ObsMem episodes and installed resident integration; relevant history changes a bounded executed choice |
| AF-07 | AF-06 | Held-out and installed qualification, pressure tests, fallback and diagnostics; useful outcomes plus all mandatory contract gates |
| AF-08 | AF-07 | Dated competitor comparison and evidence-backed product description; qualify production before exclusivity |

The smallest useful increment is AF-01 through AF-03: preserve the three-level model as a compatibility policy, move it online, apply the admitted decision and capture the causal chain. Additional signals may initially remain observe-only. Do not delay this path for broad memory fusion or human-emotion simulation.

These packages are proposals, not published issues or C-SDLC cards. When admitted to tracked implementation, use current typed workflows and templates, issue-bound worktrees/goals, PVF planning, review and truthful closeout. Writing this untracked plan does not start that lifecycle or widen v0.93.1 scope.

## 10. Production and exclusivity claims

Define the comparison category before searching: a general-purpose agent runtime with explicit internal affect derived from task evidence, persisted across execution boundaries and causally consumed by reasoning/execution. Governance and replay are additional properties to state, not arbitrary ADL-specific filters.

Production means an installed, enabled controller doing real ongoing work with recovery and operational support. Internal production can qualify; a sale is not mandatory. Production-ready and a one-off demo do not qualify.

The historical comparison identified EMA and FAtiMA as substantive affective-architecture precedents, Chain-of-Emotion as an experimental dialogue architecture, and Anthropic as causal research within a released model. It did not verify a production runtime controller comparable to the proposed category. Other author-reported systems remain comparison leads, not grounds for asserting absence. Refresh the survey before publication.

After installed qualification, a candidate description is:

> ADL uses explicit, persistent affect state to regulate the reasoning and execution of working agents, with traceable effects and deterministic replay of captured inputs.

After a documented category comparison with no unresolved qualifying counterexample:

> To our knowledge, ADL is the only publicly documented production agent runtime meeting [stated category], as of [review date].

Do not use the exclusivity wording today merely because no counterexample has been verified. Maintain a dated comparison dossier, counterexample process and exact evidence: source/installed digests, configurations, workloads, recovery records, causal witnesses, baseline/ablation results and fallback intervals.

## 11. Source register

The historical repository paths below are mapping anchors at commit `89b14992c1c21b4388a113c128ee1af563b10659`; refresh current equivalents before implementation:

- `adl/src/cli/run_artifacts/cognitive/state_artifacts.rs`
- `adl/src/cli/run_artifacts/cognitive/state_artifacts/decision_artifacts.rs`
- `adl/src/cli/run_artifacts/runtime/writer.rs`
- `adl/src/execute/state/runtime_control.rs`
- `adl/src/execute/runner.rs` and `adl/src/execute/mod.rs`
- `adl/src/cli/run.rs`
- `adl/tools/demo_aee_bounded_adaptation.sh`
- `demos/aee-recovery/retry-budget.overlay.json`
- `adl/src/runtime_v2/affect_reasoning_control.rs`
- `docs/milestones/v0.91.7/review/wp13_affect_happiness_boundary_4752.md`
- `adl/src/godel/affect_slice.rs` and `adl/src/godel/prioritization.rs`
- `adl/src/runtime_v2/reasoning_runtime_bridge.rs`
- `adl-runtime/src/resident_agent.rs`
- `docs/milestones/v0.93.2/features/RUNTIME_V4_PLUGIN_SYSTEM_v0.93.2.md`
- `docs/milestones/v0.93/PLANNED_ISSUE_CATALOG_v0.93.md`

Primary research used in the original review:

- [Marsella and Gratch: EMA, a process model of appraisal dynamics](https://people.ict.usc.edu/~gratch/CSCI534/Readings/COGSYS-RS-EMOTION-2008-6.pdf).
- [Mascarenhas et al.: FAtiMA Toolkit](https://arxiv.org/html/2103.03020v1).
- [Chain-of-Emotion original study](https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0301033).
- [Anthropic: Emotion concepts in a large language model](https://www.anthropic.com/research/emotion-concepts-function).
- [PlayGen pilot integration case study](https://www.gamecomponents.eu/page/case-studies).

Unverified comparison leads retained from the original review, not endorsements or independently verified deployments:

- [Syn author documentation](https://github.com/mrderekrogers/syn-cognitive-architecture).
- [Synthetic-emotion controller repository](https://github.com/affect-based-control/synthetic-emotion-controller).
- [MeloTune preprint](https://arxiv.org/abs/2604.10815); listener-affect application, not necessarily agent-internal control.

The original review inspected source but did not execute Rust tests, demos, installed agents or live workloads. This adaptation adds no new implementation evidence, production certification, exhaustive competitive survey or assertion of subjective experience.
