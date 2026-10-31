# v0.93.5 Wbs

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.5. Date: 2026-10-30. Setup amendment: [#1405](https://github.com/agent-logic/agent-design-language/issues/1405). Planning owner: Planning #12; implementation owners assigned at sprint activation.

## Status

First-pass successor plan; not opened for implementation or release. Planning only. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized. Create child issues/cards per activated sprint; current authenticated C-SDLC remains authority until separately accepted cutover.

## How To Use

Read the WBS and execution graph together; qualified milestone/ID pairs are identities. Preserve dependencies and adverse cases; reuse existing owner issues. Plan review does not establish product proof.

## WBS Summary

GHB. 17 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Hand off accepted GHB interfaces to [governance v0.93.6](../v0.93.6/README.md) and later [affect/humour v0.93.7](../v0.93.7/README.md); neither is a GHB baseline dependency.

## Candidate WP Sequence

The Work Packages table is the executable allocation; each row names dependencies, owner repository and result. Existing #1352 is reused. No child issues were created in this setup.

## Work Packages

| WP | Sprint | Repository | Outcome | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | 1 | agent-design-language | Consume accepted Runtime v4 and v0.93.4 tooling/governance/continuity inputs, then activate only the reviewed GHB sprint. | v0.93.3/FOUNDATION-CLOSE, v0.93.4/RELEASE |
| GHB-SPRINT | 1 | agent-logic-runtime | Open and coordinate the approved GHB program without granting execution, spend, or completion authority. | WP-01, v0.93.3/RV-08 |
| GHB-WP00 | 1 | agent-logic-runtime | Owner: program + Runtime. Dependencies: none. | GHB-SPRINT |
| GHB-WP01 | 1 | agent-design-language | Owner: ADL. Dependencies: WP00. | GHB-SPRINT, GHB-WP00 |
| GHB-WP02 | 1 | agent-design-language | Owner: ADL. Dependencies: WP01. | GHB-SPRINT, GHB-WP01 |
| GHB-WP03 | 2 | agent-logic-runtime | Owner: Runtime algorithm lane; exact module selected by WP00. Dependencies: WP02. | GHB-SPRINT, GHB-WP02 |
| GHB-WP04 | 2 | agent-logic-runtime | Owner: Runtime algorithm lane. Dependencies: WP02; integrate WP03 selection after initial archive support. | GHB-SPRINT, GHB-WP02, GHB-WP03 |
| GHB-WP05 | 2 | agent-logic-runtime | Owner: Runtime/execution. Dependencies: WP02. | GHB-SPRINT, GHB-WP02 |
| GHB-WP06 | 2 | agent-logic-runtime | Owner: evaluation lane. Dependencies: WP00 protocol, WP02 contracts, WP03 statistics, WP05 executor. | GHB-SPRINT, GHB-WP00, GHB-WP02, GHB-WP03, GHB-WP05 |
| GHB-WP07 | 2 | agent-logic-runtime | Owner: Runtime. Dependencies: WP00 audit; coordinate/reuse Runtime #112 rather than opening a duplicate. | GHB-SPRINT, GHB-WP00, v0.93.3/RV-09, v0.93.3/RV-10, v0.93.4/CM-02 |
| GHB-WP08 | 2 | agent-logic-runtime | Owner: Runtime. Dependencies: WP02, WP05, WP06, WP07. | GHB-SPRINT, GHB-WP02, GHB-WP05, GHB-WP06, GHB-WP07, v0.93.3/RV-11, v0.93.4/CM-03, v0.93.4/CM-04 |
| GHB-WP09 | 3 | agent-logic-runtime | Owner: Runtime resident integration. Dependencies: WP03–WP08. | GHB-SPRINT, GHB-WP03, GHB-WP04, GHB-WP05, GHB-WP06, GHB-WP07, GHB-WP08 |
| GHB-WP10 | 3 | agent-logic-runtime | Owner: Runtime algorithm and activation owners. Dependencies: Gate A. | GHB-SPRINT, GHB-WP09 |
| GHB-WP11 | 3 | agent-logic-runtime | Owner: evaluation + Runtime. Dependencies: WP10; test design begins during WP06. | GHB-SPRINT, GHB-WP10 |
| GHB-WP12 | 3 | agent-logic-runtime | Owner: docs/research, with independent review. Dependencies: WP01 for vocabulary; WP11 for final claims. | GHB-SPRINT, GHB-WP01, GHB-WP11 |
| GHB-REVIEW | 3 | agent-design-language | Independently accept the complete installed GHB baseline and its required continuity/standing proof, reusing unchanged input evidence. | GHB-WP12, v0.93.4/CM-04 |
| GHB-CLOSE | 3 | agent-design-language | Record the exact accepted GHB interfaces and limitations for v0.93.6 governance and v0.93.7 affect/humour. Any release remains separately authorized. | GHB-REVIEW |

## Sequencing

Prepare the GHB baseline → Build and evaluate GHB → Installed recursive improvement and handoff

## Sequencing Notes

Preserve original qualified prerequisite edges through current routing. Every package descends from its milestone WP-01. Governance completes at GOV-CLOSE without waiting for affect/humour; original combined integration and tail remain in v0.93.7.

## Acceptance Mapping

[Per-package acceptance and negative cases](QUALITY_GATE_v0.93.5.md).

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
