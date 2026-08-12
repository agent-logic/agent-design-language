# Structured Task Prompt

Template: 1.0.0

Issue: 239

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Change only terminal-envelope/publication metadata-only head reconciliation and one focused regression reproducing PR #238 topology.

## Deliverables

- Root-aware repository-grounded terminal envelope reconciliation across cached validation and cleanup consumers
- Focused PR #238-shaped acceptance regression
- Substantive-drift, malformed-publication-revision, and metadata-only non-ancestor rejection regressions
- Post-merge cached #5835 validation

## Acceptance

1. AC-1: A terminal envelope for an accepted metadata-only PR head validates against canonical publication truth.
2. AC-2: Substantive head drift, malformed revisions, and invalid ancestry remain rejected.
3. AC-3: The focused gate_finish regression reproduces PR #238 topology and passes.
4. AC-4: After merge, cached terminal validation for #5835 passes without tracked #5835 rewrites.
5. AC-5: Fresh independent exact-head review has no actionable finding.

## Dependencies

- PR #238 merged as a4c14b4ae51ec5fbc3c3b585b217958972a3246c
- Issue #5835 is closed_by_merged_pr
- Sprint danielbaustin/agent-design-language#5854 remains open pending this fix

## Inputs

- csdlc-v2/src/finish.rs
- csdlc-v2/src/review.rs
- csdlc-v2/src/git.rs
- csdlc-v2/tests/gate_finish.rs

## Non Goals

- Rewriting #5835 lifecycle cards or derived terminal cache
- Weakening exact-head review or canonical digest validation
- Changing GitHub merge or issue closure behavior
