# v0.93.7 Sprint

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.7. Date: 2026-10-30. Setup amendment: [#1405](https://github.com/agent-logic/agent-design-language/issues/1405). Planning owner: Planning #12; implementation owners assigned at sprint activation.

## Status

First-pass successor plan; not opened for implementation or release. Planning only. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized. Create child issues/cards per activated sprint; current authenticated C-SDLC remains authority until separately accepted cutover.

## How To Use

Read the WBS and execution graph together; qualified milestone/ID pairs are identities. Preserve dependencies and adverse cases; reuse existing owner issues. Plan review does not establish product proof.

## Sprint Overview

Affect and humour. 25 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Retain the original integrated qualification and final review/release tail. Record accepted product lockset, limitations and unscheduled work; another milestone requires its own decision.

## Sprint Goals

### Sprint 1: Affect and humour

WP-01, AF-01, AF-02, AF-03, AF-04, AF-05, AF-06, AF-07, AF-08, HU-01, HU-02, HU-03, HU-04.

### Sprint 2: Integrated qualification

INTEGRATE, QUALIFY.

### Sprint 3: Review and release

TAIL-01, TAIL-02, TAIL-03, TAIL-04, TAIL-05, TAIL-06, TAIL-07, TAIL-08, TAIL-09, TAIL-10.

## Sprint Goal

Affect and humour. 25 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Retain the original integrated qualification and final review/release tail. Record accepted product lockset, limitations and unscheduled work; another milestone requires its own decision.

## Planned Scope

| WP | Sprint | Repository | Outcome | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | 1 | agent-design-language | Consume accepted governance, GHB and Runtime interfaces, then activate only AF-01..08 and HU-01..04 with their integrated qualification and existing final review/release tail. | v0.93.6/GOV-CLOSE |
| AF-01 | 1 | agent-logic-runtime | Every proposed effect maps to an actual Runtime owner and a named missing or existing consumer; historical proofs retained | WP-01, v0.93.3/RV-08 |
| AF-02 | 1 | agent-logic-runtime | Bounds, missingness, scope isolation and exact captured-input replay; preserve five WP-13 signals and separately identify the two proposed extensions | AF-01 |
| AF-03 | 1 | agent-logic-runtime | Automatic same-workflow recovery; transient and deterministic failures choose appropriately different actions; no handwritten retry overlay | AF-02 |
| AF-04 | 1 | agent-logic-runtime | Each enabled effect changes actual admissible selection or dispatch; annotations and score changes alone do not pass | AF-03, v0.93.5/GHB-WP12 |
| AF-05 | 1 | agent-logic-runtime | Recovery reconciles ambiguous effects and avoids duplicate dispatch; v1 history remains readable | AF-03 |
| AF-06 | 1 | agent-logic-runtime | Relevant history changes an executed choice; unrelated tasks remain isolated; active controller identity is exposed truthfully | AF-04, AF-05 |
| AF-07 | 1 | agent-logic-runtime | Mandatory contract gates pass and predeclared outcome thresholds are met; adverse results and fallback intervals retained | AF-06 |
| AF-08 | 1 | agent-logic-runtime | Claims match demonstrated deployment and measured scope; exclusivity is omitted unless independently substantiated | AF-07 |
| HU-01 | 1 | agent-logic-runtime | Distinguish wrong assumptions from transient errors, missing inputs, impossibility, policy refusal and unknown causes | AF-01, AF-02 |
| HU-02 | 1 | agent-logic-runtime | Alternatives name changed assumptions, supporting/opposing evidence, predictions and budgets; preserve goals, truth and dignity; reject harmful reinterpretations | HU-01 |
| HU-03 | 1 | agent-logic-runtime | A validated frame changes the next executed plan; bounded attempts and oscillation detection; failed reframes retained and replayable | HU-02, AF-03, AF-05 |
| HU-04 | 1 | agent-logic-runtime | Held-out evidence shows whether explicit frames outperform competent diagnosis under matched budgets, including false-reframing and adequate-original-frame cases | HU-03, AF-06 |
| INTEGRATE | 2 | agent-logic-runtime | One installed polis joins citizen action, upstream delegation, private cognition, policy enforcement, appeal and plugin restart without contract drift. | v0.93.6/DEMO-GOV, v0.93.2/DEMO-SEC, v0.93.4/CM-04, v0.93.5/GHB-WP12, AF-08, HU-04 |
| QUALIFY | 2 | agent-logic-runtime | Independent reviewer runs the accepted success/failure/recovery denominator on the exact multi-repository lockset. | INTEGRATE |
| TAIL-01 | 3 | agent-design-language | Deliver the complete quality gate result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | QUALIFY |
| TAIL-02 | 3 | agent-design-language | Deliver the complete documentation review and external-review handoff result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-01 |
| TAIL-03 | 3 | agent-design-language | Deliver the complete publication finalization result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-02 |
| TAIL-04 | 3 | agent-design-language | Deliver the complete internal milestone review result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-03 |
| TAIL-05 | 3 | agent-design-language | Deliver the complete external or third-party review result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-04 |
| TAIL-06 | 3 | agent-design-language | Deliver the complete accepted-findings remediation or explicit deferral capture result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-05 |
| TAIL-07 | 3 | agent-design-language | Deliver the complete next-milestone planning result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-06 |
| TAIL-08 | 3 | agent-design-language | Deliver the complete next-milestone closeout planning result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-07 |
| TAIL-09 | 3 | agent-design-language | Deliver the complete next-milestone planning review result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-08 |
| TAIL-10 | 3 | agent-design-language | Deliver the complete release ceremony and milestone close result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-09 |

## Work Plan

### Sprint 1: Affect and humour

WP-01, AF-01, AF-02, AF-03, AF-04, AF-05, AF-06, AF-07, AF-08, HU-01, HU-02, HU-03, HU-04.

### Sprint 2: Integrated qualification

INTEGRATE, QUALIFY.

### Sprint 3: Review and release

TAIL-01, TAIL-02, TAIL-03, TAIL-04, TAIL-05, TAIL-06, TAIL-07, TAIL-08, TAIL-09, TAIL-10.

## Execution Policy

One accountable owner per admitted issue; native bind/start and issue-bound goal precede implementation. Independent review precedes a PR; merge only green at the reviewed head. Preserve source and evidence; administrative reconciliation and optional cleanup may run asynchronously. Do not create a second closeout PR or repeat tests/reviews merely to update records.

## Cadence Expectations

Dependency-driven waves; no calendar promises or implied parallel staffing. Parallelize only disjoint write sets with accepted interfaces; do not duplicate active work.

## Risks / Dependencies

Accepted predecessor inputs and compatibility. Provider/experiment costs require scoped approval. Missing proof is a named gap; no retries or new machinery merely to satisfy records.

## Demo / Review Plan

[Proof matrix](DEMO_MATRIX_v0.93.7.md); PVF planning_contract for this setup: registry headings, JSON/YAML mirrors, source/link validity, complete allocation and acyclic qualified dependencies. Product work must later prove each acceptance and negative case through the owning installed consumer; schema, source or CI alone is not live qualification. Reuse unchanged exact-input evidence; no broad local Rust or paid qualification campaign for planning.

## Closeout Bar

One accountable owner per admitted issue; native bind/start and issue-bound goal precede implementation. Independent review precedes a PR; merge only green at the reviewed head. Preserve source and evidence; administrative reconciliation and optional cleanup may run asynchronously. Do not create a second closeout PR or repeat tests/reviews merely to update records.

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
