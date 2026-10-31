# v0.93.4 Wbs

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.4. Date: 2026-10-30. Setup issue: [#1403](https://github.com/agent-logic/agent-design-language/issues/1403). Planning owner: Planning #12. Implementation owners assigned at sprint activation.

## Status

First-pass successor plan; scope allocation is not implementation or release acceptance. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized by this planning package. Create child issues/cards per sprint at activation; preserve native C-SDLC v3 authority until a separately accepted compatibility cutover.

## How To Use

Read the WBS and execution graph together; qualified milestone/ID pairs are identities. Preserve dependencies and adverse cases; reuse existing owner issues. Plan review does not establish product proof.

## WBS Summary

C-SDLC v4 simplification. 9 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Package boundaries are first-pass design proposals under the approved C-SDLC v4 scope; validate them against actual workflow measurements.

## Candidate WP Sequence

The Work Packages table is the executable allocation; each row names dependencies, owner repository and result. Existing #1352 is reused. No child issues were created in this setup.

## Work Packages

| WP | Sprint | Repository | Outcome | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | 1 | cognitive-sdlc | Approve one bounded issue-to-delivery workflow and measurable overhead/compatibility denominator before implementation. | v0.93.3/FOUNDATION-CLOSE |
| CS4-01 | 1 | cognitive-sdlc | Map create/start, implementation/proof, review, publish/deliver and optional closeout to source-backed sequence diagrams; measure wall/internal/external-tool/network time and identify steps to remove. | WP-01 |
| CS4-02 | 1 | cognitive-sdlc | One editable issue source, one canonical installation, minimal command transitions and precise remote identity replace duplicate projections/attestations/receipts. Define bounded recovery and explicit compatibility; retain only independently justified integrity checks. | CS4-01 |
| CS4-03 | 2 | cognitive-sdlc | Prototype ordinary operations and interrupted recovery against the approved denominator; demonstrate whether each sample stays below 30 seconds internal overhead and a ten-operation path below 300 seconds, reporting cold/warm and external costs separately. | CS4-02 |
| CS4-04 | 2 | cognitive-sdlc | Read existing v3 issue/proof/review state without rewriting history; preserve exact repository/issue/head and uncertain-effect recovery. Prove rollback and a staged cutover with one canonical installed tool set; no automatic authority fallback. | CS4-03 |
| CS4-05 | 3 | cognitive-sdlc | Implement only the reviewed workflow with changed-input validation and reusable proof/review, no repeated binary hashing or duplicate builds/CI, and asynchronous optional administrative closeout. Prove supported native macOS/Linux behavior. | CS4-04 |
| CS4-06 | 3 | cognitive-sdlc | Independent installed end-to-end acceptance covers creation through delivery, metadata-only edits, unchanged proof reuse, conflicts, lost/uncertain responses, crash recovery and optional cleanup. Every ordinary sample meets the internal-overhead goals; failed or ambiguous effects are not silently replayed. | CS4-05 |
| REVIEW | 3 | cognitive-sdlc | Resolve actionable correctness, compatibility and performance findings once on the final candidate; no extra attestation/receipt campaign. | CS4-06 |
| RELEASE | 3 | cognitive-sdlc | Obtain explicit operator cutover decision, preserve recoverable v3 state, then record one canonical admitted installation and the supported migration/rollback outcome. No cutover is authorized by this plan. | REVIEW |

## Sequencing

Workflow and design → Prototype and compatibility → Implementation, qualification and handoff

## Sequencing Notes

Source dependency edges are preserved. Cross-version Runtime edges are qualified v0.93.3/RV-xx. AF-04 consumes an accepted GHB baseline; integration consumes AF-08/HU-04, avoiding a baseline/affect cycle.

## Acceptance Mapping

[Per-package acceptance and negative cases](QUALITY_GATE_v0.93.4.md).

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
