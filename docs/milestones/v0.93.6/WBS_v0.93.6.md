# v0.93.6 Wbs

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.6. Date: 2026-10-30. Setup amendment: [#1405](https://github.com/agent-logic/agent-design-language/issues/1405). Planning owner: Planning #12; implementation owners assigned at sprint activation.

## Status

First-pass successor plan; not opened for implementation or release. Planning only. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized. Create child issues/cards per activated sprint; current authenticated C-SDLC remains authority until separately accepted cutover.

## How To Use

Read the WBS and execution graph together; qualified milestone/ID pairs are identities. Preserve dependencies and adverse cases; reuse existing owner issues. Plan review does not establish product proof.

## WBS Summary

Governance and social cognition. 17 planned work-package identities across 2 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Use GOV-CLOSE to hand off independently accepted governance interfaces and limitations to [affect/humour v0.93.7](../v0.93.7/README.md). Governance can complete before affect/humour exists.

## Candidate WP Sequence

The Work Packages table is the executable allocation; each row names dependencies, owner repository and result. Existing #1352 is reused. No child issues were created in this setup.

## Work Packages

| WP | Sprint | Repository | Outcome | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | 1 | agent-design-language | Consume accepted GHB and foundation interfaces, then activate the bounded governance/social-cognition wave independently of later affect/humour. | v0.93.5/GHB-CLOSE |
| GOV-04 | 1 | agent-logic-runtime | A review consumer emits findings over trace/outcome/policy evidence without raw private-state access. | v0.93.4/GOV-03, WP-01 |
| GOV-05 | 1 | agent-logic-runtime | A challenged finding progresses through appeal and recorded disposition without destroying original evidence. | GOV-04 |
| GOV-06 | 1 | agent-logic-runtime | Signed evidence-backed model updates replay under standing, consent/scope and projection controls. | v0.93.4/GOV-01, WP-01 |
| GOV-07 | 1 | agent-logic-runtime | Conflicting and stale entries change a bounded consumer decision through explicit uncertainty/decay events. | GOV-06 |
| GOV-08 | 1 | agent-logic-runtime | Relationship records preserve evidence and permitted scope across restart without becoming standing verdicts. | GOV-06 |
| GOV-09 | 1 | agent-logic-runtime | Authorized redacted projections can inform review while retaining challenge and source lineage. | GOV-05, GOV-07, GOV-08 |
| GOV-10 | 1 | agent-logic-runtime | Two governed consumers share evidence projections with preserved uncertainty, access rules and freshness. | GOV-09 |
| GOV-12 | 1 | agent-logic-runtime | A selected/rejected upstream call retains citizen identity, policy, provenance and verification; ToM may inform selection but cannot override authority. | v0.93.4/GOV-11, GOV-07 |
| GOV-13 | 1 | agent-logic-runtime | A governed exchange succeeds while an associated unauthorized private-state/ToM inspection is denied. | v0.93.4/GOV-11, WP-01 |
| GOV-14 | 1 | agent-logic-runtime | A policy consumer evaluates explicit polis/citizen obligations and emits reviewable evidence. | GOV-05, GOV-10, GOV-13 |
| GOV-15 | 1 | agent-logic-runtime | Guild identity, membership, scoped resources and delegated actions persist with trace and challenge support. | GOV-14, GOV-12 |
| GOV-16 | 1 | agent-logic-runtime | The operator receives evidence-linked standing, appeals and social-memory summaries without private-state leaks or scalar moral ranking. | GOV-15 |
| DEMO-GOV | 2 | agent-logic-runtime | Execute D1-D8 and guild D13 through installed Runtime consumers with accepted and refused paths. | GOV-16 |
| GOV-INTEGRATE | 2 | agent-design-language | Installed governance integrates identity, appeals, private cognition, social memory, delegated action and plugin recovery; affect/humour is not an entry or exit dependency. | DEMO-GOV, v0.93.2/DEMO-SEC, v0.93.4/CM-04, v0.93.5/GHB-CLOSE |
| GOV-REVIEW | 2 | agent-design-language | Independently exercise the installed governance success/refusal/recovery denominator and resolve actionable findings on the exact candidate; do not infer qualification from source or CI. | GOV-INTEGRATE |
| GOV-CLOSE | 2 | agent-design-language | Record accepted governance interfaces, redaction and consent boundaries, limitations and source/artifact lockset for later affect/humour integration; no release from planning. | GOV-REVIEW |

## Sequencing

Governance consumers → Installed governance acceptance

## Sequencing Notes

Preserve original qualified prerequisite edges through current routing. Every package descends from its milestone WP-01. Governance completes at GOV-CLOSE without waiting for affect/humour; original combined integration and tail remain in v0.93.7.

## Acceptance Mapping

[Per-package acceptance and negative cases](QUALITY_GATE_v0.93.6.md).

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
