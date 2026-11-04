> **Current #1416 routing (2026-11-04):** Governance/continuity GOV-01/02/03/11 and CM-01..04 moved to v0.93.3 with unchanged acceptance, negative cases and resource limits. Only the nine C-SDLC v4 setup/tooling/review/release packages remain here. C-SDLC release is not a prerequisite for the current GHB audit/spec baseline. See [lossless relocation register](../v0.93.3/REALLOCATION_1416.json) and [three-sprint decision](../v0.93.3/REORGANIZATION_1416.md). Planning is not execution, spend, installed qualification or release authority.

# v0.93.4 Sprint

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.4. Date: 2026-10-30. Setup amendment: [#1405](https://github.com/agent-logic/agent-design-language/issues/1405). Planning owner: Planning #12; implementation owners assigned at sprint activation.

## Status

First-pass successor plan; not opened for implementation or release. Planning only. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized. Create child issues/cards per activated sprint; current authenticated C-SDLC remains authority until separately accepted cutover.

## How To Use

Read the WBS and execution graph together; qualified milestone/ID pairs are identities. Preserve dependencies and adverse cases; reuse existing owner issues. Plan review does not establish product proof.

## Sprint Overview

C-SDLC v4 tooling. 9 active planned work-package identities across 3 dependency streams. Governance/continuity prerequisites now belong to v0.93.3 GHB Sprint 2; no forward dependency from GHB audit/spec to this release.

## Sprint Goals

### Sprint 1: Tooling design and prototype

WP-01, CS4-01, CS4-02, CS4-03.

### Sprint 2: Tooling compatibility and native delivery

CS4-04, CS4-05, CS4-06.

### Sprint 4: Independent acceptance and handoff

REVIEW, RELEASE.

## Sprint Goal

C-SDLC v4 tooling. 9 active planned work-package identities across 3 dependency streams. Governance/continuity prerequisites now belong to v0.93.3 GHB Sprint 2; no forward dependency from GHB audit/spec to this release.

## Planned Scope

| WP | Sprint | Repository | Outcome | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | 1 | cognitive-sdlc | Approve one bounded issue-to-delivery workflow and measurable overhead/compatibility denominator before implementation. | v0.93.3/FOUNDATION-CLOSE |
| CS4-01 | 1 | cognitive-sdlc | Map create/start, implementation/proof, review, publish/deliver and optional closeout to source-backed sequence diagrams; measure wall/internal/external-tool/network time and identify steps to remove. | WP-01 |
| CS4-02 | 1 | cognitive-sdlc | One editable issue source, one canonical installation, minimal command transitions and precise remote identity replace duplicate projections/attestations/receipts. Define bounded recovery and explicit compatibility; retain only independently justified integrity checks. | CS4-01 |
| CS4-03 | 1 | cognitive-sdlc | Prototype ordinary operations and interrupted recovery against the approved denominator; demonstrate whether each sample stays below 30 seconds internal overhead and a ten-operation path below 300 seconds, reporting cold/warm and external costs separately. | CS4-02 |
| CS4-04 | 2 | cognitive-sdlc | Read existing v3 issue/proof/review state without rewriting history; preserve exact repository/issue/head and uncertain-effect recovery. Prove rollback and a staged cutover with one canonical installed tool set; no automatic authority fallback. | CS4-03 |
| CS4-05 | 2 | cognitive-sdlc | Implement only the reviewed workflow with changed-input validation and reusable proof/review, no repeated binary hashing or duplicate builds/CI, and asynchronous optional administrative closeout. Prove supported native macOS/Linux behavior. | CS4-04 |
| CS4-06 | 2 | cognitive-sdlc | Independent installed end-to-end acceptance covers creation through delivery, metadata-only edits, unchanged proof reuse, conflicts, lost/uncertain responses, crash recovery and optional cleanup. Every ordinary sample meets the internal-overhead goals; failed or ambiguous effects are not silently replayed. | CS4-05 |
| REVIEW | 4 | cognitive-sdlc | Resolve actionable tooling correctness, compatibility/performance and installed governance/continuity findings on the exact accepted candidates, reusing unchanged proof. | CS4-06, v0.93.3/CM-04 |
| RELEASE | 4 | cognitive-sdlc | Obtain explicit operator cutover decision, preserve recoverable v3 state, then record one canonical admitted installation and the supported migration/rollback outcome. No cutover is authorized by this plan. | REVIEW |

## Work Plan

### Sprint 1: Tooling design and prototype

WP-01, CS4-01, CS4-02, CS4-03.

### Sprint 2: Tooling compatibility and native delivery

CS4-04, CS4-05, CS4-06.

### Sprint 4: Independent acceptance and handoff

REVIEW, RELEASE.

## Execution Policy

One accountable owner per admitted issue; native bind/start and issue-bound goal precede implementation. Independent review precedes a PR; merge only green at the reviewed head. Preserve source and evidence; administrative reconciliation and optional cleanup may run asynchronously. Do not create a second closeout PR or repeat tests/reviews merely to update records.

## Cadence Expectations

Dependency-driven waves; no calendar promises or implied parallel staffing. Parallelize only disjoint write sets with accepted interfaces; do not duplicate active work.

## Risks / Dependencies

Accepted predecessor inputs and compatibility. Provider/experiment costs require scoped approval. Missing proof is a named gap; no retries or new machinery merely to satisfy records.

## Demo / Review Plan

[Proof matrix](DEMO_MATRIX_v0.93.4.md); PVF planning_contract for this setup: registry headings, JSON/YAML mirrors, source/link validity, complete allocation and acyclic qualified dependencies. Product work must later prove each acceptance and negative case through the owning installed consumer; schema, source or CI alone is not live qualification. Reuse unchanged exact-input evidence; no broad local Rust or paid qualification campaign for planning.

## Closeout Bar

One accountable owner per admitted issue; native bind/start and issue-bound goal precede implementation. Independent review precedes a PR; merge only green at the reviewed head. Preserve source and evidence; administrative reconciliation and optional cleanup may run asynchronously. Do not create a second closeout PR or repeat tests/reviews merely to update records.

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
