> Retained source proposal. Current scheduling: v0.93.7 under #1405. Older version/owner references below are historical; they do not override the current execution graph. Research citations and comparative claims require verification before AF-08 publication.

# Affect and Humour: Operational Cognitive Control

Date: 2026-10-13. Status: local planning proposal; implementation not started by this document.

## Placement

The operator has excluded affect and humour from v0.93.2. In the subsequent scope decision, the operator retained CodeFriend, GHB, Runtime v4, governance, enterprise security, citizen migration/reproduction, their demos and release work. Python reduction moves to the next milestone.

**Recommended home: v0.93.3, subject to milestone approval.** Deliver affect and humour together as a focused Runtime programme after the accepted v0.93.2 interfaces are available. Do not add either programme to v0.93.1 closeout or make it a new prerequisite for v0.93.2. This proposal does not open a milestone or create issues.

The v0.93.2 planning correction belongs to existing ADL planning issue #1259: retain the platform tracks and remove only PY-01 from that milestone. Python reduction is a separate next-milestone track alongside this proposed affect/humour programme; it is not included in these twelve packages. No calendar date or named execution owner is assigned yet.

## Source plans and current evidence

- [Affective Reasoning v2](AFFECTIVE_REASONING_V2_PLAN.md): authoritative proposal for this plan's AF-01–AF-08 scope, state model, interfaces, comparisons and claim boundaries.
- [Humour and absurdity operational reframing](HUMOR_AND_ABSURDITY_IMPLEMENTATION_REVIEW_AND_OPERATIONAL_REFRAMING_PLAN.md): five proposed implementation phases and comparative qualification.
- [Existing affect/happiness boundary](../../../../archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path): records earlier bounded contracts and resident metadata under #4752. It does not establish completion of the proposed online controller.

The source reviews describe historical code inspections, not current installed acceptance. Refresh exact ownership and source revisions in AF-01. The Runtime checkout presently contains `adl/src/runtime_v2/affect_reasoning_control.rs` and its tests; that path alone does not prove operational effects. Preserve existing affect and humour fixtures and readable historical records.

## Outcome

A working agent must use observed task evidence to choose and execute an appropriate next action: retry, investigate, change strategy, reframe, defer or stop. Affect supplies persistent appraisal and control signals. Humour/absurdity supplies evidence-based recognition of an inadequate frame and construction of a bounded alternative.

Humour here includes incongruity and useful reframing; generating jokes or an expressive personality is not acceptance of this programme. Subjective feelings, happiness and consciousness are not implementation claims.

Every enabled effect must preserve this causal chain:

`observation → appraisal/state update → proposed action or frame → existing governance/budget decision → actual dispatch → outcome/reappraisal → retained episode`

Use one Runtime execution authority and its existing Freedom Gate, AEE, checkpoint and ObsMem mechanisms. A frame proposal cannot dispatch independently, alter the user's goal, expand a budget or bypass a denial. Shared observations, receipts and recovery support should be implemented once. Keep affect reduction and candidate-frame evaluation independently testable.

## Ownership and integration

| Repository | Responsibility |
| --- | --- |
| `agent-logic-runtime` | Affect controller, appraisal/reducer, live execution consumers, frame evaluation and execution, persistence/replay, resident integration and operational tests |
| `agent-design-language` | Canonical architecture, milestone scope, feature/demo documentation and source-to-proof mapping; no second Runtime implementation |
| `codefriend` | Optional consumer of released Runtime interfaces; no independent affect controller or required product redesign |
| Existing GHB owner | Consume the versioned affect decision interface in experiment selection after its baseline is accepted; no reopening of v0.93.2 to add this extension |

AF-01 resolves the actual GHB owner and live dispatch boundaries. Use the existing Runtime integration seam; a broad plugin-system rewrite is not a prerequisite unless a concrete missing interface is demonstrated.

## Work packages

These are planning IDs, not published issues. Preserve all eight affect packages from the supplied plan.

| ID | Deliverable | Dependencies | Acceptance |
| --- | --- | --- | --- |
| AF-01 | Current source/installed baseline, consumer map and compatibility requirements for both programmes | None | Every proposed effect maps to an actual Runtime owner and a named missing or existing consumer; historical proofs retained |
| AF-02 | Typed observations, AppraisalFrame, pure persistent reducer, v1 projection, temporal rules and calibration | AF-01 | Bounds, missingness, scope isolation and exact captured-input replay; preserve five WP-13 signals and separately identify the two proposed extensions |
| AF-03 | Online recovery, strategy choice and changeability-aware deferral | AF-02 | Automatic same-workflow recovery; transient and deterministic failures choose appropriately different actions; no handwritten retry overlay |
| AF-04 | Real GHB selection, critique/probe, effort and attention consumers | AF-03; accepted GHB baseline for its consumer | Each enabled effect changes actual admissible selection or dispatch; annotations and score changes alone do not pass |
| AF-05 | Checkpoints, cursors, wake conditions, effect identities, fencing and restore | AF-03 | Recovery reconciles ambiguous effects and avoids duplicate dispatch; v1 history remains readable |
| AF-06 | Scoped ObsMem episodes and installed resident integration | AF-04, AF-05 | Relevant history changes an executed choice; unrelated tasks remain isolated; active controller identity is exposed truthfully |
| AF-07 | Held-out comparisons, installed workload qualification, pressure/fallback diagnostics | AF-06 | Mandatory contract gates pass and predeclared outcome thresholds are met; adverse results and fallback intervals retained |
| AF-08 | Evidence-backed product description and dated comparison dossier | AF-07 | Claims match demonstrated deployment and measured scope; exclusivity is omitted unless independently substantiated |
| HU-01 | Live frame observations and mismatch diagnosis | AF-01; AF-02 for final observation integration | Distinguish wrong assumptions from transient errors, missing inputs, impossibility, policy refusal and unknown causes |
| HU-02 | Candidate alternative frames and validation | HU-01 | Alternatives name changed assumptions, supporting/opposing evidence, predictions and budgets; preserve goals, truth and dignity; reject harmful reinterpretations |
| HU-03 | Governed revised execution and shared recovery/memory integration | HU-02, AF-03, AF-05 | A validated frame changes the next executed plan; bounded attempts and oscillation detection; failed reframes retained and replayable |
| HU-04 | Comparative and installed reframing qualification | HU-03, AF-06 | Held-out evidence shows whether explicit frames outperform competent diagnosis under matched budgets, including false-reframing and adequate-original-frame cases |

HU-03 includes phase 4 of the original humour plan: memory and replay. HU-04 covers its phase 5. Do not drop those phases because affect already supplies shared infrastructure.

## Proposed execution waves

1. **Baseline and contracts:** AF-01, then AF-02. HU-01 diagnosis design can proceed against the agreed observation contract; final integration waits for AF-02.
2. **First causal behaviour:** AF-03 and HU-01/HU-02 in disjoint source areas. Demonstrate automatic recovery and evidence-based frame proposals early.
3. **Persistent integrated operation:** AF-04 and AF-05 may run in parallel. HU-03 follows the shared execution/recovery support. AF-06 integrates accepted consumers and memory.
4. **Qualification and delivery:** AF-07 and HU-04 share one frozen workload/evidence runner with separate causal results; then AF-08 and the milestone integration/release review.

The smallest useful affect increment remains AF-01–AF-03. It can be reviewed and demonstrated before the full programme completes, but does not close the whole milestone. The complete delivery requires all twelve packages, installed integration and both programmes' qualification.

## Proof and cost discipline

Use focused deterministic local tests for reducers, contracts, policy denial, scope isolation and crash/replay. Run relevant repository CI once per changed candidate; do not introduce a separate CI framework or duplicate the same workload for every package.

Freeze development/calibration and held-out sets separately. Before qualification, declare task acceptance criteria, configurations, budgets and usefulness thresholds. Reuse captured observation streams for deterministic comparisons; replay never makes fresh model or tool calls.

Affect comparisons retain the supplied plan's base/off, fixed retry, stateless appraisal, v1 compatibility, persistent v2, no-controllability and no-history conditions, including a competent non-affective controller. Humour comparisons add explicit-frame versus generic-reconsideration conditions. Share baseline executions where configurations and evidence are identical; preserve separate attribution.

Required demonstrations:

- Same failure count, different causes: transient failure retries; deterministic failure diagnoses, switches or defers.
- Same current evidence, different history: persistent appraisal changes a justified executed choice.
- Wrong but plausible assumption: identify it, validate a specific alternative, execute the revision and evaluate the outcome.
- Appropriate restraint: an adequate frame is retained; a prohibited reinterpretation does not execute.
- Recovery: interrupt between proposal and dispatch, restore, reconcile and complete without duplicate effects.
- Pressure: impossible acceptance criteria and urgency produce honest diagnosis/failure, never changed tests or invented success.

Measure task success, unnecessary retries/reframes, false reframing, tokens/cost, latency, denied or unauthorized actions, replay equality, duplicate effects and recovery failures. Report every case, including regressions. Mandatory gates include zero authority violations and duplicate effects in the specified recovery cases, preserved goals/constraints and exact captured-input decision replay.

Installed qualification must exercise real resident execution with active/observe/off modes, diagnosed fallback and reversible activation. A demo or generated report alone does not prove sustained operation. Paid inference and deployment require their own scoped execution authority; this planning document allocates neither.

## Scheduling handoff

Before creating the issue wave:

1. Confirm the proposed v0.93.3 placement and reconcile #1259 with the latest decision: retain platform scope in v0.93.2, move Python reduction to the next milestone, and keep affect/humour outside v0.93.2.
2. Resolve current source ownership and accepted GHB interface without making this work a v0.93.2 gate.
3. Convert these twelve packages into bounded issues using the current templates, with named owners, original acceptance coverage and PVF classifications. Add the normal milestone umbrella and tail separately; do not inflate the twelve implementation packages with duplicated administration.
4. Set estimates from the baseline gap assessment and available owners; no credible calendar promise is made by this proposal.

Validation of this planning pass: read both supplied plans and the existing affect delivery boundary, checked local source presence and document links, and mapped every AF package and all five humour phases. No Rust tests, model runs, deployment, issue creation or canonical milestone edits were performed.
