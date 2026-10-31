# v0.93.6 Release Notes

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.6. Date: 2026-10-30. Setup issue: [#1403](https://github.com/agent-logic/agent-design-language/issues/1403). Planning owner: Planning #12. Implementation owners assigned at sprint activation.

## How To Use

Read the WBS and execution graph together; qualified milestone/ID pairs are identities. Preserve dependencies and adverse cases; reuse existing owner issues. Plan review does not establish product proof.

## Summary

Governance, affect and humour. 38 planned work-package identities across 4 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Remaining governance and all twelve affect/humour packages follow the accepted GHB baseline. Original integration/qualification and ten tail identities remain here.

## Highlights

Governance, affect and humour. 38 planned work-package identities across 4 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Remaining governance and all twelve affect/humour packages follow the accepted GHB baseline. Original integration/qualification and ten tail identities remain here.

## What's New In Detail

Planned scope only:

| WP | Sprint | Repository | Outcome | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | 1 | agent-design-language | Consume accepted GHB and Runtime interfaces and activate only the next bounded remaining-governance/affect/humour sprint. | v0.93.5/GHB-CLOSE |
| GOV-04 | 1 | agent-logic-runtime | A review consumer emits findings over trace/outcome/policy evidence without raw private-state access. | v0.93.5/GOV-03 |
| GOV-05 | 1 | agent-logic-runtime | A challenged finding progresses through appeal and recorded disposition without destroying original evidence. | GOV-04 |
| GOV-06 | 1 | agent-logic-runtime | Signed evidence-backed model updates replay under standing, consent/scope and projection controls. | v0.93.5/GOV-01 |
| GOV-07 | 1 | agent-logic-runtime | Conflicting and stale entries change a bounded consumer decision through explicit uncertainty/decay events. | GOV-06 |
| GOV-08 | 1 | agent-logic-runtime | Relationship records preserve evidence and permitted scope across restart without becoming standing verdicts. | GOV-06 |
| GOV-09 | 1 | agent-logic-runtime | Authorized redacted projections can inform review while retaining challenge and source lineage. | GOV-05, GOV-07, GOV-08 |
| GOV-10 | 1 | agent-logic-runtime | Two governed consumers share evidence projections with preserved uncertainty, access rules and freshness. | GOV-09 |
| GOV-12 | 1 | agent-logic-runtime | A selected/rejected upstream call retains citizen identity, policy, provenance and verification; ToM may inform selection but cannot override authority. | v0.93.5/GOV-11, GOV-07 |
| GOV-13 | 1 | agent-logic-runtime | A governed exchange succeeds while an associated unauthorized private-state/ToM inspection is denied. | v0.93.5/GOV-11 |
| GOV-14 | 1 | agent-logic-runtime | A policy consumer evaluates explicit polis/citizen obligations and emits reviewable evidence. | GOV-05, GOV-10, GOV-13 |
| GOV-15 | 1 | agent-logic-runtime | Guild identity, membership, scoped resources and delegated actions persist with trace and challenge support. | GOV-14, GOV-12 |
| GOV-16 | 1 | agent-logic-runtime | The operator receives evidence-linked standing, appeals and social-memory summaries without private-state leaks or scalar moral ranking. | GOV-15 |
| DEMO-GOV | 1 | agent-logic-runtime | Execute D1-D8 and guild D13 through installed Runtime consumers with accepted and refused paths. | GOV-16 |
| AF-01 | 2 | agent-logic-runtime | Every proposed effect maps to an actual Runtime owner and a named missing or existing consumer; historical proofs retained | WP-01, v0.93.3/RV-08 |
| AF-02 | 2 | agent-logic-runtime | Bounds, missingness, scope isolation and exact captured-input replay; preserve five WP-13 signals and separately identify the two proposed extensions | AF-01 |
| AF-03 | 2 | agent-logic-runtime | Automatic same-workflow recovery; transient and deterministic failures choose appropriately different actions; no handwritten retry overlay | AF-02 |
| AF-04 | 2 | agent-logic-runtime | Each enabled effect changes actual admissible selection or dispatch; annotations and score changes alone do not pass | AF-03, v0.93.5/GHB-WP12 |
| AF-05 | 2 | agent-logic-runtime | Recovery reconciles ambiguous effects and avoids duplicate dispatch; v1 history remains readable | AF-03 |
| AF-06 | 2 | agent-logic-runtime | Relevant history changes an executed choice; unrelated tasks remain isolated; active controller identity is exposed truthfully | AF-04, AF-05 |
| AF-07 | 2 | agent-logic-runtime | Mandatory contract gates pass and predeclared outcome thresholds are met; adverse results and fallback intervals retained | AF-06 |
| AF-08 | 2 | agent-logic-runtime | Claims match demonstrated deployment and measured scope; exclusivity is omitted unless independently substantiated | AF-07 |
| HU-01 | 2 | agent-logic-runtime | Distinguish wrong assumptions from transient errors, missing inputs, impossibility, policy refusal and unknown causes | AF-01, AF-02 |
| HU-02 | 2 | agent-logic-runtime | Alternatives name changed assumptions, supporting/opposing evidence, predictions and budgets; preserve goals, truth and dignity; reject harmful reinterpretations | HU-01 |
| HU-03 | 2 | agent-logic-runtime | A validated frame changes the next executed plan; bounded attempts and oscillation detection; failed reframes retained and replayable | HU-02, AF-03, AF-05 |
| HU-04 | 2 | agent-logic-runtime | Held-out evidence shows whether explicit frames outperform competent diagnosis under matched budgets, including false-reframing and adequate-original-frame cases | HU-03, AF-06 |
| INTEGRATE | 3 | agent-logic-runtime | One installed polis joins citizen action, upstream delegation, private cognition, policy enforcement, appeal and plugin restart without contract drift. | DEMO-GOV, v0.93.2/DEMO-SEC, v0.93.5/CM-04, v0.93.5/GHB-WP12, AF-08, HU-04 |
| QUALIFY | 3 | agent-logic-runtime | Independent reviewer runs the accepted success/failure/recovery denominator on the exact multi-repository lockset. | INTEGRATE |
| TAIL-01 | 4 | agent-design-language | Deliver the complete quality gate result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | QUALIFY |
| TAIL-02 | 4 | agent-design-language | Deliver the complete documentation review and external-review handoff result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-01 |
| TAIL-03 | 4 | agent-design-language | Deliver the complete publication finalization result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-02 |
| TAIL-04 | 4 | agent-design-language | Deliver the complete internal milestone review result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-03 |
| TAIL-05 | 4 | agent-design-language | Deliver the complete external or third-party review result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-04 |
| TAIL-06 | 4 | agent-design-language | Deliver the complete accepted-findings remediation or explicit deferral capture result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-05 |
| TAIL-07 | 4 | agent-design-language | Deliver the complete next-milestone planning result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-06 |
| TAIL-08 | 4 | agent-design-language | Deliver the complete next-milestone closeout planning result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-07 |
| TAIL-09 | 4 | agent-design-language | Deliver the complete next-milestone planning review result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-08 |
| TAIL-10 | 4 | agent-design-language | Deliver the complete release ceremony and milestone close result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. | TAIL-09 |

## Upgrade Notes

No released implementation is produced here. Later delivery must document supported versions, migration/refusal paths and known-good rollback.

## Known Limitations

First-pass successor plan; scope allocation is not implementation or release acceptance. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized by this planning package. Create child issues/cards per sprint at activation; preserve native C-SDLC v3 authority until a separately accepted compatibility cutover.

## Validation Notes

This setup uses focused planning validation; it does not claim Runtime/tooling/cognitive behavior.

PVF planning_contract for this setup: registry headings, JSON/YAML mirrors, source/link validity, complete allocation and acyclic qualified dependencies. Product work must later prove each acceptance and negative case through the owning installed consumer; schema, source or CI alone is not live qualification. Reuse unchanged exact-input evidence; no broad local Rust or paid qualification campaign for planning.

## What's Next

[Successor handoff](NEXT_MILESTONE_HANDOFF_v0.93.6.md).

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
