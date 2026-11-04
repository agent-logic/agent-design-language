> **Current #1416 routing (2026-11-04):** Runtime v4 foundations remain in Sprints 1–3. GHB program Sprint 2 is milestone Sprint 4, including GOV-01/02/03/11 and CM-01..04 unchanged requirements; GHB program Sprint 3 is milestone Sprint 5. Runtime #238 remains the whole-program parent and closes after all three sprints. Containment never makes a child depend on parent completion. Actual Runtime v4, standing, continuity and protected evaluator integrity precede activation; installed Gate A/B proof remains required. See [lossless relocation register](../v0.93.3/REALLOCATION_1416.json) and [three-sprint decision](../v0.93.3/REORGANIZATION_1416.md). Planning is not execution, spend, installed qualification or release authority.

# v0.93.3 Sprint

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.3. Date: 2026-10-30. Setup amendment: [#1405](https://github.com/agent-logic/agent-design-language/issues/1405). Planning owner: Planning #12; implementation owners assigned at sprint activation.

## Status

Setup prepared; implementation not started. Planning only. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized. Create child issues/cards per activated sprint; current authenticated C-SDLC remains authority until separately accepted cutover.

## How To Use

Read the WBS and execution graph together; qualified milestone/ID pairs are identities. Preserve dependencies and adverse cases; reuse existing owner issues. Plan review does not establish product proof.

## Sprint Overview

38 active planned work-package identities across 5 dependency streams. Runtime v4 foundations remain in Sprints 1–3. GHB program Sprint 2 is milestone Sprint 4, including GOV-01/02/03/11 and CM-01..04 unchanged requirements; GHB program Sprint 3 is milestone Sprint 5. Runtime #238 remains the whole-program parent and closes after all three sprints. Containment never makes a child depend on parent completion. Actual Runtime v4, standing, continuity and protected evaluator integrity precede activation; installed Gate A/B proof remains required.

## Sprint Goals

### Sprint 1: Interfaces and carryover

WP-01, RV-01, ADL-1352.

### Sprint 2: Lifecycle and migration

RV-02, RV-03, RV-09, RV-10, RV-11.

### Sprint 3: Adapters, recovery and handoff

RV-04, RV-05, RV-06, RV-07, RV-08, FOUNDATION-REVIEW, FOUNDATION-CLOSE.

### Sprint 4: GHB program Sprint 2 (GHB program Sprint 2)

GOV-01, GOV-02, GOV-03, GOV-11, CM-01, CM-02, CM-03, CM-04, GHB-WP03, GHB-WP04, GHB-WP05, GHB-WP06, GHB-WP07, GHB-WP08, GHB-S2. Umbrella: [Runtime #245](https://github.com/agent-logic/agent-logic-runtime/issues/245).

### Sprint 5: GHB program Sprint 3 (GHB program Sprint 3)

GHB-PROGRAM, GHB-WP09, GHB-WP10, GHB-WP11, GHB-WP12, GHB-REVIEW, GHB-CLOSE, GHB-S3. Umbrella: [Runtime #246](https://github.com/agent-logic/agent-logic-runtime/issues/246).

## Sprint Goal

38 active planned work-package identities across 5 dependency streams. Runtime v4 foundations remain in Sprints 1–3. GHB program Sprint 2 is milestone Sprint 4, including GOV-01/02/03/11 and CM-01..04 unchanged requirements; GHB program Sprint 3 is milestone Sprint 5. Runtime #238 remains the whole-program parent and closes after all three sprints. Containment never makes a child depend on parent completion. Actual Runtime v4, standing, continuity and protected evaluator integrity precede activation; installed Gate A/B proof remains required.

## Planned Scope

| WP | Sprint | Repository | Outcome | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | 1 | agent-design-language | Admit only the reviewed Runtime v4 foundations after accepted v0.93.2 inputs; assign the first bounded sprint without activating successor programmes. | v0.93.2/TAIL-10 |
| RV-01 | 1 | agent-logic-runtime | An accepted plugin contract defines topology precedence, epochs, durable activation journal, in-flight delivery, cancellation, versioned state/config migration, two-phase reconfiguration, dependency-safe removal and recovery classes. | v0.93.1/RD-11, WP-01 |
| RV-02 | 2 | agent-logic-runtime | Existing Runtime behavior and identity survive native adapter conversion with no second scheduler. | RV-01 |
| RV-03 | 2 | agent-logic-runtime | The durable generation transaction stages, quiesces, activates and rolls back to one fenced generation after interruption; in-flight work and cancellation retain their declared disposition. | RV-02 |
| RV-09 | 2 | agent-logic-runtime | An installed plugin migrates declared prior-version state to the selected version and preserves the prior generation when the migrator refuses or fails. | RV-03 |
| RV-10 | 2 | agent-logic-runtime | Versioned configuration migration and two-phase validation/activation apply one accepted configuration, or preserve the prior version and active configuration on refusal, failure or interruption. | RV-09 |
| RV-11 | 2 | agent-logic-runtime | A requested plugin removal checks dependencies and in-flight work, refuses unsafe removal, and completes authorized removal with durable recovery state. | RV-10 |
| RV-04 | 3 | agent-logic-runtime | One real non-core plugin uses authenticated bounded transport, deadlines and process supervision. | RV-11 |
| RV-05 | 3 | agent-logic-runtime | The canonical contract drives a default-deny WIT adapter with enforced memory, time and host-call limits. | RV-04 |
| RV-06 | 3 | agent-logic-runtime | One module has equivalent CSM-visible identity, messages, failure and state behavior across native, process and WASM adapters. | RV-05 |
| RV-07 | 3 | agent-logic-runtime | Provider instances consume host-owned credentials and versioned configuration through the common inference capability. | RV-06 |
| RV-08 | 3 | agent-logic-runtime | Installed package signing, checkpoint compatibility, generation recovery and Observatory operations pass with a retained known-good rollback. | RV-07 |
| ADL-1352 | 1 | agent-design-language | Complete the existing #1352 language-limit/interpreter-free public-delivery acceptance without reopening delivered cleanup. | WP-01 |
| FOUNDATION-REVIEW | 3 | agent-design-language | One independent integrated review accepts the exact Runtime foundation lockset and carryover; reuse valid tests and qualification instead of repeating them. | RV-08, ADL-1352 |
| FOUNDATION-CLOSE | 3 | agent-design-language | Record accepted foundation interfaces, compatibility, recovery limits and successor handoff; release/tag only under a separate scoped decision. | FOUNDATION-REVIEW |
| GOV-01 | 4 | agent-logic-runtime | Runtime admits citizen conduct only through bound identity, Freedom Gate and signed trace; guest/operator action remains distinct. | RV-08, WP-01 |
| GOV-02 | 4 | agent-logic-runtime | A Runtime policy evaluator returns rights/duties decisions with trace evidence for a bounded action. | GOV-01 |
| GOV-03 | 4 | agent-logic-runtime | Standing maintenance, restriction, restoration and revocation consume existing state primitives and retain reasons and challenge context. | GOV-02 |
| GOV-11 | 4 | agent-logic-runtime | Delegated action binds identity, standing, resource, capability and policy without exceeding the delegator grant. | GOV-03 |
| CM-01 | 4 | agent-logic-runtime | A reviewed contract distinguishes same-identity migration and recovery from new-identity descendants, replication and independent clones; it defines authority, lineage, consent, state inheritance, fencing and recovery outcomes. | RV-08, GOV-01 |
| CM-02 | 4 | agent-logic-runtime | Two owned Runtime endpoints transfer one authorized citizen checkpoint and scoped continuity evidence; the source is fenced before destination activation and interruption can recover without two active holders or lost lineage. | CM-01, v0.93.2/WP-S3 |
| CM-03 | 4 | agent-logic-runtime | An authorized parent creates one distinctly identified descendant with explicit lineage, selected inherited state and newly scoped authority while preserving the parent and protecting private state. | CM-01, GOV-11, v0.93.2/WP-S5 |
| CM-04 | 4 | agent-logic-runtime | Independent installed scenarios exercise migration and descendant creation across restart, interruption and retry, verify lineage/privacy/fencing and expose the resulting identity and custody status to the operator. | CM-02, CM-03 |
| GHB-PROGRAM | 5 | agent-logic-runtime | Open and coordinate the approved GHB program without granting execution, spend, or completion authority. | GHB-CLOSE |
| GHB-WP03 | 4 | agent-logic-runtime | Owner: Runtime algorithm lane; exact module selected by WP00. Dependencies: WP02. | v0.93.2/GHB-WP02, FOUNDATION-CLOSE, GOV-03, GOV-11, CM-04 |
| GHB-WP04 | 4 | agent-logic-runtime | Owner: Runtime algorithm lane. Dependencies: WP02; integrate WP03 selection after initial archive support. | v0.93.2/GHB-WP02, GHB-WP03 |
| GHB-WP05 | 4 | agent-logic-runtime | Owner: Runtime/execution. Dependencies: WP02. | v0.93.2/GHB-WP02 |
| GHB-WP06 | 4 | agent-logic-runtime | Owner: evaluation lane. Dependencies: WP00 protocol, WP02 contracts, WP03 statistics, WP05 executor. | v0.93.2/GHB-WP00, v0.93.2/GHB-WP02, GHB-WP03, GHB-WP05 |
| GHB-WP07 | 4 | agent-logic-runtime | Owner: Runtime. Dependencies: WP00 audit; coordinate/reuse Runtime #112 rather than opening a duplicate. | v0.93.2/GHB-WP00, RV-09, RV-10, CM-02 |
| GHB-WP08 | 4 | agent-logic-runtime | Owner: Runtime. Dependencies: WP02, WP05, WP06, WP07. | v0.93.2/GHB-WP02, GHB-WP05, GHB-WP06, GHB-WP07, RV-11, CM-03, CM-04 |
| GHB-WP09 | 5 | agent-logic-runtime | Owner: Runtime resident integration. Dependencies: WP03–WP08. | GHB-WP03, GHB-WP04, GHB-WP05, GHB-WP06, GHB-WP07, GHB-WP08 |
| GHB-WP10 | 5 | agent-logic-runtime | Owner: Runtime algorithm and activation owners. Dependencies: Gate A. | GHB-WP09 |
| GHB-WP11 | 5 | agent-logic-runtime | Owner: evaluation + Runtime. Dependencies: WP10; test design begins during WP06. | GHB-WP10 |
| GHB-WP12 | 5 | agent-logic-runtime | Owner: docs/research, with independent review. Dependencies: WP01 for vocabulary; WP11 for final claims. | v0.93.2/GHB-WP01, GHB-WP11 |
| GHB-REVIEW | 5 | agent-design-language | Independently accept the complete installed GHB baseline and its required continuity/standing proof, reusing unchanged input evidence. | GHB-WP12, CM-04 |
| GHB-CLOSE | 5 | agent-design-language | Record the exact accepted GHB interfaces and limitations for v0.93.6 governance and v0.93.7 affect/humour. Any release remains separately authorized. | GHB-REVIEW |
| GHB-S2 | 4 | agent-logic-runtime | Coordinate and accept only GHB program Sprint 2; preserve its assigned package acceptance and negative cases. | GHB-WP08 |
| GHB-S3 | 5 | agent-logic-runtime | Coordinate and accept only GHB program Sprint 3; preserve its assigned package acceptance and negative cases. | GHB-CLOSE |

## Work Plan

### Sprint 1: Interfaces and carryover

WP-01, RV-01, ADL-1352.

### Sprint 2: Lifecycle and migration

RV-02, RV-03, RV-09, RV-10, RV-11.

### Sprint 3: Adapters, recovery and handoff

RV-04, RV-05, RV-06, RV-07, RV-08, FOUNDATION-REVIEW, FOUNDATION-CLOSE.

### Sprint 4: GHB program Sprint 2 (GHB program Sprint 2)

GOV-01, GOV-02, GOV-03, GOV-11, CM-01, CM-02, CM-03, CM-04, GHB-WP03, GHB-WP04, GHB-WP05, GHB-WP06, GHB-WP07, GHB-WP08, GHB-S2. Umbrella: [Runtime #245](https://github.com/agent-logic/agent-logic-runtime/issues/245).

### Sprint 5: GHB program Sprint 3 (GHB program Sprint 3)

GHB-PROGRAM, GHB-WP09, GHB-WP10, GHB-WP11, GHB-WP12, GHB-REVIEW, GHB-CLOSE, GHB-S3. Umbrella: [Runtime #246](https://github.com/agent-logic/agent-logic-runtime/issues/246).

## Execution Policy

One accountable owner per admitted issue; native bind/start and issue-bound goal precede implementation. Independent review precedes a PR; merge only green at the reviewed head. Preserve source and evidence; administrative reconciliation and optional cleanup may run asynchronously. Do not create a second closeout PR or repeat tests/reviews merely to update records.

## Cadence Expectations

Dependency-driven waves; no calendar promises or implied parallel staffing. Parallelize only disjoint write sets with accepted interfaces; do not duplicate active work.

## Risks / Dependencies

Accepted predecessor inputs and compatibility. Provider/experiment costs require scoped approval. Missing proof is a named gap; no retries or new machinery merely to satisfy records.

## Demo / Review Plan

[Proof matrix](DEMO_MATRIX_v0.93.3.md); PVF planning_contract for this setup: registry headings, JSON/YAML mirrors, source/link validity, complete allocation and acyclic qualified dependencies. Product work must later prove each acceptance and negative case through the owning installed consumer; schema, source or CI alone is not live qualification. Reuse unchanged exact-input evidence; no broad local Rust or paid qualification campaign for planning.

## Closeout Bar

One accountable owner per admitted issue; native bind/start and issue-bound goal precede implementation. Independent review precedes a PR; merge only green at the reviewed head. Preserve source and evidence; administrative reconciliation and optional cleanup may run asynchronously. Do not create a second closeout PR or repeat tests/reviews merely to update records.

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
