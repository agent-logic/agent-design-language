# v0.93.4 Design

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.4. Date: 2026-10-30. Setup amendment: [#1405](https://github.com/agent-logic/agent-design-language/issues/1405). Planning owner: Planning #12; implementation owners assigned at sprint activation.

## Purpose

C-SDLC v4 and governance/continuity foundations. 17 planned work-package identities across 4 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Hand off both accepted tooling compatibility and installed governance/continuity foundations to [GHB v0.93.5](../v0.93.5/README.md). Tool cutover alone does not accept foundation consumers.

## Problem Statement

The former 59-identity platform milestone combines independent foundation, tooling and cognitive programmes. Separate them without losing contracts, historical identity or acceptance.

## Goals

C-SDLC v4 and governance/continuity foundations. 17 planned work-package identities across 4 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Hand off both accepted tooling compatibility and installed governance/continuity foundations to [GHB v0.93.5](../v0.93.5/README.md). Tool cutover alone does not accept foundation consumers.

## Non-Goals

Planning only. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized. Create child issues/cards per activated sprint; current authenticated C-SDLC remains authority until separately accepted cutover. Component container packaging and other unadmitted TBD concepts remain deferred.

## Scope

C-SDLC v4 and governance/continuity foundations. 17 planned work-package identities across 4 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Hand off both accepted tooling compatibility and installed governance/continuity foundations to [GHB v0.93.5](../v0.93.5/README.md). Tool cutover alone does not accept foundation consumers.

## Requirements

| WP | Acceptance / proof | Adverse cases |
| --- | --- | --- |
| WP-01 | Approve one bounded issue-to-delivery workflow and measurable overhead/compatibility denominator before implementation. | Missing predecessor acceptance; Unsupported completion claim |
| CS4-01 | Map create/start, implementation/proof, review, publish/deliver and optional closeout to source-backed sequence diagrams; measure wall/internal/external-tool/network time and identify steps to remove. | Duplicate remote effect; Stale review accepted; Incompatible v3 state; Unaccounted internal overhead |
| CS4-02 | One editable issue source, one canonical installation, minimal command transitions and precise remote identity replace duplicate projections/attestations/receipts. Define bounded recovery and explicit compatibility; retain only independently justified integrity checks. | Duplicate remote effect; Stale review accepted; Incompatible v3 state; Unaccounted internal overhead |
| CS4-03 | Prototype ordinary operations and interrupted recovery against the approved denominator; demonstrate whether each sample stays below 30 seconds internal overhead and a ten-operation path below 300 seconds, reporting cold/warm and external costs separately. | Duplicate remote effect; Stale review accepted; Incompatible v3 state; Unaccounted internal overhead |
| CS4-04 | Read existing v3 issue/proof/review state without rewriting history; preserve exact repository/issue/head and uncertain-effect recovery. Prove rollback and a staged cutover with one canonical installed tool set; no automatic authority fallback. | Duplicate remote effect; Stale review accepted; Incompatible v3 state; Unaccounted internal overhead |
| CS4-05 | Implement only the reviewed workflow with changed-input validation and reusable proof/review, no repeated binary hashing or duplicate builds/CI, and asynchronous optional administrative closeout. Prove supported native macOS/Linux behavior. | Duplicate remote effect; Stale review accepted; Incompatible v3 state; Unaccounted internal overhead |
| CS4-06 | Independent installed end-to-end acceptance covers creation through delivery, metadata-only edits, unchanged proof reuse, conflicts, lost/uncertain responses, crash recovery and optional cleanup. Every ordinary sample meets the internal-overhead goals; failed or ambiguous effects are not silently replayed. | Duplicate remote effect; Stale review accepted; Incompatible v3 state; Unaccounted internal overhead |
| REVIEW | Resolve actionable correctness, compatibility and performance findings once on the final candidate; no extra attestation/receipt campaign. | Missing predecessor acceptance; Unsupported completion claim |
| RELEASE | Obtain explicit operator cutover decision, preserve recoverable v3 state, then record one canonical admitted installation and the supported migration/rollback outcome. No cutover is authorized by this plan.; Accept GOV-01/02/03/11 and CM-01..04 installed outcomes before GHB activation; C-SDLC cutover alone does not accept these foundations. | Missing predecessor acceptance; Unsupported completion claim |
| GOV-01 | Runtime admits citizen conduct only through bound identity, Freedom Gate and signed trace; guest/operator action remains distinct.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings.; Installed cognitive-loop and instinct-originated actions traverse bound identity, Freedom Gate and temporal anchoring; guest or stale identity cannot acquire citizen authority. | Guest impersonates citizen; Instinct bypasses governance; Missing or inconsistent temporal anchor |
| GOV-02 | A Runtime policy evaluator returns rights/duties decisions with trace evidence for a bounded action.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Evidence-free verdict |
| GOV-03 | Standing maintenance, restriction, restoration and revocation consume existing state primitives and retain reasons and challenge context.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Unexplained restriction; Restoration loses history |
| GOV-11 | Delegated action binds identity, standing, resource, capability and policy without exceeding the delegator grant.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings.; A bounded remote action accepts a current scoped delegation and rejects refusal, revocation and replay without applying the action twice. | Overbroad delegation; Revoked grant; Remote replay duplicates effect; Remote refusal ignored |
| CM-01 | A reviewed contract distinguishes same-identity migration and recovery from new-identity descendants, replication and independent clones; it defines authority, lineage, consent, state inheritance, fencing and recovery outcomes.; Independent design review checks the enumerated success, denial and recovery examples against the contract; implementation and installed scenario counts belong to downstream results. | Copy treated as same active identity; Implicit grant inheritance; Ambiguous ownership during transfer |
| CM-02 | Two owned Runtime endpoints transfer one authorized citizen checkpoint and scoped continuity evidence; the source is fenced before destination activation and interruption can recover without two active holders or lost lineage.; Exact versioned inputs and actual positive, denial and recovery scenario counts are retained; independent review resolves actionable findings. | Stale or replayed transfer; Unauthorized destination; Interrupted handover; Source continues after destination activation; Wrong key or corrupt checkpoint |
| CM-03 | An authorized parent creates one distinctly identified descendant with explicit lineage, selected inherited state and newly scoped authority while preserving the parent and protecting private state.; Exact versioned inputs and actual positive, denial and recovery scenario counts are retained; independent review resolves actionable findings. | Unauthorized cloning; Duplicate descendant on retry; Private state inherited without scope; Parent standing or credentials copied as authority |
| CM-04 | Independent installed scenarios exercise migration and descendant creation across restart, interruption and retry, verify lineage/privacy/fencing and expose the resulting identity and custody status to the operator.; Exact versioned inputs and actual positive, denial and recovery scenario counts are retained; independent review resolves actionable findings. | Split-brain identity; Orphaned in-transit custody; Silent lineage loss; Retry creates extra descendant |

## Proposed Design

The required workflow is issue intake → optional preparation/bind → bounded implementation → focused proof → independent review → reviewed publication/delivery → optional asynchronous cleanup. Start with source-backed sequence diagrams and baseline measurements; command names and storage layout are design outputs, not predetermined new machinery.

Use one editable semantic issue source and one canonical admitted installation. Validate changed inputs; reuse valid exact-input proof/review; remove redundant projection drift, attestations, receipts and repeated binary hashing unless an identified threat/recovery invariant requires a check. Keep repository/issue/head identity and uncertain-remote-effect reconciliation explicit. Do not introduce a second state engine or blanket concurrency cap.

Performance acceptance is **less than 30 seconds internal overhead for each ordinary-operation sample**, and **less than 300 seconds internal overhead for the standard ten-operation lifecycle**. Record sample count, cold/warm conditions, wall time and independently measured network/external-tool time; report unclassified time as internal rather than hiding it. These are design/performance goals, **not hard kill timers**; do not reinstate the fixed wall-clock deadlines targeted by C-SDLC #328. Report p50/p95/max without using averages to hide failed samples.

Native macOS and Linux, interrupted/uncertain remote operations, changed metadata and unchanged proof reuse, conflicting heads, old v3 records and rollback all need compatibility proof. Do not bulk-rewrite old evidence or switch shared binaries from this plan.

Parallel foundation track: existing citizen identity, standing/delegation, migration/fencing and descendant lineage obligations remain in GOV-01/02/03/11 and CM-01..04. No new identity or policy engine. Independent REVIEW/RELEASE must consume both tracks.

| WP | Sprint | Repository | Outcome | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | 1 | cognitive-sdlc | Approve one bounded issue-to-delivery workflow and measurable overhead/compatibility denominator before implementation. | v0.93.3/FOUNDATION-CLOSE |
| CS4-01 | 1 | cognitive-sdlc | Map create/start, implementation/proof, review, publish/deliver and optional closeout to source-backed sequence diagrams; measure wall/internal/external-tool/network time and identify steps to remove. | WP-01 |
| CS4-02 | 1 | cognitive-sdlc | One editable issue source, one canonical installation, minimal command transitions and precise remote identity replace duplicate projections/attestations/receipts. Define bounded recovery and explicit compatibility; retain only independently justified integrity checks. | CS4-01 |
| CS4-03 | 1 | cognitive-sdlc | Prototype ordinary operations and interrupted recovery against the approved denominator; demonstrate whether each sample stays below 30 seconds internal overhead and a ten-operation path below 300 seconds, reporting cold/warm and external costs separately. | CS4-02 |
| CS4-04 | 2 | cognitive-sdlc | Read existing v3 issue/proof/review state without rewriting history; preserve exact repository/issue/head and uncertain-effect recovery. Prove rollback and a staged cutover with one canonical installed tool set; no automatic authority fallback. | CS4-03 |
| CS4-05 | 2 | cognitive-sdlc | Implement only the reviewed workflow with changed-input validation and reusable proof/review, no repeated binary hashing or duplicate builds/CI, and asynchronous optional administrative closeout. Prove supported native macOS/Linux behavior. | CS4-04 |
| CS4-06 | 2 | cognitive-sdlc | Independent installed end-to-end acceptance covers creation through delivery, metadata-only edits, unchanged proof reuse, conflicts, lost/uncertain responses, crash recovery and optional cleanup. Every ordinary sample meets the internal-overhead goals; failed or ambiguous effects are not silently replayed. | CS4-05 |
| REVIEW | 4 | cognitive-sdlc | Resolve actionable tooling correctness, compatibility/performance and installed governance/continuity findings on the exact accepted candidates, reusing unchanged proof. | CS4-06, CM-04 |
| RELEASE | 4 | cognitive-sdlc | Obtain explicit operator cutover decision, preserve recoverable v3 state, then record one canonical admitted installation and the supported migration/rollback outcome. No cutover is authorized by this plan. | REVIEW |
| GOV-01 | 3 | agent-logic-runtime | Runtime admits citizen conduct only through bound identity, Freedom Gate and signed trace; guest/operator action remains distinct. | v0.93.3/RV-08, WP-01 |
| GOV-02 | 3 | agent-logic-runtime | A Runtime policy evaluator returns rights/duties decisions with trace evidence for a bounded action. | GOV-01 |
| GOV-03 | 3 | agent-logic-runtime | Standing maintenance, restriction, restoration and revocation consume existing state primitives and retain reasons and challenge context. | GOV-02 |
| GOV-11 | 3 | agent-logic-runtime | Delegated action binds identity, standing, resource, capability and policy without exceeding the delegator grant. | GOV-03 |
| CM-01 | 3 | agent-logic-runtime | A reviewed contract distinguishes same-identity migration and recovery from new-identity descendants, replication and independent clones; it defines authority, lineage, consent, state inheritance, fencing and recovery outcomes. | v0.93.3/RV-08, GOV-01 |
| CM-02 | 3 | agent-logic-runtime | Two owned Runtime endpoints transfer one authorized citizen checkpoint and scoped continuity evidence; the source is fenced before destination activation and interruption can recover without two active holders or lost lineage. | CM-01, v0.93.2/WP-S3 |
| CM-03 | 3 | agent-logic-runtime | An authorized parent creates one distinctly identified descendant with explicit lineage, selected inherited state and newly scoped authority while preserving the parent and protecting private state. | CM-01, GOV-11, v0.93.2/WP-S5 |
| CM-04 | 3 | agent-logic-runtime | Independent installed scenarios exercise migration and descendant creation across restart, interruption and retry, verify lineage/privacy/fencing and expose the resulting identity and custody status to the operator. | CM-02, CM-03 |

## Interfaces And Contracts

Runtime foundations export accepted versioned plugin/lifecycle/state/configuration interfaces. C-SDLC exports an accepted v3-compatible workflow/cutover contract. Cognitive consumers preserve identity, consent, privacy and existing resource/governance checks.

## Validation Plan

PVF planning_contract for this setup: registry headings, JSON/YAML mirrors, source/link validity, complete allocation and acyclic qualified dependencies. Product work must later prove each acceptance and negative case through the owning installed consumer; schema, source or CI alone is not live qualification. Reuse unchanged exact-input evidence; no broad local Rust or paid qualification campaign for planning.

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
