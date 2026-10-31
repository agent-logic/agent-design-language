# v0.93.4 Release Notes

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.4. Date: 2026-10-30. Setup issue: [#1403](https://github.com/agent-logic/agent-design-language/issues/1403). Planning owner: Planning #12. Implementation owners assigned at sprint activation.

## How To Use

Read the WBS and execution graph together; qualified milestone/ID pairs are identities. Preserve dependencies and adverse cases; reuse existing owner issues. Plan review does not establish product proof.

## Summary

C-SDLC v4 simplification. 9 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Package boundaries are first-pass design proposals under the approved C-SDLC v4 scope; validate them against actual workflow measurements.

## Highlights

C-SDLC v4 simplification. 9 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Package boundaries are first-pass design proposals under the approved C-SDLC v4 scope; validate them against actual workflow measurements.

## What's New In Detail

Planned scope only:

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

## Upgrade Notes

No released implementation is produced here. Later delivery must document supported versions, migration/refusal paths and known-good rollback.

## Known Limitations

First-pass successor plan; scope allocation is not implementation or release acceptance. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized by this planning package. Create child issues/cards per sprint at activation; preserve native C-SDLC v3 authority until a separately accepted compatibility cutover.

## Validation Notes

This setup uses focused planning validation; it does not claim Runtime/tooling/cognitive behavior.

PVF planning_contract for this setup: registry headings, JSON/YAML mirrors, source/link validity, complete allocation and acyclic qualified dependencies. Product work must later prove each acceptance and negative case through the owning installed consumer; schema, source or CI alone is not live qualification. Reuse unchanged exact-input evidence; no broad local Rust or paid qualification campaign for planning.

## What's Next

[Successor handoff](NEXT_MILESTONE_HANDOFF_v0.93.4.md).

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
