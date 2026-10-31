# v0.93.5 Wbs

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.5. Date: 2026-10-30. Setup issue: [#1403](https://github.com/agent-logic/agent-design-language/issues/1403). Planning owner: Planning #12. Implementation owners assigned at sprint activation.

## Status

First-pass successor plan; scope allocation is not implementation or release acceptance. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized by this planning package. Create child issues/cards per sprint at activation; preserve native C-SDLC v3 authority until a separately accepted compatibility cutover.

## How To Use

Read the WBS and execution graph together; qualified milestone/ID pairs are identities. Preserve dependencies and adverse cases; reuse existing owner issues. Plan review does not establish product proof.

## WBS Summary

GHB and required continuity foundations. 25 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. GHB retains the eight required standing/delegation and citizen-migration/reproduction prerequisite packages; all broader cognitive work moves to v0.93.6.

## Candidate WP Sequence

The Work Packages table is the executable allocation; each row names dependencies, owner repository and result. Existing #1352 is reused. No child issues were created in this setup.

## Work Packages

| WP | Sprint | Repository | Outcome | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | 1 | agent-design-language | Consume accepted Runtime foundations and tooling compatibility, then activate only the reviewed GHB wave and its minimum existing continuity/standing prerequisites. | v0.93.3/FOUNDATION-CLOSE, v0.93.4/RELEASE |
| GOV-01 | 1 | agent-logic-runtime | Runtime admits citizen conduct only through bound identity, Freedom Gate and signed trace; guest/operator action remains distinct. | v0.93.3/RV-08, WP-01 |
| GOV-02 | 1 | agent-logic-runtime | A Runtime policy evaluator returns rights/duties decisions with trace evidence for a bounded action. | GOV-01 |
| GOV-03 | 1 | agent-logic-runtime | Standing maintenance, restriction, restoration and revocation consume existing state primitives and retain reasons and challenge context. | GOV-02 |
| GOV-11 | 1 | agent-logic-runtime | Delegated action binds identity, standing, resource, capability and policy without exceeding the delegator grant. | GOV-03 |
| CM-01 | 1 | agent-logic-runtime | A reviewed contract distinguishes same-identity migration and recovery from new-identity descendants, replication and independent clones; it defines authority, lineage, consent, state inheritance, fencing and recovery outcomes. | v0.93.3/RV-08, GOV-01 |
| CM-02 | 1 | agent-logic-runtime | Two owned Runtime endpoints transfer one authorized citizen checkpoint and scoped continuity evidence; the source is fenced before destination activation and interruption can recover without two active holders or lost lineage. | CM-01, v0.93.2/WP-S3 |
| CM-03 | 1 | agent-logic-runtime | An authorized parent creates one distinctly identified descendant with explicit lineage, selected inherited state and newly scoped authority while preserving the parent and protecting private state. | CM-01, GOV-11, v0.93.2/WP-S5 |
| CM-04 | 1 | agent-logic-runtime | Independent installed scenarios exercise migration and descendant creation across restart, interruption and retry, verify lineage/privacy/fencing and expose the resulting identity and custody status to the operator. | CM-02, CM-03 |
| GHB-SPRINT | 2 | agent-logic-runtime | Open and coordinate the approved GHB program without granting execution, spend, or completion authority. | WP-01, v0.93.3/RV-08 |
| GHB-WP00 | 2 | agent-logic-runtime | Owner: program + Runtime. Dependencies: none. | GHB-SPRINT |
| GHB-WP01 | 2 | agent-design-language | Owner: ADL. Dependencies: WP00. | GHB-SPRINT, GHB-WP00 |
| GHB-WP02 | 2 | agent-design-language | Owner: ADL. Dependencies: WP01. | GHB-SPRINT, GHB-WP01 |
| GHB-WP03 | 2 | agent-logic-runtime | Owner: Runtime algorithm lane; exact module selected by WP00. Dependencies: WP02. | GHB-SPRINT, GHB-WP02 |
| GHB-WP04 | 2 | agent-logic-runtime | Owner: Runtime algorithm lane. Dependencies: WP02; integrate WP03 selection after initial archive support. | GHB-SPRINT, GHB-WP02, GHB-WP03 |
| GHB-WP05 | 2 | agent-logic-runtime | Owner: Runtime/execution. Dependencies: WP02. | GHB-SPRINT, GHB-WP02 |
| GHB-WP06 | 2 | agent-logic-runtime | Owner: evaluation lane. Dependencies: WP00 protocol, WP02 contracts, WP03 statistics, WP05 executor. | GHB-SPRINT, GHB-WP00, GHB-WP02, GHB-WP03, GHB-WP05 |
| GHB-WP07 | 2 | agent-logic-runtime | Owner: Runtime. Dependencies: WP00 audit; coordinate/reuse Runtime #112 rather than opening a duplicate. | GHB-SPRINT, GHB-WP00, v0.93.3/RV-09, v0.93.3/RV-10, CM-02 |
| GHB-WP08 | 2 | agent-logic-runtime | Owner: Runtime. Dependencies: WP02, WP05, WP06, WP07. | GHB-SPRINT, GHB-WP02, GHB-WP05, GHB-WP06, GHB-WP07, v0.93.3/RV-11, CM-03, CM-04 |
| GHB-WP09 | 2 | agent-logic-runtime | Owner: Runtime resident integration. Dependencies: WP03–WP08. | GHB-SPRINT, GHB-WP03, GHB-WP04, GHB-WP05, GHB-WP06, GHB-WP07, GHB-WP08 |
| GHB-WP10 | 2 | agent-logic-runtime | Owner: Runtime algorithm and activation owners. Dependencies: Gate A. | GHB-SPRINT, GHB-WP09 |
| GHB-WP11 | 2 | agent-logic-runtime | Owner: evaluation + Runtime. Dependencies: WP10; test design begins during WP06. | GHB-SPRINT, GHB-WP10 |
| GHB-WP12 | 2 | agent-logic-runtime | Owner: docs/research, with independent review. Dependencies: WP01 for vocabulary; WP11 for final claims. | GHB-SPRINT, GHB-WP01, GHB-WP11 |
| GHB-REVIEW | 3 | agent-design-language | Independently accept the complete installed GHB baseline and its required continuity/standing proof, reusing unchanged input evidence. | GHB-WP12, CM-04 |
| GHB-CLOSE | 3 | agent-design-language | Record the exact accepted GHB/continuity interfaces and limitations for v0.93.6. Any release remains separately authorized. | GHB-REVIEW |

## Sequencing

Minimum GHB prerequisites → Installed GHB baseline → Acceptance and handoff

## Sequencing Notes

Source dependency edges are preserved. Cross-version Runtime edges are qualified v0.93.3/RV-xx. AF-04 consumes an accepted GHB baseline; integration consumes AF-08/HU-04, avoiding a baseline/affect cycle.

## Acceptance Mapping

[Per-package acceptance and negative cases](QUALITY_GATE_v0.93.5.md).

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
