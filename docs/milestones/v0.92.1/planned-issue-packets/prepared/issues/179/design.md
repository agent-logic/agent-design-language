# V3-16 Design

Issue: #179

## Objective

Prove complete safety parity, execute bounded v3-only canaries, migrate authority without dual writes, and perform the separately approved selector cutover.

## Scope

Representative v2 corpus, normalized parity runner, unsupported-field register, read-only shadow, opt-in v3 issue canaries, performance/effect measurement, migration tooling, operator runbook, rollback window, installation, one operator skill, selector switch, and post-cutover audit.

## Dependencies

- V3-10A: issue #171
- V3-10B: issue #172
- V3-11A: issue #173
- V3-11B: issue #174
- V3-12: issue #175
- V3-13: issue #176
- V3-14: issue #177
- V3-15: issue #178

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Parity matrix, shadow reports, canary receipts, measured effect report, migration map, freeze/delta/cutover runbook, rollback criteria, stable binary installation, operator skill, selector change, post-cutover audit, and a retained regression corpus for every known v2 tooling failure and lifecycle dead end discovered before cutover.

## Owned Paths

- `csdlc-v3/migration/**`
- `csdlc-v3/canary/**`
- `csdlc-v3/install/**`
- `csdlc-v3/evidence/cutover/**`
- `.csdlc/issues/179/**`
- `.csdlc/prepared/issues/179/**`
- `.csdlc/prepared/issues/179/validate-outcome.rb`
- `.csdlc/evidence/179/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

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

## PVF Lanes

- `v3-16-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/179/validate-outcome.rb`.
- `v3-16-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-16-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Full offline suite, cross-platform release matrix, representative shadow corpus, known-defect regression corpus, recovered-card canary receipts, live canary receipts, exact-head CI, migration rehearsal, second-run no-op, authority scan, and post-cutover reconciliation.

## Authority Boundary

- Issue V3-16 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Immediate v2 deletion, rewriting remote history, transactional remote rollback, migration without freeze/delta reconciliation, or forcing all open v2 issues to v3.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Any unexplained parity mismatch, unsupported field, dual writer, stale review, failed canary, missing rollback evidence, or unapproved selector mutation.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-16`
