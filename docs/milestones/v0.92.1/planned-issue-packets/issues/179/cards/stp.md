# Structured Task Prompt

Template: 1.0.0

Issue: 179

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-16 within its exact owned paths and authority boundary.

## Deliverables

- Parity matrix, shadow reports, canary receipts, measured effect report, migration map, freeze/delta/cutover runbook, rollback criteria, stable binary installation, operator skill, selector change, post-cutover audit, and a retained regression corpus for every known v2 tooling failure and lifecycle dead end discovered before cutover.

## Acceptance

1. Normalized parity covers cards, lifecycle, validation, review, both publication linkage modes, linkage-aware finish, and cleanup with no unexplained mismatch.
2. Every imported record reports unsupported fields before mutation.
3. At least the approved canary cohort completes end to end on v3-only authority.
4. The canary cohort includes normal authoring and post-review correction for every card family, plus the issue #73 STP-denominator recovery journey; doctor must identify a valid next operation at each intermediate state.
5. Every known v2 tooling defect in the retained register has a passing v3 positive or negative regression, or a reviewed explicit non-parity decision.
6. Each migrated issue receives an archived exact v2 snapshot and a durable writer fence; the canonical v2 index is absent before v3 mutation begins.
7. Supported v2 tools and repository guards reject fenced issue mutation and any reintroduced v2 index or post-fence v2 state.
8. No issue is writable by supported v2 and v3 authorities simultaneously.
9. The final delta precedes authority switch; source archival follows cutover.
10. Cutover requires exact independent review and explicit operator approval.
11. V2 remains available only as the time-bounded read-only importer/rollback surface defined by policy.

## Dependencies

- V3-10A: issue #171
- V3-10B: issue #172
- V3-11A: issue #173
- V3-11B: issue #174
- V3-12: issue #175
- V3-13: issue #176
- V3-14: issue #177
- V3-15: issue #178

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-16
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Immediate v2 deletion, rewriting remote history, transactional remote rollback, migration without freeze/delta reconciliation, or forcing all open v2 issues to v3.
