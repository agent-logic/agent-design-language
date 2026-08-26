# Issue #308 design: WP-20 demo matrix and proof coverage

## Objective

Reconcile v0.92 demo, AEE, activation, feature-coverage, and artifact-index truth at one exact revision. The work proves consistency of existing child-owned evidence; it does not create synthetic proof, implement child features, or absorb WP-21/WP-21A cleanup.

## Entry gate

Execution is fail-closed until all four WP-20 predecessors are terminal, reconciled, and ancestral to the selected base revision:

- #256 — WP-18
- #340 — WP-18A
- #341 — WP-18B
- legacy #5839 — validated historical terminal authority for WP-19

Bootstrap and design approval do not satisfy this gate. Binding must re-read the canonical authorities and prove ancestry immediately before execution.

## Owned surfaces

- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`
- `docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md`
- `adl/tools/validate_v092_demo_proof_coverage.py`
- `adl/tools/test_v092_demo_proof_coverage.sh`
- `.csdlc/evidence/308`

All cited dependency records, sibling outputs, child proof artifacts, and external systems are read-only inputs.

## Reconciliation model

The validator treats every accepted proof row as a joined record keyed by stable feature/demo identity. Matrix, coverage, activation ledger, and AEE index must agree on owner, command, status, and exact revision. An accepted row must include positive evidence, required negative evidence, platform and credential posture, review state, and explicit non-claims.

The validator rejects missing artifacts, duplicate ownership, planned-as-passed status, synthetic proof, unsupported platform claims, and cross-surface revision disagreement. Failure produces a retained rejected-coverage report and never upgrades a claim.

## Execution sequence

1. Reverify predecessor terminal/reconciliation/ancestry truth and freeze the exact revision denominator.
2. Inventory existing demo, AEE, activation, and child proof evidence without modifying child-owned artifacts.
3. Reconcile the four owned documentation surfaces and correct WP-20 versus WP-21/WP-21A ownership.
4. Implement the fail-closed coverage validator and focused positive and negative tests.
5. Run focused validation, retain the exact-revision artifact index, and complete independent exact-head review.

## Invariants

- Missing evidence remains missing evidence.
- Planned, blocked, failed, uncovered, or synthetic rows cannot become accepted proof.
- Proof artifacts retain their producing owner and exact revision.
- WP-20 does not implement features or absorb reduction/refactoring work.
- Milestone publication, release, and quality approval remain outside this issue.

## Rollback

Restore the prior demo matrix, feature coverage, and activation ledger; remove the new artifact-index and validator changes; retain the rejected-coverage report under issue evidence. Rollback must not modify child proof artifacts or upgrade any unaccepted row.
