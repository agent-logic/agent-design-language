# Rustdoc And Documentation Cleanup

## Metadata

- Feature Name: Rustdoc And Documentation Cleanup
- Milestone Target: `v0.91.2`
- Status: implemented
- Planned WP Home: WP-15
- Source Docs: `.adl/docs/TBD/rust_refactoring/RUSTDOC_GAP_ANALYSIS.md`; `.adl/docs/TBD/ADL_DOC_CLEANUP_LEDGER.md`
- Proof Modes: docs, checks, review

## Purpose

Close rustdoc and documentation hygiene gaps that remain visible in local TBD
tracking. This is a repo-truth cleanup lane, not a cosmetic rewrite.

## Scope

In scope:

- Rustdoc gap remediation plan and patches.
- Doc cleanup ledger update.
- Stale milestone claim cleanup.
- Validation for changed docs.
- Top-level docs navigation and workflow-surface truth cleanup where those
  surfaces still point at stale milestone or control-plane behavior.

Out of scope:

- Broad rewrite without issue scope.
- Retiring source packets before their underlying gaps are closed.
- Unsupported implementation claims.

## Acceptance Criteria

- Rustdoc/doc claims match current code.
- No host paths or unresolved scaffold language remain in promoted docs.
- Cleanup evidence is recorded.

## Current Reconciliation

The local cleanup ledger records that `#3014` completed the bounded workflow,
navigation, and planning cleanup slice. Duplicate active-looking ledger entries
do not reopen that completed work without a new concrete finding. Historical
coverage estimates are retained context rather than current documentation
coverage, and a warning-free rustdoc render does not prove that every API has
complete semantic documentation.
