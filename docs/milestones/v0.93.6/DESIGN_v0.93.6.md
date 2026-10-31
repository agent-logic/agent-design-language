# v0.93.6 Design

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.6. Date: 2026-10-30. Setup amendment: [#1405](https://github.com/agent-logic/agent-design-language/issues/1405). Planning owner: Planning #12; implementation owners assigned at sprint activation.

## Purpose

Governance and social cognition. 17 planned work-package identities across 2 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Use GOV-CLOSE to hand off independently accepted governance interfaces and limitations to [affect/humour v0.93.7](../v0.93.7/README.md). Governance can complete before affect/humour exists.

## Problem Statement

The former 59-identity platform milestone combines independent foundation, tooling and cognitive programmes. Separate them without losing contracts, historical identity or acceptance.

## Goals

Governance and social cognition. 17 planned work-package identities across 2 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Use GOV-CLOSE to hand off independently accepted governance interfaces and limitations to [affect/humour v0.93.7](../v0.93.7/README.md). Governance can complete before affect/humour exists.

## Non-Goals

Planning only. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized. Create child issues/cards per activated sprint; current authenticated C-SDLC remains authority until separately accepted cutover. Component container packaging and other unadmitted TBD concepts remain deferred.

## Scope

Governance and social cognition. 17 planned work-package identities across 2 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Use GOV-CLOSE to hand off independently accepted governance interfaces and limitations to [affect/humour v0.93.7](../v0.93.7/README.md). Governance can complete before affect/humour exists.

## Requirements

| WP | Acceptance / proof | Adverse cases |
| --- | --- | --- |
| WP-01 | The governance wave consumes accepted GHB/continuity inputs and has its own installed integration/review/handoff exit. Affect/humour is not a prerequisite. | Missing predecessor acceptance; Unsupported completion claim |
| GOV-04 | A review consumer emits findings over trace/outcome/policy evidence without raw private-state access.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Fabricated evidence; Raw state disclosure |
| GOV-05 | A challenged finding progresses through appeal and recorded disposition without destroying original evidence.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Replay changes verdict; Evidence lost on appeal |
| GOV-06 | Signed evidence-backed model updates replay under standing, consent/scope and projection controls.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Unsigned update; Guest scope exceeded |
| GOV-07 | Conflicting and stale entries change a bounded consumer decision through explicit uncertainty/decay events.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Conflict overwritten; Stale confidence unchanged |
| GOV-08 | Relationship records preserve evidence and permitted scope across restart without becoming standing verdicts.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Untraceable relationship assertion |
| GOV-09 | Authorized redacted projections can inform review while retaining challenge and source lineage.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Raw ToM leak; Scalar moral verdict |
| GOV-10 | Two governed consumers share evidence projections with preserved uncertainty, access rules and freshness.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Private model copied; Stale projection trusted |
| GOV-12 | A selected/rejected upstream call retains citizen identity, policy, provenance and verification; ToM may inform selection but cannot override authority.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Provider becomes sovereign; Unverified answer accepted |
| GOV-13 | A governed exchange succeeds while an associated unauthorized private-state/ToM inspection is denied.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Message receipt grants inspection |
| GOV-14 | A policy consumer evaluates explicit polis/citizen obligations and emits reviewable evidence.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings.; The installed obligations consumer evaluates accepted, expired and missed commitments and rejects causally invalid ordering using explicit temporal evidence and reviewable policy disposition. | Policy text treated as executed behavior; Expired commitment treated as current; Missed deadline silently satisfied; Causally invalid commitment accepted |
| GOV-15 | Guild identity, membership, scoped resources and delegated actions persist with trace and challenge support.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Revoked membership acts; Guild exceeds citizen authority |
| GOV-16 | The operator receives evidence-linked standing, appeals and social-memory summaries without private-state leaks or scalar moral ranking.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Hidden uncertainty; Protected data in report |
| DEMO-GOV | Execute D1-D8 and guild D13 through installed Runtime consumers with accepted and refused paths.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Demo replaces production path |
| GOV-INTEGRATE | Installed governance integrates identity, appeals, private cognition, social memory, delegated action and plugin recovery; affect/humour is not an entry or exit dependency. | Missing installed success/refusal/recovery evidence; Private-state disclosure or unsupported acceptance claim |
| GOV-REVIEW | Independently exercise the installed governance success/refusal/recovery denominator and resolve actionable findings on the exact candidate; do not infer qualification from source or CI. | Missing installed success/refusal/recovery evidence; Private-state disclosure or unsupported acceptance claim |
| GOV-CLOSE | Record accepted governance interfaces, redaction and consent boundaries, limitations and source/artifact lockset for later affect/humour integration; no release from planning. | Missing installed success/refusal/recovery evidence; Private-state disclosure or unsupported acceptance claim |

## Proposed Design

One owner for each existing execution/state authority. Preserve plugin and consumer compatibility; no duplicate schedulers, citizen identity engines or model/provider routes.

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

## Interfaces And Contracts

Runtime foundations export accepted versioned plugin/lifecycle/state/configuration interfaces. C-SDLC exports an accepted v3-compatible workflow/cutover contract. Cognitive consumers preserve identity, consent, privacy and existing resource/governance checks.

## Validation Plan

PVF planning_contract for this setup: registry headings, JSON/YAML mirrors, source/link validity, complete allocation and acyclic qualified dependencies. Product work must later prove each acceptance and negative case through the owning installed consumer; schema, source or CI alone is not live qualification. Reuse unchanged exact-input evidence; no broad local Rust or paid qualification campaign for planning.

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
